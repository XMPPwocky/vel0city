use bsp;
use bsp::cast::CastResult;
use map::Map;
use player::{
    Player,
    PlayerFlags,
    PLAYER_ONGROUND,
    PLAYER_JUMPED
};
use na;
use Game;

pub struct MoveInput {
    /// The velocity the player "wishes" to have 
    pub wishvel: na::Vec3<f32>,

    pub jump: bool,
    pub reset: bool,
}

fn simple_move(map: &Map, pl: &mut Player, dt: f32) {
    let mut dt = dt;
    let mut numcontacts = 0;
    let mut contacts: [na::Vec3<f32>; 4] = [na::zero(); 4]; 
    let mut v = pl.vel;
    for _ in 0..3 {
        if dt <= 0.0 { 
            break;
        }

        let moveray = bsp::cast::Ray {
            orig: pl.pos,
            dir: v * dt,
            halfextents: pl.halfextents
        };

        let cast = map.bsp.cast_ray(&moveray);

        if let Some(bsp::cast::CastResult { toi, norm}) = cast {
            if toi > 0.0 {
                numcontacts = 1;
                pl.pos = pl.pos + (v * toi * dt); 
                dt = dt * (1.0 - toi);
                if toi >= 1.0 {
                    break;
                }
            } else {
                numcontacts += 1;
            }
            contacts[numcontacts - 1] = norm;

            let mut bad = false;
            v = pl.vel;
            for i in 0..numcontacts {
                clip_velocity(&mut v, &contacts[i]); 
                bad = false;
                for j in (0..numcontacts).filter(|&j| j != i) {
                    if na::dot(&contacts[j], &v) < 0.0 {
                        bad = true; 
                        break;
                    }
                }
                if !bad {
                    break;
                }
            }
            if bad {
                if numcontacts == 1 {
                    // nothing
                } else if numcontacts == 2 {
                    println!("crease ");
                    let movedir = na::normalize(&v);
                    let crease = na::cross(&contacts[0], &contacts[1]);
                    v = crease * na::dot(&v, &crease);
                    v = v * (1.0 + 0.5 * na::dot(&movedir, &contacts[0]) + 0.5 * na::dot(&movedir, &contacts[1])); 
                } else {
                    // stuck in corner
                    v = na::zero();
                }
            }
            if na::dot(&v, &pl.vel) < 0.0 {
                v = na::zero(); 
            }
        } else {
            pl.pos = pl.pos + v * dt;
            break;
        }
    }
    pl.vel = v;
}

fn how_far(map: &Map, pl: &Player, movement: na::Vec3<f32>) -> (na::Vec3<f32>, na::Vec3<f32>) {
    let trace = map.bsp.cast_ray(&bsp::cast::Ray {
        orig: pl.pos,
        dir: movement,
        halfextents: pl.halfextents
    });
    if let Some(trace) = trace {
        (pl.pos.to_vec() + (movement * trace.toi), na::zero())
    } else {
        (pl.pos.to_vec() + movement, na::zero())
    }
}
fn horiz_speed(vel: &na::Vec3<f32>) -> f32 {
    na::norm(&na::Vec2::new(vel.x, vel.z))
}

pub fn move_player(game: &mut Game, playeridx: u32, input: &MoveInput, dt: f32) {
    {
        let pl = &mut game.players[playeridx as usize];
        if input.reset {
            pl.pos = na::Pnt3::new(0.0, 10.0, 0.0);
            pl.vel = na::zero();
            pl.flags = PlayerFlags::empty(); 
        };

        let accel = if pl.flags.contains(PLAYER_ONGROUND) {
            game.movesettings.accel
        } else {
            game.movesettings.airaccel
        };

        if input.jump && pl.flags.contains(PLAYER_ONGROUND) { 
            if !pl.flags.contains(PLAYER_JUMPED) {
                let jspeed = game.movesettings.jumpspeed;

                pl.vel.y = jspeed; 

                pl.flags.remove(PLAYER_ONGROUND);
            }
            //pl.flags.insert(PLAYER_JUMPED);
        }

        let friction = if pl.flags.contains(PLAYER_ONGROUND) {
            game.movesettings.friction 
        } else {
            0.0
        };

        let speedcap = if pl.flags.contains(PLAYER_ONGROUND) {
            game.movesettings.movespeed
        } else {
            game.movesettings.airspeed
        };
        
        let speed = na::norm(&pl.vel);
        if !na::approx_eq(&speed, &0.0) {
            let dir = na::normalize(&pl.vel);
            let removespeed = friction * dt * if speed < game.movesettings.speedeps {
                // Below this speed, switch from an exponential slowdown to a linear one.
                // Otherwise, the player will asymptotically approach 0 velocity, but never
                // completely stop.
                game.movesettings.speedeps
            } else {
                speed
            };

            let newspeed = na::clamp(speed - removespeed, 0.0, game.movesettings.maxspeed); 

            pl.vel = dir * newspeed;
        }

        let horizvel = na::Vec3::new(pl.vel.x, pl.vel.y, pl.vel.z);
        let wishspeed = na::clamp(na::norm(&input.wishvel), 0.0, speedcap);
        if !na::approx_eq(&wishspeed, &0.0) { 
            let movedir = na::normalize(&input.wishvel);

            let curspeed = na::dot(&horizvel, &movedir); 
            // movespeed, not speedcap, or airaccel is way too low
            let maxdelta = accel * game.movesettings.movespeed * dt;
            let addspeed = na::clamp((wishspeed - curspeed), 0.0, maxdelta);
            pl.vel = pl.vel + (movedir * addspeed);
        }

        let downray = bsp::cast::Ray {
            orig: pl.pos,
            dir: na::Vec3::new(0.0, -0.1, 0.0),
            halfextents: pl.halfextents
        };

        let cast = game.map.bsp.cast_ray(&downray);

        let hit_floor = if let Some(bsp::cast::CastResult { norm, toi, ..}) = cast {
            if toi == 0.0 && norm.y > 0.7 {
                true
            } else {
                false
            }
        } else {
            false
        };
        if hit_floor {
            pl.flags.insert(PLAYER_ONGROUND)
        } else {
            pl.flags.remove(PLAYER_ONGROUND);
            pl.vel.y -= game.movesettings.gravity * dt;
            // clamp velocity again after gravity
            let speed = na::norm(&pl.vel);
            if !na::approx_eq(&speed, &0.0) {
                let dir = na::normalize(&pl.vel);
                let newspeed = na::clamp(speed, 0.0, game.movesettings.maxspeed); 

                pl.vel = dir * newspeed;
            }
        }

        
        if !input.jump {
            pl.flags.remove(PLAYER_JUMPED);
        }

        let stepsize = 2.5;

        let startpos = pl.pos;
        let startvel = pl.vel;
        simple_move(&game.map, pl, dt);
        let downpos = pl.pos;
        let downvel = pl.vel;

        pl.pos = startpos;
        pl.vel = startvel;
        let (upstart, _) = how_far(&game.map, pl, na::Vec3::new(0.0, stepsize, 0.0));
        pl.pos = upstart.to_pnt();
        simple_move(&game.map, pl, dt);
        
        let (downstart, _) = how_far(&game.map, pl, na::Vec3::new(0.0, -stepsize , 0.0));
        pl.pos = downstart.to_pnt(); 

        let updist = horiz_speed(&(pl.pos.to_vec() - startpos.to_vec()));
        let downdist = horiz_speed(&(downpos.to_vec() - startpos.to_vec()));
        if downdist > updist { 
            pl.pos = downpos;
            pl.vel = downvel;
        }
        pl.vel.y = downvel.y;
    }
}

fn clip_middle(n: f32, eps: f32) -> f32 { 
    if na::abs(&n) < eps {
        0.0
    } else {
        n
    }
}
fn clip_velocity(vel: &mut na::Vec3<f32>, norm: &na::Vec3<f32>) {
    let d = na::dot(vel, norm);
    *vel = *vel - (*norm * d * 1.01);
    vel.x = clip_middle(vel.x, 0.5);
    vel.y = clip_middle(vel.y, 0.5);
    vel.z = clip_middle(vel.z, 0.5);
}


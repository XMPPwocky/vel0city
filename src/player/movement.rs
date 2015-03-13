use bsp;
use bsp::Plane;
use bsp::PlaneCollisionVisitor;
use bsp::cast::CastResult;
use player::{
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

        let horizvel = na::Vec3::new(pl.vel.x, 0.0, pl.vel.z);
        let wishspeed = na::clamp(na::norm(&input.wishvel), 0.0, speedcap);
        if !na::approx_eq(&wishspeed, &0.0) { 
            let movedir = na::normalize(&input.wishvel);

            let curspeed = na::dot(&horizvel, &movedir); 
            let maxdelta = accel * wishspeed * dt;
            let addspeed = na::clamp((wishspeed - curspeed), 0.0, maxdelta);
            pl.vel = pl.vel + (movedir * addspeed);
        }


        pl.vel.y -= game.movesettings.gravity * dt;

        // clamp velocity again after gravity
        let speed = na::norm(&pl.vel);
        if !na::approx_eq(&speed, &0.0) {
            let dir = na::normalize(&pl.vel);
            let newspeed = na::clamp(speed, 0.0, game.movesettings.maxspeed); 

            pl.vel = dir * newspeed;
        }

        if !input.jump {
            pl.flags.remove(PLAYER_JUMPED);
        }

        if input.jump && pl.flags.contains(PLAYER_ONGROUND) { 
            if !pl.flags.contains(PLAYER_JUMPED) {
                pl.vel.y = game.movesettings.jumpspeed;
                pl.flags.remove(PLAYER_ONGROUND);
            }
            pl.flags.insert(PLAYER_JUMPED);
        }


        let mut dt = dt;
        let mut hit_floor = false;
        let mut numcontacts = 0;
        let mut contacts: [na::Vec3<f32>; 5] = [na::zero(); 5]; 
        let mut v = pl.vel;
        for _ in 0..4 {
            if na::approx_eq(&dt, &0.0) {
                break;
            }

            let moveray = bsp::cast::Ray {
                orig: pl.pos,
                dir: v * dt,
                halfextents: pl.halfextents
            };

            let mut vis = RelevantPlanesVisitor { 
                best: None,
                vel: v,
                pos: pl.pos.to_vec(),
            };

            game.map.bsp.cast_ray_visitor(&moveray, &mut vis);

            if let Some(bsp::cast::CastResult { toi, norm }) = vis.best {
                if norm.y < -0.7 {
                    hit_floor = true;
                }

                if toi >= 0.0 {
                    numcontacts = 1;
                    pl.pos = pl.pos + (v * dt * toi);
                    dt = dt * (1.0 - toi);
                } else {
                    numcontacts += 1;
                }
                contacts[numcontacts - 1] = norm;
                v = pl.vel;
                /*
                for i in 0..numcontacts {
                    clip_velocity(&mut v, &contacts[i]); 
                    bad = false;
                    for j in (0..numcontacts).filter(|&j| j != i) {
                        if na::dot(&contacts[j], &v) > 0.0 {
                            bad = true; 
                            break;
                        }
                    }
                    if !bad {
                        break;
                    }
                }*/
                if true {
                    if numcontacts == 1 {
                        clip_velocity(&mut v, &contacts[0]);
                    } else if numcontacts == 2 {
                        let crease = na::cross(&contacts[0], &contacts[1]);
                        v = crease * na::dot(&v, &crease);
                    } else {
                        v = na::zero();
                    }
                }
            } else {
                pl.pos = pl.pos + v * dt;
                break;
            }
        }
        pl.vel = v;
        if hit_floor {
            pl.flags.insert(PLAYER_ONGROUND)
        } else {
            pl.flags.remove(PLAYER_ONGROUND)
        }
    }
}

struct RelevantPlanesVisitor {
    best: Option<CastResult>,
    vel: na::Vec3<f32>,
    pos: na::Vec3<f32>,
}
impl PlaneCollisionVisitor for RelevantPlanesVisitor {
    fn visit_plane(&mut self, plane: &Plane, castresult: &CastResult) {
        let cnorm = if na::dot(&plane.norm, &self.pos) >= plane.dist {
            plane.norm * -1.0
        } else {
            plane.norm
        };

        if !plane_matters(&self.vel, &cnorm) {
            return;
        }

        if let Some(CastResult { toi: best_toi, .. }) = self.best {
            if castresult.toi < best_toi {
                    self.best = Some(
                        CastResult {
                            norm: cnorm,
                            .. *castresult
                        });
            }
        } else {
            self.best = Some(
                CastResult {
                    norm: cnorm,
                    .. *castresult
                });
        }
    }
    fn should_visit_both(&self) -> bool { false }
}
fn plane_matters(vel: &na::Vec3<f32>, norm: &na::Vec3<f32>) -> bool {
    na::dot(vel, norm) > 0.0
}

fn clip_velocity(vel: &mut na::Vec3<f32>, norm: &na::Vec3<f32>) {
    let mut d = na::dot(vel, norm);
    *vel = *vel - (*norm * d * 1.01);
}


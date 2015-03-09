use bsp;
use bsp::Plane;
use bsp::PlaneCollisionVisitor;
use bsp::cast::CastResult;
use na;
use Game;

pub struct MoveInput {
    /// The velocity the player "wishes" to have 
    pub wishvel: na::Vec3<f32>,
}

pub fn move_player(game: &mut Game, playeridx: u32, input: &MoveInput, dt: f32) {
    let pl = &mut game.players[playeridx as usize];

    let movespeed = na::clamp(na::norm(&input.wishvel), 0.0, game.movesettings.movespeed);
    println!("movespeed {:?}", movespeed);
    if !na::approx_eq(&movespeed, &0.0) { 
        let movedir = na::normalize(&input.wishvel);

        let curspeed = na::dot(&pl.vel, &movedir); 
        let maxdelta = game.movesettings.accel * dt;
        let addspeed = na::clamp((movespeed - curspeed), -maxdelta, maxdelta);
        println!("addspeed  {:?}", addspeed);
        pl.vel = pl.vel + (movedir * addspeed);
    }

    let speed = na::norm(&pl.vel);
    if speed < game.movesettings.speedeps {
        pl.vel = na::zero();
    }
    if speed > game.movesettings.maxspeed {
        let dir = na::normalize(&pl.vel);
        pl.vel = dir * game.movesettings.maxspeed;
    }

    pl.vel.y -= game.movesettings.gravity * dt;

    let mut dt = dt;
    for _ in 0..3 {
        let moveray = bsp::cast::Ray {
            orig: pl.pos,
            dir: pl.vel * dt,
            halfextents: pl.halfextents
        };

        let mut vis = ClipMoveVisitor { 
            best: None,
            vel: pl.vel,
            curvel: pl.vel,
            pos: pl.pos.to_vec()
        };

        game.map.bsp.cast_ray_visitor(&moveray, &mut vis);

        if let Some(bsp::cast::CastResult { toi, .. }) = vis.best {
            pl.pos = pl.pos + (pl.vel * dt * toi);
            dt = dt * (1.0 - toi);
        } else {
            pl.pos = pl.pos + pl.vel * dt;
            break;
        }
        pl.vel = vis.curvel;
    }
}

struct ClipMoveVisitor {
    best: Option<CastResult>,
    vel: na::Vec3<f32>,
    curvel: na::Vec3<f32>,
    pos: na::Vec3<f32>,
}
impl PlaneCollisionVisitor for ClipMoveVisitor {
    fn visit_plane(&mut self, plane: &Plane, castresult: &CastResult) {
        let cnorm = if na::dot(&plane.norm, &self.pos) > plane.dist {
            plane.norm * -1.0
        } else {
            plane.norm
        };

        if let Some(CastResult { toi: best_toi, .. }) = self.best {
            if na::approx_eq(&castresult.toi, &best_toi) {
                clip_velocity(&mut self.curvel, &cnorm); 
            } else if castresult.toi < best_toi {
                self.curvel = self.vel;
                if clip_velocity(&mut self.curvel, &cnorm) { 
                    self.best = Some(*castresult);
                }
            }
        } else {
            self.curvel = self.vel;
            if clip_velocity(&mut self.curvel, &cnorm) { 
                self.best = Some(*castresult);
            }
        }
    }
}
fn clip_velocity(vel: &mut na::Vec3<f32>, norm: &na::Vec3<f32>) -> bool {
    let d = na::clamp(na::dot(vel, norm), 0.0, 1.0);
    if na::approx_eq(&d, &0.0) {
        false
    } else {
        *vel = *vel - (*norm * d);
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use na::{self,
        ApproxEq
    };

    #[test]
    fn movement_clipping() {
        let mut game = ::test::simple_game();
        game.players[0].pos = na::Pnt3::new(0.0, 10.0, 0.0);
        let input = MoveInput {
            wishvel: na::Vec3::new(0.0, -200.0, 0.0)
        };
        move_player(&mut game, 0, &input, 1.0);
        assert_approx_eq!(game.players[0].pos.y, ::player::PLAYER_HALFEXTENTS.y);
        assert_approx_eq!(game.players[0].vel.y, 0.0); 
    }

    #[test]
    fn gravity() {
        let mut game = ::test::simple_game();
        game.movesettings.gravity = 5.0; 
        game.players[0].pos = na::Pnt3::new(0.0, 10.0, 0.0);
        let input = MoveInput {
            wishvel: na::Vec3::new(0.0, 0.0, 0.0)
        };
        move_player(&mut game, 0, &input, 1.0);
        assert_approx_eq!(game.players[0].pos.y, 5.0); 
    }
}


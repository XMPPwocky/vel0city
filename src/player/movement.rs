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
    pl.vel = input.wishvel;
    pl.vel.y -= game.settings.gravity * dt;

    let moveray = bsp::cast::Ray {
        orig: pl.pos,
        dir: pl.vel * dt,
        halfextents: pl.halfextents
    };

    let mut vis = ClipMoveVisitor { 
        best: None,
        pos: pl.pos.to_vec(),
        vel: pl.vel,
        curvel: pl.vel
    };

    game.map.bsp.cast_ray_visitor(&moveray, &mut vis);

    if let Some(bsp::cast::CastResult { toi, .. }) = vis.best {
        pl.pos = pl.pos + (pl.vel * dt * toi);
    } else {
        pl.pos = pl.pos + pl.vel * dt;
    }
    pl.vel = vis.curvel;
}

struct ClipMoveVisitor {
    best: Option<CastResult>,
    pos: na::Vec3<f32>,
    vel: na::Vec3<f32>,
    curvel: na::Vec3<f32>,
}
impl PlaneCollisionVisitor for ClipMoveVisitor {
    fn visit_plane(&mut self, plane: &Plane, castresult: &CastResult) {
        let cnorm = plane.norm * if na::dot(&self.pos, &plane.norm) - plane.dist >= 0.0 {
            1.0
        } else {
            -1.0
        };

        if let Some(CastResult { toi: best_toi, .. }) = self.best {
            if na::approx_eq(&castresult.toi, &best_toi) {
                clip_velocity(&mut self.curvel, &cnorm); 
            } else if castresult.toi < best_toi {
                self.best = Some(*castresult);
                self.curvel = self.vel;
                clip_velocity(&mut self.curvel, &cnorm); 
            }
        } else {
            self.best = Some(*castresult);
            self.curvel = self.vel;
            clip_velocity(&mut self.curvel, &cnorm); 
        }
    }
}
fn clip_velocity(vel: &mut na::Vec3<f32>, norm: &na::Vec3<f32>) {
    *vel = *vel - (*norm * na::dot(vel, norm) * 1.01);
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
        game.settings.gravity = 5.0; 
        game.players[0].pos = na::Pnt3::new(0.0, 10.0, 0.0);
        let input = MoveInput {
            wishvel: na::Vec3::new(0.0, 0.0, 0.0)
        };
        move_player(&mut game, 0, &input, 1.0);
        assert_approx_eq!(game.players[0].pos.y, 5.0); 
    }
}


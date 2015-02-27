use bsp;
use na;
use Game;

pub struct MoveInput {
    /// The velocity the player "wishes" to have 
    pub wishvel: na::Vec3<f32>,
}

pub fn move_player(game: &mut Game, playeridx: u32, input: &MoveInput, dt: f32) {
    let pl = &mut game.players[playeridx as usize];
    pl.vel = input.wishvel;

    let moveray = bsp::cast::Ray {
        orig: pl.pos,
        dir: pl.vel * dt,
        halfextents: pl.halfextents
    };

    let cast = game.map.bsp.cast_ray(&moveray);
    if let Some(bsp::cast::CastResult { toi, norm }) = cast {
        pl.pos = pl.pos + (pl.vel * dt * toi);
        clip_velocity(&mut pl.vel, &norm);
    } else {
        pl.pos = pl.pos + pl.vel * dt;
    }
}

fn clip_velocity(vel: &mut na::Vec3<f32>, norm: &na::Vec3<f32>) {
    *vel = *vel - na::dot(vel, norm);
}

#[cfg(test)]
mod test {
    use super::*;
    use na;

    #[test]
    fn movement_clipping() {
        let mut game = ::test::simple_game();
        game.players[0].pos = na::Pnt3::new(0.0, 10.0, 0.0);
        let input = MoveInput {
            wishvel: na::Vec3::new(0.0, -20.0, 0.0)
        };
        move_player(&mut game, 0, &input, 1.0);
        assert_approx_eq!(game.players[0].pos.y, ::player::PLAYER_HALFEXTENTS.y);
    }
}


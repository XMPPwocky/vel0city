use graphics::Model;
use na;
use std::sync::Arc;

pub mod movement;

// remove this later
pub const PLAYER_HALFEXTENTS: na::Vec3<f32> = na::Vec3 { x: 0.5, y: 0.5, z: 0.5 };

pub struct Player {
    pub pos: na::Pnt3<f32>,
    pub eyeheight: f32,
    pub eyeang: na::UnitQuat<f32>,

    pub halfextents: na::Vec3<f32>,
    pub vel: na::Vec3<f32>,
}

#[cfg(test)]
pub mod test {
    use std::sync::Arc;
    use na;
    use super::PLAYER_HALFEXTENTS;
    use super::Player;

    pub fn simple_player() -> Player {
        Player {
            pos: na::Pnt3::new(0.0, 0.0, 0.0),
            eyeheight: 0.0,
            eyeang: na::zero(),

            halfextents: PLAYER_HALFEXTENTS,
            vel: na::zero(),
        }
    }
}

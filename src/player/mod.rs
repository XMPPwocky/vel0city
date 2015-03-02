use na;

pub mod movement;

// remove this later
pub const PLAYER_HALFEXTENTS: na::Vec3<f32> = na::Vec3 { x: 1.0, y: 1.0, z: 1.0 };

pub struct Player {
    pub pos: na::Pnt3<f32>,
    pub eyeheight: f32,
    pub eyeang: na::UnitQuat<f32>,

    pub halfextents: na::Vec3<f32>,
    pub vel: na::Vec3<f32>,
}

#[cfg(test)]
pub mod test {
    use na;
    use super::PLAYER_HALFEXTENTS;
    use super::Player;

    pub fn simple_player() -> Player {
        Player {
            pos: na::Pnt3::new(0.0, 0.0, 0.0),
            eyeheight: 0.0,
            eyeang: na::UnitQuat::new_with_euler_angles(0.0, 0.0, 0.0),

            halfextents: PLAYER_HALFEXTENTS,
            vel: na::zero(),
        }
    }
}

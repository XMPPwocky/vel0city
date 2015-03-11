use na;

pub mod movement;

// remove this later
pub const PLAYER_HALFEXTENTS: na::Vec3<f32> = na::Vec3 { x: 3.0, y: 6.0, z: 3.0 };

bitflags! {
    flags PlayerFlags: u32 {
        const PLAYER_ONGROUND = 0b00_00_00_01,
        const PLAYER_JUMPED = 0b00_00_00_10,
    }
}

pub struct Player {
    pub pos: na::Pnt3<f32>,
    pub eyeheight: f32,
    pub eyeang: na::UnitQuat<f32>,

    pub halfextents: na::Vec3<f32>,
    pub vel: na::Vec3<f32>,
    pub flags: PlayerFlags
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
            flags: PlayerFlags::empty()
        }
    }
}

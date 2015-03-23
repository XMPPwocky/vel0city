use na;

pub mod movement;

// remove this later
pub const PLAYER_HALFEXTENTS: na::Vec3<f32> = na::Vec3 { x: 4.0, y: 6.0, z: 4.0 };

bitflags! {
    flags PlayerFlags: u32 {
        const PLAYER_ONGROUND = 0b00_00_00_01,
        const PLAYER_HOLDING_JUMP = 0b00_00_00_10,
        const PLAYER_CAN_STEP = 0b00_00_01_00,
    }
}

pub struct Player {
    pub pos: na::Pnt3<f32>,
    pub flags: PlayerFlags,
    pub vel: na::Vec3<f32>,
    pub eyeheight: f32,
    pub halfextents: na::Vec3<f32>,
    pub eyeang: na::Vec3<f32>,
    pub landtime: f32,
    pub holdjumptime: f32,
}

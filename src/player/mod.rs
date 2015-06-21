use na;

pub mod movement;

// remove this later
pub const PLAYER_HALFEXTENTS: na::Vec3<f32> = na::Vec3 { x: 8.0, y: 12.0, z: 8.0 };

bitflags! {
    flags PlayerFlags: u32 {
        const PLAYER_ONGROUND = 0b00_00_00_01,
        const PLAYER_HOLDING_JUMP = 0b00_00_00_10,
        const PLAYER_CAN_STEP = 0b00_00_01_00,
        const PLAYER_MUST_DIE = 0b00_00_10_00,
    }
}

#[derive(Clone, Debug)]
pub struct GrappleTarget {
    pos: na::Pnt3<f32>,
    dist: f32
}

pub struct Player {
    pub pos: na::Pnt3<f32>,
    pub flags: PlayerFlags,
    pub vel: na::Vec3<f32>,
    pub eyeheight: f32,
    pub halfextents: na::Vec3<f32>,
    pub eyeang: na::UnitQuat<f32>,
    pub landtime: f32,
    pub holdjumptime: f32,
    
    pub grapple: Option<GrappleTarget>
}
impl Player {
    pub fn get_eyepos(&self) -> na::Pnt3<f32> {
        self.pos + na::Vec3 { x: 0.0, y: -self.eyeheight, z: 0.0 }
    }
}
impl Default for Player {
    fn default() -> Player {
        Player {
            pos: na::Pnt3::new(0.0, 0.0, 0.0),
            flags: PlayerFlags::empty(),
            vel: na::zero(),
            eyeheight: PLAYER_HALFEXTENTS.y * 0.8,
            halfextents: PLAYER_HALFEXTENTS,
            eyeang: na::UnitQuat::new(na::Vec3::new(0.0, 0.0, 0.0)), 
            landtime: 0.0,
            holdjumptime: 0.0,
            grapple: None
        }
    }
}

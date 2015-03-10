use glutin::VirtualKeyCode;
use std;

#[derive(Clone)]
pub struct MoveSettings {
    /// The acceleration due to gravity.
    pub gravity: f32,
    /// How fast players can accelerate
    pub accel: f32,
    /// How fast players can accelerate in midair
    pub airaccel: f32,
    /// The speed below which players will instantly stop
    pub speedeps: f32,
    /// A hard speed cap to prevent utter engine breakage.
    pub maxspeed: f32,
    /// Maximum "normal" player speed.
    pub movespeed: f32,
    
    pub airspeed: f32,
    
    pub jumpspeed: f32,

    pub friction: f32,
}
impl std::default::Default for MoveSettings {
    fn default() -> MoveSettings {
        MoveSettings {
            gravity: 40.0,
            accel: 10.0,
            airaccel: 5.0,
            speedeps: 20.0,
            maxspeed: 1000.0,
            movespeed: 30.0,
            airspeed: 10.0,
            jumpspeed: 60.0,
            friction: 2.5, 
        }
    }
}

pub struct InputSettings {
    pub sensitivity: f32,

    pub forwardkey: VirtualKeyCode,
    pub backkey: VirtualKeyCode,
    pub leftkey: VirtualKeyCode,
    pub rightkey: VirtualKeyCode,
    pub jumpkey: VirtualKeyCode,
}


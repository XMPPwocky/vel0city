use glutin::VirtualKeyCode;

#[derive(Default, Clone)]
pub struct MoveSettings {
    /// The acceleration due to gravity.
    pub gravity: f32,
    /// How fast players can accelerate
    pub accel: f32,
    /// The speed below which players will instantly stop
    pub speedeps: f32,
    /// A hard speed cap to prevent utter engine breakage.
    pub maxspeed: f32,
    /// Maximum "normal" player speed.
    pub movespeed: f32,
}

pub struct InputSettings {
    pub sensitivity: f32,

    pub forwardkey: VirtualKeyCode,
    pub backkey: VirtualKeyCode,
    pub leftkey: VirtualKeyCode,
    pub rightkey: VirtualKeyCode,
    pub jumpkey: VirtualKeyCode,
}


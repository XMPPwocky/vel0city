#[derive(Default, Clone)]
pub struct Settings {
    /// The acceleration due to gravity.
    pub gravity: f32,
    /// How fast players can accelerate
    pub accel: f32,
    /// The speed below which players will instantly stop
    pub speedeps: f32,
    /// A hard speed cap to prevent utter engine breakage.
    pub maxspeed: f32,
    /// Maximum "normal" player speed.
    pub maxmovespeed: f32,
}

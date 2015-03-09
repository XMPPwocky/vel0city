#[derive(Default, Clone)]
pub struct Settings {
    /// The acceleration due to gravity.
    pub gravity: f32,
    /// How fast players can accelerate
    pub accel: f32,
    /// The speed below which players will instantly stop
    pub speedeps: f32,
    pub maxspeed: f32,
}

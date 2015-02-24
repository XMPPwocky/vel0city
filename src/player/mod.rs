use graphics::Model;
use na;
use std::sync::Arc;

pub struct Player {
    pub pos: na::Vec3<f32>,
    pub eyeheight: f32,
    pub eyeang: na::Quat<f32>,
    pub halfextents: na::Vec3<f32>,
    pub model: Arc<Model>
}

use Model;
use na;
use std::default::Default;
use std::rc::Arc;

#[allow
pub struct Player {
    pub pos: na::Vec3<f32>,
    pub eyeheight: f32,
    pub eyeang: na::Quat<f32>,
    pub halfextents: na::Vec3<f32>,
    pub model: Arc<Model>
}

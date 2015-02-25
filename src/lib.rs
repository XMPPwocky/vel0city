extern crate "nalgebra" as na;
extern crate "rustc-serialize" as rustc_serialize; 
extern crate gfx;
extern crate glfw;

pub mod bsp;
pub mod graphics;
pub mod player;
pub mod settings;

pub struct Game {
    pub players: Vec<player::Player>,

    pub settings: settings::Settings,
}

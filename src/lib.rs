extern crate "nalgebra" as na;
//extern crate ncollide;
extern crate gfx;
extern crate glfw;

pub mod player;

pub struct Game {
    pub players: Vec<player::Player>
}

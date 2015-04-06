#![feature(core)]

#[macro_use]
extern crate nalgebra as na;
extern crate byteorder;
#[macro_use]
extern crate glium;
extern crate glutin;
extern crate image;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate rustc_serialize;

extern crate vel0city_map;
extern crate vel0city_base;
extern crate vel0city_graphics;

pub use vel0city_base::assets as assets;
pub use vel0city_map as map;
pub use vel0city_graphics as graphics;

pub mod input;
pub mod player;
pub mod particle;
pub mod settings;

pub struct Game {
    pub map: map::Map,
    pub players: Vec<player::Player>,

    pub movesettings: settings::MoveSettings,
    pub timescale: f32,
    pub time: f32,
}

#[cfg(test)]
pub mod test {
    use super::{map, Game, player};

}

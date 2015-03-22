#![feature(core, collections, old_io, io, path)]

#[macro_use]
extern crate "nalgebra" as na;
extern crate byteorder;
#[macro_use]
extern crate glium;
extern crate glutin;
extern crate image;
extern crate "rustc-serialize" as rustc_serialize;
#[macro_use]
extern crate bitflags;
extern crate wavefront_obj;

pub mod assets;
pub mod bsp;
pub mod graphics;
pub mod input;
pub mod map;
pub mod player;
pub mod settings;
pub mod qbsp_import;

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

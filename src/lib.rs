#![feature(io, fs, path)]

#[macro_use]
extern crate "nalgebra" as na;
extern crate byteorder;
#[macro_use]
extern crate glium;
extern crate glutin;
extern crate image;
extern crate time;
extern crate "rustc-serialize" as rustc_serialize;
extern crate wavefront_obj;

pub mod assets;
pub mod bsp;
pub mod graphics;
pub mod map;
pub mod player;
pub mod settings;
pub mod qbsp_import;

pub struct Game {
    pub map: map::Map,
    pub players: Vec<player::Player>,

    pub settings: settings::Settings,
}

#[cfg(test)]
pub mod test {
    use super::{map, Game, player};

    pub fn simple_game() -> Game {
        Game {
            map: map::test::single_plane_map(),
            players: vec![player::test::simple_player()],
            settings: ::std::default::Default::default()
        }
    }
}

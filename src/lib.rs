extern crate "nalgebra" as na;
#[macro_use]
extern crate glium;
extern crate glutin;
extern crate time;
extern crate "rustc-serialize" as rustc_serialize;
extern crate wavefront_obj;

macro_rules! assert_approx_eq {
    ($a: expr, $b: expr) => {
        if na::approx_eq(&$a, &$b) {
            ()
        } else {
            panic!("{:?} != {:?}", $a, $b);
        }
    }
}

pub mod bsp;
pub mod graphics;
pub mod map;
pub mod player;
pub mod settings;

pub struct Game {
    pub map: map::Map,
    pub players: Vec<player::Player>,

    pub settings: settings::Settings,
}

#[cfg(test)]
pub mod test {
    use super::{map, Game, player, settings};
    use glium;

    pub fn simple_game(display: &glium::Display) -> Game {
        Game {
            map: map::test::single_plane_map(display),
            players: vec![player::test::simple_player()],
            settings: ::std::default::Default::default()
        }
    }
}

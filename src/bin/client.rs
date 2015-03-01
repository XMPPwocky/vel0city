#![feature(io, fs)]

extern crate glium;
extern crate glutin;
extern crate vel0city;
extern crate wavefront_obj;

use std::io::Read;

pub struct Client {
    playermodel: vel0city::graphics::Model
}
impl Client {
    fn new(display: &glium::Display) -> Client {
        let mut playerobjfile = std::fs::File::open("assets/player.obj").unwrap();
        let mut s = String::new();
        playerobjfile.read_to_string(&mut s).unwrap();
        let playerobj = &wavefront_obj::obj::parse(s).unwrap().objects[0];

        let playermodel = vel0city::graphics::wavefront::obj_to_model(playerobj,
                                                                      display);

        Client {
            playermodel: playermodel,
        }
    }
}

fn main() {
}

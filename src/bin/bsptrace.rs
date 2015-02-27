#![feature(io, fs)]
use vel0city::bsp;
use std::io::Read;
use rustc_serialize::json;

extern crate vel0city;
extern crate "nalgebra" as na;
extern crate "rustc-serialize" as rustc_serialize;

fn main() { 
    let mut f = std::fs::File::open("testbsp.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let tree: bsp::Tree = json::decode(&s).unwrap();

    for z in (0..80) {
        for x in (0..80) {
            let ray = bsp::cast::Ray {
                orig: na::Pnt3::new(
                          (x - 40) as f32 / 10.0,
                          10.0,
                          (z - 40) as f32 / 10.0,
                          ),
                dir: na::Vec3::new(0.0, -9.5, 0.0),
                halfextents: na::Vec3::new(0.0, 0.0, 0.5)
            };

            if let Some(c) = tree.cast_ray(&ray) {
                if c.toi <= 1.0 {
                    print!("X");
                } else {
                    print!("x");
                }
            } else {
                print!(".");
            }

        }
        println!("");
    }
}

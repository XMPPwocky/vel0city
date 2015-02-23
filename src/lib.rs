#![feature(io, plugin)]
#![plugin(ecs)]

pub use ::base::*;

#[macro_use]
extern crate ecs;
extern crate "nalgebra" as na;
extern crate gfx;
extern crate glfw;


pub mod graphics;

/// Components that don't really fit anywhere else.
pub mod base {
    component! {
        Position {
            x: f32,
            y: f32,
            z: f32
        }
    }
}

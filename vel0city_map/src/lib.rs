#![feature(core)]

#[macro_use]
extern crate glium;
extern crate vel0city_base;
extern crate nalgebra as na;
extern crate byteorder;
extern crate image;

pub mod bsp;
pub mod q3_import;

pub struct Map {
    pub bsp: bsp::Tree,
}

#[derive(Debug)]
pub struct MapFace {
    pub texture: i32,
    pub lightmap: i32,
    pub index_start: u32,
    pub index_count: u32,
}

#[derive(Copy, Clone, Debug)]
pub struct MapVertex {
    pub position: [f32; 3],
    pub texcoords: [f32; 2],
    pub lightmaptexcoords: [f32; 2],
    pub normal: [f32; 3]
}
implement_vertex!(MapVertex, position, texcoords, lightmaptexcoords, normal);

pub struct GraphicsMap {
    pub vertices: glium::VertexBuffer<MapVertex>,
    pub indices: glium::IndexBuffer,
    pub faces: Vec<MapFace>, 
    pub textures: Vec<glium::Texture2d>,
    pub lightmaps: Vec<glium::Texture2d>,
    pub shaders: Vec<glium::Program>,
}


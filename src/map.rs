use bsp;
use glium;
use graphics;
use graphics::Vertex;

pub struct MapModel {
    pub vertices: glium::VertexBuffer<graphics::Vertex>,
    pub textures: Vec<glium::Texture2d>,
    pub shaders: Vec<glium::Program>,
}

pub struct Map {
    pub bsp: bsp::Tree,
}

pub fn single_plane_map() -> Map {
    Map {
        bsp: ::bsp::test_tree(),
    }
}

#[cfg(test)]
pub mod test {
    pub use super::single_plane_map;
}


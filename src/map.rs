use bsp;
use glium;
use na;

pub struct LeafModel {
    pub firstvirt: u32,
    pub lastvirt: u32,
    pub texture: u32,
    pub shader: u32,
}

pub struct Map {
    pub bsp: bsp::Tree,
    pub vertices: glium::VertexBuffer<graphics::Vertex>,
    pub textures: Vec<glium::Texture2D>,
    pub shaders: Vec<glium::Program>,
    pub leafmodels: Vec<LeafModel>, 
}

pub fn single_plane_map(display: &glium::Display) -> Map {
    Map {
        bsp: ::bsp::test_tree(),
        vertices: glium::VertexBuffer::new(display, vec![]),
        textures: vec![],
        shaders: vec![],
        leafmodels: vec![]
    }
}

#[cfg(test)]
pub mod test {
    pub use super::single_plane_map;
}


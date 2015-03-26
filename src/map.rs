use bsp;
use glium;

pub struct Map {
    pub bsp: bsp::Tree,
}

pub fn single_plane_map() -> Map {
    Map {
        bsp: ::bsp::test_tree(),
    }
}

#[derive(Debug)]
pub struct MapFace {
    pub texture: i32,
    pub lightmap: i32,
    pub index_start: u32,
    pub index_count: u32,
}

#[derive(Copy, Debug)]
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

#[cfg(test)]
pub mod test {
    pub use super::single_plane_map;
}


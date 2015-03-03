use assets;
use bsp;
use glium;
use graphics;
use graphics::Vertex;

pub struct LeafModel {
    pub firstvirt: u32,
    pub lastvirt: u32,
    pub texture: u32,
    pub shader: u32,
}

pub struct MapModel {
    pub vertices: glium::VertexBuffer<graphics::Vertex>,
    pub textures: Vec<glium::Texture2d>,
    pub shaders: Vec<glium::Program>,
    pub leafmodels: Vec<LeafModel>, 
}

pub struct Map {
    pub bsp: bsp::Tree,
}

pub fn single_plane_map() -> Map {
/*    let tex = vec![
        vec![(0u8, 0u8, 0u8), (0u8, 255u8, 0u8)],
        vec![(0u8, 0u8, 255u8), (0u8, 255u8, 255u8)]
    ];
    let tex = glium::Texture2d::new(display, tex);
    let program = glium::Program::from_source(
        &display,
        &assets::load_str_asset("vertex.glsl").unwrap(),
        &assets::load_str_asset("fragment.glsl").unwrap(),
        None
        ).unwrap();

    let verts = vec![
        Vertex {
            position: [-4096.0, 0.0, 4096.0],
            texcoords: [0.0, 0.0]
        },
        Vertex {
            position: [4096.0, 0.0, 4096.0],
            texcoords: [1.0, 0.0]
        },
        Vertex {
            position: [0.0, 0.0, -4096.0],
            texcoords: [0.5, 1.0]
        }
    ];*/

    Map {
        bsp: ::bsp::test_tree(),
    }
}

#[cfg(test)]
pub mod test {
    pub use super::single_plane_map;
}


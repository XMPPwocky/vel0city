use Game;
use glium;
use glium::Surface;
use na::{
    self,
    ToHomogeneous
};
use std::sync::Arc;

pub mod wavefront;

#[derive(Copy)]
pub struct Vertex {
    pub position: [f32; 3], 
    pub texcoords: [f32; 2] 
}
implement_vertex!(Vertex, position, texcoords);

pub struct Model {
    mesh: glium::VertexBuffer<Vertex>,
    indices: glium::IndexBuffer, 
    program: Arc<glium::Program>, 
    texture: glium::Texture2d,
}

/// Hard to describe, but you'll know it if you see it.
pub struct View {
    pub w2s: na::Mat4<f32>,
    pub drawparams: glium::DrawParameters,
}

pub fn draw_view(game: &Game,
                 view: &View,
                 playermodel: &Model,
                 frame: &mut glium::Frame) { 
    
    /*{
        let uniforms = uniform! { 
            transform: *(view.w2s).as_array(),
            color: &game.map.textures[0]
        };

        frame.draw(&game.map.vertices,
                   &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                   &game.map.shaders[0],
                   &uniforms,
                   &view.drawparams).unwrap()
    }*/

     
    for player in &game.players {
        let m2w = na::Iso3 {
            translation: player.pos.to_vec(),
            rotation: player.eyeang.to_rot(),
        }.to_homogeneous();

        let uniforms = uniform! { 
            transform: *(view.w2s * m2w).as_array(),
            color: &playermodel.texture
        };

        frame.draw(&playermodel.mesh,
                   &playermodel.indices,
                   &playermodel.program,
                   &uniforms,
                   &view.drawparams).unwrap();
    }
}

pub fn stub_display() -> glium::Display {
    use glutin;
    use glium::DisplayBuild;

    glutin::HeadlessRendererBuilder::new(640, 480).build_glium().unwrap()
}

use Game;
use glium;
use glium::Surface;
use na::{
    self,
    ToHomogeneous
};

pub mod wavefront;

#[derive(Copy)]
pub struct Vertex {
    position: [f32; 3], 
    texcoords: [f32; 2] 
}
implement_vertex!(Vertex, position, texcoords);

pub struct Model {
    mesh: glium::VertexBuffer<Vertex>,
    indices: glium::IndexBuffer, 
    program: glium::Program, 
    texture: glium::Texture2d,
}

/// Hard to describe, but you'll know it if you see it.
pub struct View {
    w2s: na::Mat4<f32>,
    drawparams: glium::DrawParameters,
}

pub fn draw_view(game: &Game,
                 view: &View,
                 playermodel: &Model,
                 frame: &mut glium::Frame) { 
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

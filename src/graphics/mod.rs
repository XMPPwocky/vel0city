use Game;
use glium;
use glium::Surface;
use na;
use std::sync::Arc;

pub mod wavefront;
pub mod hud;

#[derive(Copy)]
pub struct Vertex {
    pub position: [f32; 3], 
    pub texcoords: [f32; 2] 
}
implement_vertex!(Vertex, position, texcoords);

pub struct Model {
    pub mesh: glium::VertexBuffer<Vertex>,
    pub indices: glium::IndexBuffer, 
    pub program: Arc<glium::Program>, 
    pub texture: glium::Texture2d,
}

/// Hard to describe, but you'll know it if you see it.
pub struct View {
    pub w2s: na::Mat4<f32>,
    pub drawparams: glium::DrawParameters,
}

pub fn draw_view(game: &Game,
                 view: &View,
                 playermodel: &Model,
                 mapmodels: &[Model],
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


    for mapmodel in mapmodels {
        let samp = glium::uniforms::Sampler::new(&mapmodel.texture)
            .wrap_function(glium::uniforms::SamplerWrapFunction::Repeat)
            .anisotropy(4)
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear)
            .minify_filter(glium::uniforms::MinifySamplerFilter::LinearMipmapLinear);

        let uniforms = uniform! { 
            transform: *(view.w2s).as_array(),
            color: samp
        };
        frame.draw(&mapmodel.mesh,
                   &mapmodel.indices,
                   &mapmodel.program,
                   &uniforms,
                   &view.drawparams).unwrap();
    }
     
    /*
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
    }*/
}

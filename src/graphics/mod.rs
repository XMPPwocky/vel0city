use glium;
use glium::Surface;
use map::GraphicsMap;
use na;
use std::sync::Arc;
use std::default::Default;

pub mod wavefront;
pub mod hud;
pub mod passes;

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

pub struct View {
    pub w2s: na::Mat4<f32>,
    pub cam: na::Mat4<f32>,
}

pub struct Light {
    pub position: na::Vec3<f32>,
    pub color: na::Vec3<f32>,
    pub intensity: f32,
    pub radius: f32, 
}

pub struct Scene {
    pub map: GraphicsMap,
    pub lights: Vec<Light>,
}

pub fn draw_scene<S: glium::Surface>(surface: &mut S,
                                     scene: &Scene,
                                     view: &View) {
    draw_map(surface, &scene.map, view);
}

fn draw_map<S: glium::Surface>(surface: &mut S, map: &GraphicsMap, view: &View) {
    let drawparams_main = glium::DrawParameters {
        depth_test: glium::DepthTest::IfLess,
        depth_write: true,
        backface_culling: glium::BackfaceCullingMode::CullCounterClockWise,
        ..Default::default()
    };

    for face in &map.faces {
        let color = &map.textures[face.texture as usize];
        let colorsamp = glium::uniforms::Sampler::new(color)
            .anisotropy(16)
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
            .minify_filter(glium::uniforms::MinifySamplerFilter::LinearMipmapLinear);

        if face.lightmap >= 0 {
            let lightmap = &map.lightmaps[face.lightmap as usize];
            let lmsamp = glium::uniforms::Sampler::new(lightmap)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);

            let uniforms = uniform! { 
                w2s: *(view.w2s).as_array(),
                cam: *(view.cam).as_array(),
                model: *na::new_identity::<na::Mat4<_>>(4).as_array(), 
                diffuse: colorsamp,
                lightmap: lmsamp 
            };
            surface.draw(&map.vertices,
                       &map.indices.slice(face.index_start as usize, face.index_count as usize).unwrap(),
                       &map.shaders[0],
                       &uniforms,
                       &drawparams_main).unwrap();
        } else {
    println!("Skipping un-lightmapped face...");
        }
    }
}

#[derive(Copy)]
pub struct QuadVertex {
    position: [f32; 2]
}
implement_vertex!(QuadVertex, position);


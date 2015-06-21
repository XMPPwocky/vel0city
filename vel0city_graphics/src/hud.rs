use vel0city_base::assets;
use glium;
use QuadVertex;
use na;
use na::{
    ToHomogeneous
};
use glium::index::PrimitiveType::TriangleStrip;
use std::default::Default;

pub struct HudManager {
    quad_verts: glium::VertexBuffer<QuadVertex>,
    quad_indices: glium::IndexBuffer<u8>,
    quad_shader: glium::Program,
}
impl HudManager {
    pub fn new(d: &glium::Display) -> HudManager {
        let verts = vec![
            QuadVertex { position: [-1.0, -1.0] },
            QuadVertex { position: [1.0, -1.0] },
            QuadVertex { position: [-1.0, 1.0] },
            QuadVertex { position: [1.0, 1.0] },
        ];
        
        let program = glium::Program::from_source(
            d,
            &assets::load_str_asset("shaders/hud_vertex.glsl").unwrap(),
            &assets::load_str_asset("shaders/hud_fragment.glsl").unwrap(),
            None
            ).unwrap();


        HudManager {
            quad_verts: glium::VertexBuffer::new(d, verts),
            quad_indices: glium::index::IndexBuffer::new(d, TriangleStrip, vec![0u8, 1, 2, 3]),
            quad_shader: program,
        }
    }

    pub fn draw_elements<S>(&self,
                            target: &mut S,
                            context: &Context,
                            elements: &[Element]
                           ) where S: glium::Surface {
        for element in elements {
            match element.element_type {
                ElementType::TransformedBlit { ref texture, f } => {
                    if let Some(customtransform) = f(context) {
                        let samp = glium::uniforms::Sampler::new(texture)
                            .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                            .minify_filter(glium::uniforms::MinifySamplerFilter::LinearMipmapLinear)
                            .anisotropy(8);

                        let uniforms = uniform! { 
                            transform: *(customtransform * element.transform.to_homogeneous().to_homogeneous()).as_array(),
                            color: samp
                        };
                        let drawparams = glium::DrawParameters {
                            blending_function: Some(glium::BlendingFunction::Addition {
                                source: glium::LinearBlendingFactor::SourceAlpha,
                                destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                            }),
                            ..Default::default()
                        };
                        target.draw(&self.quad_verts,
                                    &self.quad_indices,
                                    &self.quad_shader,
                                    &uniforms,
                                    &drawparams).unwrap()
                    }
                }
            }
        }
    }
}


pub struct Context {
    pub eyeang: na::UnitQuat<f32>,
    pub player_vel: na::Vec3<f32>,
}

pub struct Element {
    pub transform: na::Iso2<f32>, 
    pub element_type: ElementType
}

pub enum ElementType {
    TransformedBlit {
        texture: glium::Texture2d,
        f: fn(&Context) -> Option<na::Mat4<f32>>,
    }
}

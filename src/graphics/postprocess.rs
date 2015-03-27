use graphics::QuadVertex;
use glium;

pub struct Technique {
    shader: glium::Program,
    drawparams: glium::DrawParameters,
}
impl Technique {
    pub fn new(shader: glium::Program) -> Technique {
        Technique { shader: shader, drawparams: ::std::default::Default::default() }
    }
}

pub struct PostprocessInputs {
    pub color: glium::Texture2d, 
    pub normal: glium::Texture2d,
    pub position: glium::Texture2d,
    pub depth: glium::texture::DepthTexture2d,
}
impl PostprocessInputs {
    pub fn new(d: &glium::Display, dimensions: (u32, u32)) -> PostprocessInputs {
        let color = glium::texture::Texture2d::new_empty(&d, glium::texture::UncompressedFloatFormat::F32F32F32F32, dimensions.0, dimensions.1); 
        let normal = glium::texture::Texture2d::new_empty(&d, glium::texture::UncompressedFloatFormat::F32F32F32F32, dimensions.0, dimensions.1); 
        let position = glium::texture::Texture2d::new_empty(&d, glium::texture::UncompressedFloatFormat::F32F32F32F32, dimensions.0, dimensions.1); 
        let depth = glium::texture::DepthTexture2d::new_empty(&d, glium::texture::DepthFormat::F32, dimensions.0, dimensions.1); 

        PostprocessInputs {
            color: color,
            normal: normal,
            position: position,
            depth: depth
        }
    }
}
pub struct PostprocessSystem {
    quad_verts: glium::VertexBuffer<QuadVertex>,
    quad_indices: glium::IndexBuffer,
}
impl PostprocessSystem {
    pub fn new(d: &glium::Display) -> PostprocessSystem {
        let verts = vec![
            QuadVertex { position: [-1.0, -1.0] },
            QuadVertex { position: [1.0, -1.0] },
            QuadVertex { position: [-1.0, 1.0] },
            QuadVertex { position: [1.0, 1.0] },
        ];
        let indices = glium::index::TriangleStrip(vec![0u8, 1, 2, 3]);

        PostprocessSystem {
            quad_verts: glium::VertexBuffer::new(d, verts),
            quad_indices: glium::IndexBuffer::new(d, indices),
        }
    }

    pub fn postprocess<S>(&self,
                       input: &PostprocessInputs, 
                       output: &mut S, 
                       technique: &Technique) where S: glium::Surface {
        let uniforms = uniform! {
            color_texture: &input.color,
            normal_texture: &input.normal,
            position_texture: &input.position,
            depth_texture: &input.depth
        };

        output.draw(&self.quad_verts,
                    &self.quad_indices,
                    &technique.shader,
                    &uniforms,
                    &technique.drawparams).unwrap();
    }
}


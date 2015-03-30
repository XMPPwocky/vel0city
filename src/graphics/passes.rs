use graphics::QuadVertex;
use graphics::Light;
use glium;
use glium::Surface;

pub struct Technique {
    pub shader: glium::Program,
    pub drawparams: glium::DrawParameters,
}
impl Technique {
    pub fn new(shader: glium::Program) -> Technique {
        Technique { shader: shader, drawparams: ::std::default::Default::default() }
    }
}

pub struct PassData {
    pub color: glium::Texture2d, 
    pub light: glium::Texture2d, 
    pub normal: glium::Texture2d,
    pub position: glium::Texture2d,
    pub depth: glium::texture::DepthTexture2d,
}

impl PassData {
    pub fn new(d: &glium::Display, dimensions: (u32, u32)) -> PassData {
        let color = glium::texture::Texture2d::new_empty(d, glium::texture::UncompressedFloatFormat::F32F32F32F32, dimensions.0, dimensions.1); 
        let light = glium::texture::Texture2d::new_empty(d, glium::texture::UncompressedFloatFormat::F32F32F32F32, dimensions.0, dimensions.1); 
        let normal = glium::texture::Texture2d::new_empty(d, glium::texture::UncompressedFloatFormat::F32F32F32F32, dimensions.0, dimensions.1); 
        let position = glium::texture::Texture2d::new_empty(d, glium::texture::UncompressedFloatFormat::F32F32F32F32, dimensions.0, dimensions.1); 
        let depth = glium::texture::DepthTexture2d::new_empty(d, glium::texture::DepthFormat::F32, dimensions.0, dimensions.1); 

        PassData {
            color: color,
            light: light,
            normal: normal,
            position: position,
            depth: depth
        }
    }
    pub fn get_framebuffer(&self, display: &glium::Display) -> glium::framebuffer::MultiOutputFrameBuffer {
        let fboutputs = [
            ("color_out", &self.color),
            ("light_out", &self.light),
            ("normal_out", &self.normal),
            ("position_out", &self.position),
        ];
        glium::framebuffer::MultiOutputFrameBuffer::with_depth_buffer(display, &fboutputs, &self.depth)
    }
}

pub struct PostprocessPassData {
    pub color: glium::Texture2d, 
}
impl PostprocessPassData {
    pub fn new(d: &glium::Display, dimensions: (u32, u32)) -> PostprocessPassData {
        let color = glium::texture::Texture2d::new_empty(d, glium::texture::UncompressedFloatFormat::F32F32F32F32, dimensions.0, dimensions.1); 

        PostprocessPassData {
            color: color,
        }
    }
    pub fn get_framebuffer(&self, display: &glium::Display) -> glium::framebuffer::SimpleFrameBuffer {
        glium::framebuffer::SimpleFrameBuffer::new(display, &self.color)
    }
}
pub struct LightPassData {
    pub light: glium::Texture2d, 
}
impl LightPassData {
    pub fn new(d: &glium::Display, dimensions: (u32, u32)) -> LightPassData {
        let light = glium::texture::Texture2d::new_empty(d, glium::texture::UncompressedFloatFormat::F32F32F32F32, dimensions.0, dimensions.1); 

        LightPassData {
            light: light,
        }
    }
    pub fn get_framebuffer(&self, display: &glium::Display) -> glium::framebuffer::SimpleFrameBuffer {
        glium::framebuffer::SimpleFrameBuffer::new(display, &self.light)
    }
}
pub struct PassSystem {
    quad_verts: glium::VertexBuffer<QuadVertex>,
    quad_indices: glium::IndexBuffer,
}
impl PassSystem {
    pub fn new(d: &glium::Display) -> PassSystem {
        let verts = vec![
            QuadVertex { position: [-1.0, -1.0] },
            QuadVertex { position: [1.0, -1.0] },
            QuadVertex { position: [-1.0, 1.0] },
            QuadVertex { position: [1.0, 1.0] },
        ];
        let indices = glium::index::TriangleStrip(vec![0u8, 1, 2, 3]);

        PassSystem {
            quad_verts: glium::VertexBuffer::new(d, verts),
            quad_indices: glium::IndexBuffer::new(d, indices),
        }
    }

    pub fn postprocess<S>(&self,
                       display: &glium::Display,
                       input_prepass: &PassData, 
                       input_light: &LightPassData, 
                       output: &mut S, 
                       technique: &Technique) where S: glium::Surface { 
        let colorsamp = glium::uniforms::Sampler::new(&input_prepass.color)
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);
        let normsamp = glium::uniforms::Sampler::new(&input_prepass.normal)
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);
        let possamp = glium::uniforms::Sampler::new(&input_prepass.position)
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);
        let depthsamp = glium::uniforms::Sampler::new(&input_prepass.depth)
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);
        let lightsamp = glium::uniforms::Sampler::new(&input_light.light)
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);
        let uniforms = uniform! {
            color_texture: colorsamp,
            normal_texture: normsamp,
            position_texture: possamp, 
            depth_texture: depthsamp,
            light_texture: lightsamp
        };

        output.draw(&self.quad_verts,
                         &self.quad_indices,
                         &technique.shader,
                         &uniforms,
                         &technique.drawparams).unwrap();
    }

    pub fn light_passes(&self,
                        display: &glium::Display,
                       input: &PassData, 
                       output: &mut LightPassData, 
                       lights: &[Light],
                       technique: &Technique) { 
        let mut framebuffer = output.get_framebuffer(display);
        for light in lights {
            let colorsamp = glium::uniforms::Sampler::new(&input.color)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);
            let normsamp = glium::uniforms::Sampler::new(&input.normal)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);
            let possamp = glium::uniforms::Sampler::new(&input.position)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);
            let depthsamp = glium::uniforms::Sampler::new(&input.depth)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);
            let lightsamp = glium::uniforms::Sampler::new(&input.light)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);

            let uniforms = uniform! {
                color_texture: colorsamp,
                normal_texture: normsamp,
                position_texture: possamp, 
                depth_texture: depthsamp,
                light_texture: lightsamp,

                light_position: *light.position.as_array(),
                light_intensity: light.intensity,
                light_attenuation: light.attenuation,
                light_color: *light.color.as_array()
            };

            framebuffer.draw(&self.quad_verts,
                        &self.quad_indices,
                        &technique.shader,
                        &uniforms,
                        &technique.drawparams).unwrap();
        }
    }
}


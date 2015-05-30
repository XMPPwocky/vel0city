use QuadVertex;
use Light;
use glium;
use glium::Surface;
use View;
use na;

pub struct Technique {
    pub shader: glium::Program,
    pub drawparams: glium::DrawParameters<'static>,
}
impl Technique {
    pub fn new(shader: glium::Program) -> Technique {
        Technique { shader: shader, drawparams: ::std::default::Default::default() }
    }
}

pub struct PassData {
    pub diffuse: glium::Texture2d, 
    pub light: glium::Texture2d, 
    pub normal: glium::Texture2d,
    pub position: glium::Texture2d,
    pub depth: glium::texture::DepthTexture2d,
}

impl PassData {
    pub fn new(d: &glium::Display, dimensions: (u32, u32)) -> PassData {
        let diffuse = glium::texture::Texture2d::new_empty(d, glium::texture::UncompressedFloatFormat::F32F32F32F32, dimensions.0, dimensions.1); 
        let light = glium::texture::Texture2d::new_empty(d, glium::texture::UncompressedFloatFormat::F32F32F32F32, dimensions.0, dimensions.1); 
        let normal = glium::texture::Texture2d::new_empty(d, glium::texture::UncompressedFloatFormat::F32F32F32F32, dimensions.0, dimensions.1); 
        let position = glium::texture::Texture2d::new_empty(d, glium::texture::UncompressedFloatFormat::F32F32F32F32, dimensions.0, dimensions.1); 
        let depth = glium::texture::DepthTexture2d::new_empty(d, glium::texture::DepthFormat::F32, dimensions.0, dimensions.1); 

        PassData {
            diffuse: diffuse,
            light: light,
            normal: normal,
            position: position,
            depth: depth
        }
    }
    pub fn get_framebuffer_for_prepass(&self, display: &glium::Display) -> glium::framebuffer::MultiOutputFrameBuffer {
        let fboutputs = [
            ("diffuse_out", &self.diffuse),
            ("light_out", &self.light),
            ("normal_out", &self.normal),
            ("position_out", &self.position),
        ];
        glium::framebuffer::MultiOutputFrameBuffer::with_depth_buffer(display, &fboutputs, &self.depth)
    }
    pub fn get_framebuffer_for_lightpass(&self, display: &glium::Display) -> glium::framebuffer::MultiOutputFrameBuffer {
        let fboutputs = [
            ("light_out", &self.light),
        ];
        glium::framebuffer::MultiOutputFrameBuffer::with_depth_buffer(display, &fboutputs, &self.depth)
    }
}

pub struct PassSystem {
    quad_verts: glium::VertexBuffer<QuadVertex>,
}
impl PassSystem {
    pub fn new(d: &glium::Display) -> PassSystem {
        let verts = vec![
            QuadVertex { position: [-1.0, -1.0] },
            QuadVertex { position: [1.0, -1.0] },
            QuadVertex { position: [-1.0, 1.0] },
            QuadVertex { position: [1.0, 1.0] },
        ];

        PassSystem {
            quad_verts: glium::VertexBuffer::new(d, verts),
        }
    }

    pub fn postprocess<S>(&self,
                       input: &PassData, 
                       output: &mut S, 
                       technique: &Technique) where S: glium::Surface { 
        let diffusesamp = glium::uniforms::Sampler::new(&input.diffuse)
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
            diffuse_texture: diffusesamp,
            normal_texture: normsamp,
            position_texture: possamp, 
            depth_texture: depthsamp,
            light_texture: lightsamp
        };

        output.draw(&self.quad_verts,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                         &technique.shader,
                         &uniforms,
                         &technique.drawparams).unwrap();
    }

    pub fn light_passes(&self,
                        display: &glium::Display,
                       data: &mut PassData, 
                       lights: &[Light],
                       view: &View,
                       technique: &Technique) { 
        let mut framebuffer = data.get_framebuffer_for_lightpass(display);
        for light in lights {
            let diffusesamp = glium::uniforms::Sampler::new(&data.diffuse)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);
            let normsamp = glium::uniforms::Sampler::new(&data.normal)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);
            let possamp = glium::uniforms::Sampler::new(&data.position)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear);

            let uniforms = uniform! {
                diffuse_texture: diffusesamp,
                normal_texture: normsamp,
                position_texture: possamp, 

                cam_inv: *na::inv(&view.cam).unwrap().as_array(),

                light_position: *light.position.as_array(),
                light_intensity: light.intensity,
                light_radius: light.radius,
                light_color: *light.color.as_array(),
                light_cutoff: 0.0003,
                light_max_distance: light.radius * ((light.intensity / 0.0003).sqrt() - 1.0)
            };

            framebuffer.draw(&self.quad_verts,
                             &glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                             &technique.shader,
                             &uniforms,
                             &technique.drawparams).unwrap();
        }
    }
}


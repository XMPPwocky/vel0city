#![feature(core)]

#[macro_use]
extern crate glium;
extern crate vel0city_base;
extern crate nalgebra as na;
extern crate byteorder;
extern crate image;

pub mod bsp;
pub mod q3_import;

use cast::{
    CastResult,
    Ray
};

pub struct Model {
    pub brush: u32,
    pub n_brushes: u32 
}
pub struct Entity {
    pub model: u32
}

pub struct Map {
    pub bsp: bsp::Tree,
    pub models: Vec<Model>,
    pub entities: Vec<Entity>
}

impl Map {
    pub fn cast_ray(&self, ray: &Ray) -> Option<CastResult> {
        let mut best = self.bsp.cast_ray(ray);
        for (entityidx, entity) in self.entities.iter().enumerate() {
            let model = &self.models[entity.model as usize];
            for brush in &self.bsp.brushes[model.brush as usize .. (model.brush + model.n_brushes) as usize] {
                let mut brushcast = brush.cast_ray(ray, (0.0, 1.0));
                if let Some(brushcast) = brushcast.as_mut() {
                    brushcast.entity = Some(entityidx as u32);
                }
                best = cast::combine_results(best, brushcast);
            }
        }

        best
    }
}

#[derive(Debug)]
pub struct MapFace {
    pub texture: i32,
    pub lightmap: i32,
    pub index_start: u32,
    pub index_count: u32,
}

#[derive(Copy, Clone, Debug)]
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

pub mod cast {
    use na;

    /// Secretly not a ray, it can have thickness to it.
    pub struct Ray {
        pub orig: na::Pnt3<f32>,
        pub dir: na::Vec3<f32>,
        pub halfextents: na::Vec3<f32>,
    }

    #[derive(Copy, Clone,Debug, PartialEq)]
    pub struct CastResult {
        /// Time of impact.
        pub toi: f32,
        /// Normal of the plane it hit. 
        pub norm: na::Vec3<f32>,

        /// Entity hit by the cast.
        pub entity: Option<u32>
    }
    pub fn combine_results(a: Option<CastResult>, b: Option<CastResult>) -> Option<CastResult> {
        if let Some(a) = a {
            match b {
                Some(b) => {
                    if a.toi <= b.toi {
                        Some(a)
                    } else {
                        Some(b)
                    }
                },
                None => Some(a)
            }
        } else {
            b
        }
    }

}

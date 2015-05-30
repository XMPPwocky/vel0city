use wavefront_obj;
use Vertex;
use glium;
use std::sync::Arc;
use glium::index::PrimitiveType::TrianglesList;

pub fn obj_to_model(obj: &wavefront_obj::obj::Object,
                    program: Arc<glium::Program>,
                    texture: glium::Texture2d,
                    display: &glium::Display) -> ::Model {
    
    use wavefront_obj::obj::Shape::Triangle;

    let mut verts = vec![];
    for v in obj.vertices.iter() {
        verts.push(Vertex { position: [v.x as f32, v.y as f32, v.z as f32,], texcoords: [1.0, 0.5] });
    }

    let mut triangles = vec![];
    for (v1, v2, v3) in obj.geometry[0].shapes.iter()
        .filter_map(|shape|
             if let &Triangle(v1, v2, v3) = shape {
                 Some((v1, v2, v3))
             } else {
                 None
             }) {

            triangles.push(v1.0 as u32);
            triangles.push(v2.0 as u32);
            triangles.push(v3.0 as u32);

        }

    ::Model {
        mesh: glium::VertexBuffer::new(display, verts),
        indices: glium::IndexBuffer::new(display, TrianglesList, triangles),
        program: program,
        texture: texture
    }
}

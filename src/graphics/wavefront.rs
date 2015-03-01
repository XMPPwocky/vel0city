use wavefront_obj;
use graphics::{self,
    Vertex
};
use glium;

pub fn obj_to_model(obj: &wavefront_obj::obj::Object,
                    program: Arc<glium::Program>,
                    texture: glium::Texture2d,
                    display: &glium::Display) -> graphics::Model {
    
    use wavefront_obj::obj::Shape::Triangle;

    let mut verts = vec![];
    let mut triangles = vec![];
    for (v1, v2, v3) in &obj.geometries[0].shapes
        .map(|shape|
             if let &Triangle(v1, v2, v3) = shape {
                 (v1, v2, v3)
             } else {
                 panic!("only triangles for now");
             }) {

        let v = obj.vertices[v1];
        let t = obj.tex_vertices[v1];

        verts.push(Vertex {
            position: [v.x, v.y, v.z],
            texcoords: [t.x, t.y] 
        });
        triangles.push(verts.len());

        let v = obj.vertices[v2];
        let t = obj.tex_vertices[v2];

        verts.push(Vertex {
            position: [v.x, v.y, v.z],
            texcoords: [t.x, t.y] 
        });
        triangles.push(verts.len());

        let v = obj.vertices[v3];
        let t = obj.tex_vertices[v3];

        verts.push(Vertex {
            position: [v.x, v.y, v.z],
            texcoords: [t.x, t.y] 
        });
        triangles.push(verts.len());

        }
    
    Model {
        mesh: glium::VertexBuffer::new(display, verts),
        indices: glium::IndexBuffer::new(display, glium::index::TrianglesList(triangles)),
        program: program,
        texture: texture
    }
}

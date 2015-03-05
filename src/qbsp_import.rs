#![allow(dead_code, unused_variables)]
use assets;
use bsp;
use byteorder::{self, LittleEndian, ReadBytesExt};
use std::io::{Cursor, SeekFrom, Seek};
use graphics;
use glium;
use na;

#[derive(Debug)]
pub enum BspError {
    ByteOrderError(byteorder::Error)
}
impl ::std::error::FromError<byteorder::Error> for BspError {
    fn from_error(e: byteorder::Error) -> BspError {
        BspError::ByteOrderError(e)
    }
}
pub fn import_collision(data: &[u8]) -> Result<bsp::Tree, BspError> {
    let directory = try!(read_directory(data));
    let planes = try!(read_planes(directory.planes));
    let nodes = try!(read_nodes(directory.nodes, &planes));
    let model = try!(read_model(directory.models));
    let leaves = try!(read_leaves(directory.leaves)); 

    Ok(bsp::Tree {
        root: model.bsp_root,
        leaves: leaves,
        inodes: nodes,
    })
}

pub fn import_graphics_model(data: &[u8], display: &glium::Display) -> Result<graphics::Model, BspError> {
    let directory = try!(read_directory(data));
    let faces = try!(read_faces(directory.faces));
    let ledges = try!(read_ledges(directory.ledges));
    let edges = try!(read_edges(directory.edges));
    let vertices = try!(read_vertices(directory.vertices));

    let mut indices = vec![];
    let mut verts = vec![];
    let mut face_edges = vec![];
    let mut face_verts = vec![];
    for face in faces {
        face_edges.clear();
        face_verts.clear();
        face_edges.extend(ledges.iter().skip(face.ledge_id as usize).take(face.ledge_num as usize)
            .map(|&ledge| if ledge > 0 { edges[ledge as usize] } else { let e = edges[-ledge as usize]; Edge { vertex0: e.vertex1, vertex1: e.vertex0 } }));
            
        if face_edges.len() < 3 {
            continue;
        }

        face_verts.push(face_edges[0].vertex0);
        face_verts.extend(face_edges[0..face_edges.len() - 1].iter().map(|edge| edge.vertex1));

        let mut average = na::zero();
        for &vert in &face_verts {
            let vert = &vertices[vert as usize];
            average = average + na::Vec3::new(vert.x, vert.y, vert.z);
        }
        average = average * (1.0 / face_verts.len() as f32);
        verts.push(graphics::Vertex {
            position: [average.x, average.z, average.y],
            texcoords: [1.0, 1.0]
        });
        let center = (verts.len() - 1) as u32;

        for (id, &vert) in face_verts.iter().enumerate() {
            let vert = &vertices[vert as usize];
            verts.push(graphics::Vertex {
                position: [vert.x, vert.z, vert.y],
                texcoords: [0.0, 0.0]
            });
            let id = id;

            let prev = ((id - 1) % face_verts.len()) as u32;

            indices.push(center);
            indices.push(center + prev + 1);
            indices.push(center + id as u32 + 1);
        }

    }
    println!("{:?}", &indices[0..12]);
    println!("{}", indices.len());

    let tex = vec![
        vec![(0u8, 0u8, 255u8), (0u8, 255u8, 0u8)],
        vec![(255u8, 0u8, 0u8), (255u8, 255u8, 255u8)]
    ];
    let tex = glium::Texture2d::new(display, tex);
    let program = glium::Program::from_source(
        &display,
        &assets::load_str_asset("vertex.glsl").unwrap(),
        &assets::load_str_asset("fragment.glsl").unwrap(),
        None
        ).unwrap();


    Ok(graphics::Model {
        mesh: glium::VertexBuffer::new(display, verts),
        indices: glium::IndexBuffer::new(display, glium::index::TrianglesList(indices)),
        program: ::std::sync::Arc::new(program),
        texture: tex
    })
}

struct Directory<'a> {
    planes: &'a [u8],
    vertices: &'a [u8],
    nodes: &'a [u8],
    faces: &'a [u8],
    clipnodes: &'a [u8],
    leaves: &'a [u8],
    edges: &'a [u8],
    ledges: &'a [u8],
    models: &'a [u8]
}

fn read_directory(data: &[u8]) -> byteorder::Result<Directory> {
    let mut cursor = Cursor::new(data);

    cursor.seek(SeekFrom::Start(4 + 8)).unwrap();
    let planes_offset = try!(cursor.read_u32::<LittleEndian>());
    let planes_len = try!(cursor.read_u32::<LittleEndian>());

    cursor.seek(SeekFrom::Current(8)).unwrap();
    let vertices_offset = try!(cursor.read_u32::<LittleEndian>());
    let vertices_len = try!(cursor.read_u32::<LittleEndian>());
     
    cursor.seek(SeekFrom::Current(8)).unwrap();
    let nodes_offset = try!(cursor.read_u32::<LittleEndian>());
    let nodes_len = try!(cursor.read_u32::<LittleEndian>());

    cursor.seek(SeekFrom::Current(8)).unwrap();
    let faces_offset = try!(cursor.read_u32::<LittleEndian>());
    let faces_len = try!(cursor.read_u32::<LittleEndian>());

    cursor.seek(SeekFrom::Current(8)).unwrap();
    let clipnodes_offset = try!(cursor.read_u32::<LittleEndian>());
    let clipnodes_len = try!(cursor.read_u32::<LittleEndian>());
     
    cursor.seek(SeekFrom::Current(0)).unwrap();
    let leaves_offset = try!(cursor.read_u32::<LittleEndian>());
    let leaves_len = try!(cursor.read_u32::<LittleEndian>());

    cursor.seek(SeekFrom::Current(8)).unwrap();
    let edges_offset = try!(cursor.read_u32::<LittleEndian>());
    let edges_len = try!(cursor.read_u32::<LittleEndian>());

    
    cursor.seek(SeekFrom::Current(0)).unwrap();
    let ledges_offset = try!(cursor.read_u32::<LittleEndian>());
    let ledges_len = try!(cursor.read_u32::<LittleEndian>());

    cursor.seek(SeekFrom::Current(0)).unwrap();
    let models_offset = try!(cursor.read_u32::<LittleEndian>());
    let models_len = try!(cursor.read_u32::<LittleEndian>());

    Ok(Directory {
        planes: &data[planes_offset as usize .. (planes_offset + planes_len) as usize],
        vertices: &data[vertices_offset as usize .. (vertices_offset + vertices_len) as usize], 
        nodes: &data[nodes_offset as usize .. (nodes_offset + nodes_len) as usize], 
        faces: &data[faces_offset as usize .. (faces_offset + faces_len) as usize], 
        clipnodes: &data[clipnodes_offset as usize .. (clipnodes_offset + clipnodes_len) as usize], 
        leaves: &data[leaves_offset as usize .. (leaves_offset + leaves_len) as usize],
        edges: &data[edges_offset as usize .. (edges_offset + edges_len) as usize],
        ledges: &data[ledges_offset as usize .. (ledges_offset + ledges_len) as usize],
        models: &data[models_offset as usize .. (models_offset + models_len) as usize] 
    })
}

fn read_plane(data: &[u8]) -> byteorder::Result<bsp::Plane> {
    let mut cursor = Cursor::new(data);

    let n_x = try!(cursor.read_f32::<LittleEndian>()); 
    let n_y = try!(cursor.read_f32::<LittleEndian>()); 
    let n_z = try!(cursor.read_f32::<LittleEndian>()); 
    let dist = try!(cursor.read_f32::<LittleEndian>()); 

    Ok(bsp::Plane {
        norm: na::Vec3::new(n_x, n_z, n_y),
        dist: dist
    })
}
fn read_planes(data: &[u8]) -> byteorder::Result<Vec<bsp::Plane>> {
    data.chunks(20)
        .map(|chunk| read_plane(chunk))
        .collect()
}


fn read_node(data: &[u8], planes: &[bsp::Plane]) -> byteorder::Result<bsp::InnerNode> {
    let mut cursor = Cursor::new(data);

    let plane_id = try!(cursor.read_u32::<LittleEndian>()); 
    let front = try!(cursor.read_i16::<LittleEndian>()); 
    let back = try!(cursor.read_i16::<LittleEndian>()); 

    Ok(bsp::InnerNode {
        plane: planes[plane_id as usize].clone(),
        pos: front as i32,
        neg: back as i32,
    })
}
fn read_nodes(data: &[u8], planes: &[bsp::Plane]) -> byteorder::Result<Vec<bsp::InnerNode>> {
    data.chunks(24)
        .map(|chunk| read_node(chunk, planes))
        .collect()
}

fn read_clipnode(data: &[u8], planes: &[bsp::Plane]) -> byteorder::Result<bsp::InnerNode> {
    let mut cursor = Cursor::new(data);

    let plane_id = try!(cursor.read_u32::<LittleEndian>()); 
    let front = try!(cursor.read_i16::<LittleEndian>()); 
    let back = try!(cursor.read_i16::<LittleEndian>()); 

    Ok(bsp::InnerNode {
        plane: planes[plane_id as usize].clone(),
        pos: front as i32,
        neg: back as i32,
    })
}
fn read_clipnodes(data: &[u8], planes: &[bsp::Plane]) -> byteorder::Result<Vec<bsp::InnerNode>> {
    data.chunks(8)
        .map(|chunk| read_node(chunk, planes))
        .collect()
}

fn read_leaves(data: &[u8]) -> byteorder::Result<Vec<bsp::Leaf>> {
    let mut leaves: Vec<_> = data.chunks(30)
        .map(|_chunk| bsp::Leaf { solid: true }) 
        .collect();
    leaves[0].solid = false;
    Ok(leaves)
}

struct Model {
    bsp_root: i32,
    clip_root: i32
}

fn read_model(data: &[u8]) -> byteorder::Result<Model> {
    let mut cursor = Cursor::new(data);
    cursor.seek(SeekFrom::Start(28)).unwrap();
    let bsp_root = try!(cursor.read_i32::<LittleEndian>());
    let clip_root = try!(cursor.read_i32::<LittleEndian>());
    Ok(Model{
        bsp_root: bsp_root,
        clip_root: clip_root
    })
}

#[derive(Debug)]
struct Face {
    ledge_id: u32,
    ledge_num: u16
}

fn read_face(data: &[u8]) -> byteorder::Result<Face> {
    let mut cursor = Cursor::new(data);
    cursor.seek(SeekFrom::Start(4)).unwrap();
    let ledge_id = try!(cursor.read_u32::<LittleEndian>());
    let ledge_num = try!(cursor.read_u16::<LittleEndian>());

    Ok(Face {
        ledge_id: ledge_id,
        ledge_num: ledge_num
    })
}

fn read_faces(data: &[u8]) -> byteorder::Result<Vec<Face>> {
    data.chunks(20)
        .map(|chunk| read_face(chunk))
        .collect()
}

fn read_ledges(data: &[u8]) -> byteorder::Result<Vec<i32>> {
    data.chunks(4)
        .map(|chunk| {
            let mut cursor = Cursor::new(chunk); cursor.read_i32::<LittleEndian>()
        })
        .collect()
}

#[derive(Debug, Copy)]
struct Edge {
    vertex0: u16,
    vertex1: u16
}

fn read_edge(data: &[u8]) -> byteorder::Result<Edge> {
    let mut cursor = Cursor::new(data);
    let vertex0 = try!(cursor.read_u16::<LittleEndian>());
    let vertex1 = try!(cursor.read_u16::<LittleEndian>());

    Ok(Edge {
        vertex0: vertex0,
        vertex1: vertex1,
    })
}

fn read_edges(data: &[u8]) -> byteorder::Result<Vec<Edge>> {
    data.chunks(4)
        .map(|chunk| read_edge(chunk))
        .collect()
}

#[derive(Debug)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32
}

fn read_vertex(data: &[u8]) -> byteorder::Result<Vertex> {
    let mut cursor = Cursor::new(data);
    let x = try!(cursor.read_f32::<LittleEndian>());
    let y = try!(cursor.read_f32::<LittleEndian>());
    let z = try!(cursor.read_f32::<LittleEndian>());

    Ok(Vertex {
        x: x,
        y: y,
        z: z
    })
}

fn read_vertices(data: &[u8]) -> byteorder::Result<Vec<Vertex>> {
    data.chunks(12)
        .map(|chunk| read_vertex(chunk))
        .collect()
}

#[test]
fn import() {
}

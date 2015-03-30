#![allow(dead_code, unused_variables)]
use assets;
use bsp;
use byteorder::{self, LittleEndian, ReadBytesExt};
use std::io::{Cursor, SeekFrom, Seek};
use glium;
use image;
use na;
use map::{
    GraphicsMap,
    MapVertex,
    MapFace
};

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
    let leaves = try!(read_leaves(directory.leaves)); 
    let textures = try!(read_textures(directory.textures));
    let brushsides = try!(read_brushsides(directory.brushsides, &planes, &textures));
    let brushes = try!(read_brushes(directory.brushes, &brushsides));
    let leafbrushes = try!(read_leafbrushes(directory.leafbrushes));

    Ok(bsp::Tree {
        brushes: brushes,
        leafbrushes: leafbrushes, 
        leaves: leaves,
        inodes: nodes,
    })
}

pub fn import_graphics_model(data: &[u8], display: &glium::Display) -> Result<GraphicsMap, BspError> {
    let directory = try!(read_directory(data));
    let faces = try!(read_faces(directory.faces));
    let vertices = try!(read_vertices(directory.vertices));
    let meshverts = try!(read_meshverts(directory.meshverts));
    let textures = try!(read_textures(directory.textures));
    let lightmaps = try!(read_lightmaps(directory.lightmaps));

    let mut indices = vec![];
    let mut fixed_faces = vec![];
    for face in faces {
        let index_start = indices.len();
        for meshvert in &meshverts[face.meshvert as usize.. (face.meshvert + face.n_meshverts) as usize] {
            indices.push(face.vertex as u32 + *meshvert);
        }
        let index_end = indices.len();

        fixed_faces.push(MapFace {
            texture: face.texture,
            index_start: index_start as u32,
            index_count: (index_end - index_start) as u32,
            lightmap: face.lightmap,
        });
    }



    let loaded_textures = textures.iter().map(|tex| {
        debug!("Loading {:?}", tex.name);
        let contents = assets::load_bin_asset(&(tex.name.clone() + ".png")).unwrap_or_else(|_| assets::load_bin_asset("textures/radiant/notex.png").unwrap());
        let image = image::load(::std::io::Cursor::new(contents), image::PNG).unwrap();
        let texture = glium::Texture2d::new(display, image);
        texture
    }).collect();
    let loaded_lightmaps = lightmaps.into_iter().map(|lm|  
                                                     glium::Texture2d::new(display, lm.data)
                                                    ).collect();
    let loaded_vertices = vertices.iter().map(|vert| {
        MapVertex {
            position: [vert.position.x, -1.0 * vert.position.z, vert.position.y],
            texcoords: [1.0 - vert.texcoords.x, 1.0 - vert.texcoords.y],
            lightmaptexcoords: [vert.lightmaptexcoords.x, vert.lightmaptexcoords.y],
            normal: [vert.normal.x, -1.0 * vert.normal.z, vert.normal.y]
        }
    }).collect();

    let main_program = glium::Program::from_source(
        display,
        &assets::load_str_asset("shaders/prepass/vertex.glsl").unwrap(),
        &assets::load_str_asset("shaders/prepass/fragment.glsl").unwrap(),
        None
        ).unwrap();

    Ok(GraphicsMap {
        vertices: glium::VertexBuffer::new(display, loaded_vertices),
        indices: glium::IndexBuffer::new(display, glium::index::TrianglesList(indices)),
        shaders: vec![main_program],
        textures: loaded_textures,
        lightmaps: loaded_lightmaps,
        faces: fixed_faces,
    })
}

struct Directory<'a> {
    textures: &'a [u8],
    planes: &'a [u8],
    nodes: &'a [u8],
    leaves: &'a [u8],
    leafbrushes: &'a [u8],
    brushes: &'a [u8],
    brushsides: &'a [u8],
    vertices: &'a [u8],
    meshverts: &'a [u8],
    faces: &'a [u8],
    lightmaps: &'a [u8],
}

fn read_directory(data: &[u8]) -> byteorder::Result<Directory> {
    let mut cursor = Cursor::new(data);

    cursor.seek(SeekFrom::Start(8 + 8)).unwrap();
    let textures_offset = try!(cursor.read_u32::<LittleEndian>());
    let textures_len = try!(cursor.read_u32::<LittleEndian>());

    cursor.seek(SeekFrom::Current(0)).unwrap();
    let planes_offset = try!(cursor.read_u32::<LittleEndian>());
    let planes_len = try!(cursor.read_u32::<LittleEndian>());

    cursor.seek(SeekFrom::Current(0)).unwrap();
    let nodes_offset = try!(cursor.read_u32::<LittleEndian>());
    let nodes_len = try!(cursor.read_u32::<LittleEndian>());

    cursor.seek(SeekFrom::Current(0)).unwrap();
    let leaves_offset = try!(cursor.read_u32::<LittleEndian>());
    let leaves_len = try!(cursor.read_u32::<LittleEndian>());

    cursor.seek(SeekFrom::Current(8*1)).unwrap();
    let leafbrushes_offset = try!(cursor.read_u32::<LittleEndian>());
    let leafbrushes_len = try!(cursor.read_u32::<LittleEndian>());

    cursor.seek(SeekFrom::Current(8*1)).unwrap();
    let brushes_offset = try!(cursor.read_u32::<LittleEndian>());
    let brushes_len = try!(cursor.read_u32::<LittleEndian>());

    cursor.seek(SeekFrom::Current(0)).unwrap();
    let brushsides_offset = try!(cursor.read_u32::<LittleEndian>());
    let brushsides_len = try!(cursor.read_u32::<LittleEndian>());

    cursor.seek(SeekFrom::Current(0)).unwrap();
    let vertices_offset = try!(cursor.read_u32::<LittleEndian>());
    let vertices_len = try!(cursor.read_u32::<LittleEndian>());

    cursor.seek(SeekFrom::Current(0)).unwrap();
    let meshverts_offset = try!(cursor.read_u32::<LittleEndian>());
    let meshverts_len = try!(cursor.read_u32::<LittleEndian>());

    cursor.seek(SeekFrom::Current(8*1)).unwrap();
    let faces_offset = try!(cursor.read_u32::<LittleEndian>());
    let faces_len = try!(cursor.read_u32::<LittleEndian>());

    cursor.seek(SeekFrom::Current(8*0)).unwrap();
    let lightmaps_offset = try!(cursor.read_u32::<LittleEndian>());
    let lightmaps_len = try!(cursor.read_u32::<LittleEndian>());

    Ok(Directory {
        textures: &data[textures_offset as usize .. (textures_offset + textures_len) as usize],
        planes: &data[planes_offset as usize .. (planes_offset + planes_len) as usize],
        nodes: &data[nodes_offset as usize .. (nodes_offset + nodes_len) as usize], 
        leaves: &data[leaves_offset as usize .. (leaves_offset + leaves_len) as usize],
        leafbrushes: &data[leafbrushes_offset as usize .. (leafbrushes_offset + leafbrushes_len) as usize],
        brushes: &data[brushes_offset as usize .. (brushes_offset + brushes_len) as usize],
        brushsides: &data[brushsides_offset as usize .. (brushsides_offset + brushsides_len) as usize],
        vertices: &data[vertices_offset as usize .. (vertices_offset + vertices_len) as usize], 
        meshverts: &data[meshverts_offset as usize .. (meshverts_offset + meshverts_len) as usize], 
        faces: &data[faces_offset as usize .. (faces_offset + faces_len) as usize], 
        lightmaps: &data[lightmaps_offset as usize .. (lightmaps_offset + lightmaps_len) as usize], 
    })
}

fn read_plane(data: &[u8]) -> byteorder::Result<bsp::Plane> {
    let mut cursor = Cursor::new(data);

    let n_x = try!(cursor.read_f32::<LittleEndian>()); 
    let n_y = try!(cursor.read_f32::<LittleEndian>()); 
    let n_z = try!(cursor.read_f32::<LittleEndian>()); 
    let dist = try!(cursor.read_f32::<LittleEndian>()); 

    Ok(bsp::Plane {
        norm: na::Vec3::new(n_x, -n_z, n_y),
        dist: dist
    })
}
fn read_planes(data: &[u8]) -> byteorder::Result<Vec<bsp::Plane>> {
    data.chunks(16)
        .map(|chunk| read_plane(chunk))
        .collect()
}


fn read_node(data: &[u8], planes: &[bsp::Plane]) -> byteorder::Result<bsp::InnerNode> {
    let mut cursor = Cursor::new(data);

    let plane_id = try!(cursor.read_i32::<LittleEndian>()); 
    let front = try!(cursor.read_i32::<LittleEndian>()); 
    let back = try!(cursor.read_i32::<LittleEndian>()); 

    Ok(bsp::InnerNode {
        plane: planes[plane_id as usize].clone(),
        pos: front as i32,
        neg: back as i32,
    })
}
fn read_nodes(data: &[u8], planes: &[bsp::Plane]) -> byteorder::Result<Vec<bsp::InnerNode>> {
    data.chunks(36)
        .map(|chunk| read_node(chunk, planes))
        .collect()
}

fn read_brushside(data: &[u8], planes: &[bsp::Plane], textures: &[Texture]) -> byteorder::Result<bsp::BrushSide> {
    let mut cursor = Cursor::new(data);
    let plane_id = try!(cursor.read_i32::<LittleEndian>());
    let texture_id = try!(cursor.read_i32::<LittleEndian>());
    let tex = &textures[texture_id as usize];
    Ok(bsp::BrushSide {
        plane: planes[plane_id as usize].clone(),
        contents: tex.contents,
        flags: tex.flags
    })
}

fn read_brushsides(data: &[u8], planes: &[bsp::Plane], textures: &[Texture]) -> byteorder::Result<Vec<bsp::BrushSide>> {
    data.chunks(8)
        .map(|chunk| read_brushside(chunk, planes, textures))
        .collect()
}

fn read_brushes(data: &[u8], brushsides: &[bsp::BrushSide]) -> byteorder::Result<Vec<bsp::Brush>> {
    data.chunks(12)
        .map(|chunk| read_brush(chunk, brushsides))
        .collect()
}

fn read_brush(data: &[u8], brushsides: &[bsp::BrushSide]) -> byteorder::Result<bsp::Brush> {
    let mut cursor = Cursor::new(data);
    let brushside = try!(cursor.read_i32::<LittleEndian>());
    let n_brushsides = try!(cursor.read_i32::<LittleEndian>());
    Ok(bsp::Brush {
        sides: brushsides[brushside as usize .. (brushside + n_brushsides) as usize].to_vec()
    })
}


fn read_leaf(data: &[u8]) -> byteorder::Result<bsp::Leaf> {
    let mut cursor = Cursor::new(data);
    cursor.seek(SeekFrom::Start(40)).unwrap();

    let leafbrush = try!(cursor.read_i32::<LittleEndian>()); 
    let n_leafbrushes = try!(cursor.read_i32::<LittleEndian>()); 
    Ok(bsp::Leaf {
        leafbrush: leafbrush,
        n_leafbrushes: n_leafbrushes
    })
}

fn read_leaves(data: &[u8]) -> byteorder::Result<Vec<bsp::Leaf>> {
    data.chunks(48)
        .map(|chunk| read_leaf(chunk))
        .collect()
}

fn read_leafbrushes(data: &[u8]) -> byteorder::Result<Vec<u32>> {
    data.chunks(4)
        .map(|chunk| {
            let mut cursor = Cursor::new(chunk);
            cursor.read_u32::<LittleEndian>()
        })
        .collect()
}

fn read_meshverts(data: &[u8]) -> byteorder::Result<Vec<u32>> {
    data.chunks(4)
        .map(|chunk| {
            let mut cursor = Cursor::new(chunk);
            cursor.read_u32::<LittleEndian>()
        })
        .collect()
}

struct Model {
    face: i32,
    n_faces: i32,
}

fn read_model(data: &[u8]) -> byteorder::Result<Model> {
    let mut cursor = Cursor::new(data);
    cursor.seek(SeekFrom::Start(28)).unwrap();
    unimplemented!();
    
}

#[derive(Debug)]
struct Face {
    texture: i32,
    lightmap: i32,
    vertex: i32,
    n_vertexes: i32,
    meshvert: i32,
    n_meshverts: i32,
}

fn read_face(data: &[u8]) -> byteorder::Result<Face> {
    let mut cursor = Cursor::new(data);
    let texture = try!(cursor.read_i32::<LittleEndian>()); 
    cursor.seek(SeekFrom::Current(8)).unwrap();
    let vertex = try!(cursor.read_i32::<LittleEndian>()); 
    let n_vertexes = try!(cursor.read_i32::<LittleEndian>()); 
    let meshvert = try!(cursor.read_i32::<LittleEndian>()); 
    let n_meshverts = try!(cursor.read_i32::<LittleEndian>()); 
    let lightmap = try!(cursor.read_i32::<LittleEndian>()); 

    Ok(Face {
        texture: texture,
        vertex: vertex,
        n_vertexes: n_vertexes,
        meshvert: meshvert,
        n_meshverts: n_meshverts, 
        lightmap: lightmap,
    })
}

fn read_faces(data: &[u8]) -> byteorder::Result<Vec<Face>> {
    data.chunks(104)
        .map(|chunk| read_face(chunk))
        .collect()
}

#[derive(Debug)]
struct Texture {
    name: String, 
    flags: i32,
    contents: i32,
}

fn read_texture(data: &[u8]) -> byteorder::Result<Texture> {
    let mut cursor = Cursor::new(data);
    let name = &data[0..64];
    let namelen = name.iter()
        .position(|&c| c == 0)
        .unwrap_or(name.len());
    let name = String::from_utf8_lossy(&name[..namelen]).to_string();

    let flags = try!(cursor.read_i32::<LittleEndian>());
    let contents = try!(cursor.read_i32::<LittleEndian>());

    Ok(Texture {
        name: name,
        flags: flags,
        contents: contents
    })
}

fn read_textures(data: &[u8]) -> byteorder::Result<Vec<Texture>> {
    data.chunks(72)
        .map(|chunk| read_texture(chunk))
        .collect()
}

#[derive(Debug)]
struct Lightmap {
    data: Vec<Vec<(u8, u8, u8)>>
}
fn read_lightmaps(data: &[u8]) -> byteorder::Result<Vec<Lightmap>> {
    data.chunks(128*128*3)
        .map(|row| Ok(Lightmap { data: try!(row.chunks(128*3)
             .map(|col| col.chunks(3).map(|px| {

                 let mut cursor = Cursor::new(px);
                 let r = try!(cursor.read_u8());
                 let g = try!(cursor.read_u8());
                 let b = try!(cursor.read_u8());
                 Ok((r, g, b))
             })
                  .collect::<byteorder::Result<Vec<_>>>())
             .collect::<Result<Vec<_>, _>>()) }))
        .collect::<Result<Vec<_>, _>>()
}

struct Vertex {
    position: na::Vec3<f32>,
    texcoords: na::Vec2<f32>,
    lightmaptexcoords: na::Vec2<f32>,
    normal: na::Vec3<f32>,
}
fn read_vertex(data: &[u8]) -> byteorder::Result<Vertex> {
    let mut cursor = Cursor::new(data);
    let p_x = try!(cursor.read_f32::<LittleEndian>());
    let p_y = try!(cursor.read_f32::<LittleEndian>());
    let p_z = try!(cursor.read_f32::<LittleEndian>());
    
    let t_x = try!(cursor.read_f32::<LittleEndian>());
    let t_y = try!(cursor.read_f32::<LittleEndian>());
    let lt_x = try!(cursor.read_f32::<LittleEndian>());
    let lt_y = try!(cursor.read_f32::<LittleEndian>());
    let n_x = try!(cursor.read_f32::<LittleEndian>());
    let n_y = try!(cursor.read_f32::<LittleEndian>());
    let n_z = try!(cursor.read_f32::<LittleEndian>());

    Ok(Vertex {
        position: na::Vec3::new(p_x, p_y, p_z),
        texcoords: na::Vec2::new(t_x, t_y),
        lightmaptexcoords: na::Vec2::new(lt_x, lt_y),
        normal: na::Vec3::new(n_x, n_y, n_z)
    })
}

fn read_vertices(data: &[u8]) -> byteorder::Result<Vec<Vertex>> {
    data.chunks(44)
        .map(|chunk| read_vertex(chunk))
        .collect()
}

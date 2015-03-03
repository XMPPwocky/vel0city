use bsp;
use byteorder::{self, LittleEndian, ReadBytesExt};
use std::io::{Cursor, SeekFrom, Seek};
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
pub fn import_bsp(data: &[u8]) -> Result<bsp::Tree, BspError> {
    let directory = try!(read_directory(data));
    let planes = try!(read_planes(directory.planes));
    let nodes = try!(read_nodes(directory.nodes, &planes));
    let leaves = try!(read_leaves(directory.leaves)); 
    let root = try!(read_model_for_root(directory.models));

    Ok(bsp::Tree {
        root: root,
        leaves: leaves,
        inodes: nodes
    })
}

struct Directory<'a> {
    planes: &'a [u8],
    nodes: &'a [u8],
    leaves: &'a [u8],
    models: &'a [u8]
}

fn read_directory(data: &[u8]) -> byteorder::Result<Directory> {
    let mut cursor = Cursor::new(data);

    cursor.seek(SeekFrom::Start(4 + 8)).unwrap();
    let planes_offset = try!(cursor.read_u32::<LittleEndian>());
    let planes_len = try!(cursor.read_u32::<LittleEndian>());
     
    cursor.seek(SeekFrom::Current(24)).unwrap();
    let nodes_offset = try!(cursor.read_u32::<LittleEndian>());
    let nodes_len = try!(cursor.read_u32::<LittleEndian>());
     
    cursor.seek(SeekFrom::Current(32)).unwrap();
    let leaves_offset = try!(cursor.read_u32::<LittleEndian>());
    let leaves_len = try!(cursor.read_u32::<LittleEndian>());
    
    cursor.seek(SeekFrom::Current(24)).unwrap();
    let models_offset = try!(cursor.read_u32::<LittleEndian>());
    let models_len = try!(cursor.read_u32::<LittleEndian>());

    Ok(Directory {
        planes: &data[planes_offset as usize .. (planes_offset + planes_len) as usize],
        nodes: &data[nodes_offset as usize .. (nodes_offset + nodes_len) as usize], 
        leaves: &data[leaves_offset as usize .. (leaves_offset + leaves_len) as usize],
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
        norm: na::Vec3::new(n_x, n_y, n_z),
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

fn read_leaves(data: &[u8]) -> byteorder::Result<Vec<bsp::Leaf>> {
    let mut leaves: Vec<_> = data.chunks(30)
        .map(|_chunk| bsp::Leaf { solid: false }) 
        .collect();
    leaves[0].solid = true;
    Ok(leaves)
}

fn read_model_for_root(data: &[u8]) -> byteorder::Result<i32> {
    let mut cursor = Cursor::new(data);
    cursor.seek(SeekFrom::Start(28)).unwrap();
    cursor.read_i32::<LittleEndian>()
}
    
#[test]
fn import() {
}

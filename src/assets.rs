use std::io;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;

fn name_to_path(name: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push("assets/");
    path.push(name);
    path
}

pub fn load_bin_asset(name: &str) -> io::Result<Vec<u8>> {
    let path = name_to_path(name);

    let mut v = Vec::new();
    try!(File::open(&path).and_then(|mut f| f.read_to_end(&mut v)));
    Ok(v)
}

pub fn load_str_asset(name: &str) -> io::Result<String> {
    let path = name_to_path(name);

    let mut v = String::new();
    try!(File::open(&path).and_then(|mut f| f.read_to_string(&mut v)));
    Ok(v)
}

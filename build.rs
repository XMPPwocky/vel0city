#![feature(old_path)]

extern crate capnpc;

fn main() {
	let res = compile(Path::new("."), &[Path::new("netcode.capnp")]);
    match res {
        Ok(_) => (),
        Err(std::old_io::IoError { kind: std::old_io::EndOfFile, .. }) => (),
        Err(e) => Err(e).unwrap()
    }
}

pub fn compile(prefix : Path, files : &[Path]) -> ::std::old_io::IoResult<()> {
    let out_dir = Path::new(::std::env::var("OUT_DIR").unwrap());
    let cwd = try!(::std::env::current_dir());
    try!(::std::env::set_current_dir(&out_dir));

    let mut command = ::std::old_io::Command::new("capnp");
    command
        .arg("compile")
        .arg("-orust")
        .arg(format!("--src-prefix={}", cwd.join(prefix).display()));

    for file in files.iter() {
        command.arg(format!("{}", cwd.join(file).display()));
    }

    command.stdout(::std::old_io::process::CreatePipe(false, true));
    command.stderr(::std::old_io::process::InheritFd(2));

    let mut p =  try!(command.spawn());
    let mut child_stdout = p.stdout.take().unwrap();
    try!(capnpc::codegen::main(&mut child_stdout));
    try!(p.wait());
    Ok(())
}

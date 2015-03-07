#![feature(old_path)]

extern crate capnpc;

fn main() {
    return;
    /*
	let res = compile(Path::new("."), &[Path::new("netcode.capnp")]);
    match res {
        Ok(_) => (),
        Err(std::old_io::IoError { kind: std::old_io::EndOfFile, .. }) => (),
        Err(e) => Err(e).unwrap()
    }*/
}

pub fn compile(prefix : Path, files : &[Path]) {
/*
    let out_dir = Path::new(::std::env::var("OUT_DIR").unwrap());
    let cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(&out_dir).unwrap();

    let mut command = ::std::old_io::Command::new("capnp");
    command
        .arg("compile")
        .arg("-orust")
        .arg(format!("--src-prefix={}", cwd.join(&prefix).display()));

    for file in files.iter() {
        command.arg(format!("{}", cwd.join(file).display()));
    }

    command.stdout(::std::old_io::process::CreatePipe(false, true));
    command.stderr(::std::old_io::process::InheritFd(2));

    let mut p = command.spawn().unwrap();
    let mut child_stdout = p.stdout.take().unwrap();
    capnpc::codegen::main(&mut child_stdout).unwrap();
    p.wait().unwrap();
*/
}

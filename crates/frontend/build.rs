use npm_rs::*;
use std::{fs, io};

fn main() {
    // TODO: add cargo:rerun-if-changed for important files

    let paths = fs::read_dir(".")
        .unwrap()
        .filter(|entry| entry.is_ok())
        .map(|file| file.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    /*for path in paths {
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }*/
    //println!("cargo:rerun-if-changed=src/main.ts");

    NpmEnv::default()
        .with_node_env(&NodeEnv::from_cargo_profile().unwrap_or_default())
        .with_env("FOO", "bar")
        .init_env()
        .install(None)
        .run("build")
        .exec()
        .unwrap();
}

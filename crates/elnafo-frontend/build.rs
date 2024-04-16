use ignore::Walk;
use npm_rs::*;

fn main() {
    for entry in Walk::new(".")
        .filter_map(Result::ok)
        .filter(|e| !e.path().is_dir())
        .filter(|e| e.file_name() != "package-lock.json")
    {
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }

    NpmEnv::default()
        .with_node_env(&NodeEnv::from_cargo_profile().unwrap_or_default())
        .init_env()
        .install(None)
        .run("build")
        .exec()
        .unwrap();
}

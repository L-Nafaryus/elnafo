use npm_rs::*;

fn main() {
    let exit_status = NpmEnv::default()
        .with_node_env(&NodeEnv::from_cargo_profile().unwrap_or_default())
        .with_env("FOO", "bar")
        .init_env()
        .install(None)
        .run("build")
        .exec();
}

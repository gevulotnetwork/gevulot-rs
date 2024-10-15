use prost_build::Config;
use std::{env::set_current_dir, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let mut config = Config::new();
    config.enable_type_names();

    set_current_dir("./").unwrap();
    tonic_buf_build::compile_from_buf_workspace(
        tonic_build::configure().build_client(true).out_dir(out_dir),
        Some(config),
    )
    .unwrap();
}

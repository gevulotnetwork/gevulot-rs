use prost_build::Config;
use std::io::Read;
use std::{env::set_current_dir, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let mut config = Config::new();
    config.enable_type_names();

    set_current_dir("./").unwrap();

    // Since there is no buf on Rust build environment, we manually (locally)
    // export all proto-files using buf (see ./scripts/prepare_publish.sh),
    // and add them to the package.
    // So when building on docs.rs, raw protoc is used.
    if std::env::var("DOCS_RS").is_ok() {
        // Read the list of proto files to compile from buf_exported/protos.txt
        let protos_file = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap())
            .join("buf_exported")
            .join("protos.txt");
        let mut buffer = String::new();
        std::fs::File::open(&protos_file)
            .expect("failed to open protos.txt")
            .read_to_string(&mut buffer)
            .expect("failed to read protos.txt");
        let protos = buffer
            .lines()
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();

        let includes = vec!["./proto", "./buf_exported"];

        let tonic_builder = tonic_build::configure().build_client(true).out_dir(out_dir);
        tonic_builder
            .compile_with_config(config, &protos, &includes)
            .unwrap();
    } else {
        tonic_buf_build::compile_from_buf_workspace(
            tonic_build::configure().build_client(true).out_dir(out_dir),
            Some(config),
        )
        .unwrap();
    }
}

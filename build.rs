use prost_build::Config;
use std::io::Read;
use std::{env::set_current_dir, path::PathBuf};

const DEFAULT_CHAIN_ID: &str = "gevulot";
const DEFAULT_TOKEN_DENOM: &str = "ucredit";

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
        // There is a bug in `tonic-buf-build` that causes build to always rerun.
        // We workaround this by emmiting valid rerun-s manually.
        // See https://github.com/Valensas/tonic-buf-build/issues/7.
        const TONIC_RERUN_IF_CHANGED: bool = false;
        println!("cargo:rerun-if-changed=proto");

        tonic_buf_build::compile_from_buf_workspace(
            tonic_build::configure()
                .build_client(true)
                .emit_rerun_if_changed(TONIC_RERUN_IF_CHANGED)
                .out_dir(out_dir),
            Some(config),
        )
        .unwrap();
    }

    let chain_id = std::env::var("GEVULOT_CHAIN_ID").unwrap_or(DEFAULT_CHAIN_ID.to_string());
    println!("cargo:rustc-env=GEVULOT_CHAIN_ID={}", chain_id);
    println!("cargo:rerun-if-env-changed=GEVULOT_CHAIN_ID");

    let denom = std::env::var("GEVULOT_TOKEN_DENOM").unwrap_or(DEFAULT_TOKEN_DENOM.to_string());
    println!("cargo:rustc-env=GEVULOT_TOKEN_DENOM={}", denom);
    println!("cargo:rerun-if-env-changed=GEVULOT_TOKEN_DENOM");
}

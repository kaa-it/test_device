use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(out_dir.join("myservice_descriptor.bin"))
        .compile(&["proto/command.proto"], &["proto"])?;

    println!("cargo:rerun-if-changed=proto/command.proto");

    Ok(())
}

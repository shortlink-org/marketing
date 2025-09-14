use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let protos = &[
        "src/infrastructure/rpc/newsletter/v1/newsletter.proto",
        "src/infrastructure/rpc/newsletter/v1/api.proto",
    ];
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let fds = out_dir.join("infrastructure.rpc.newsletter.v1_descriptor.bin");

    tonic_prost_build::configure()
        .file_descriptor_set_path(&fds) // <- generate descriptor set
        .build_client(true)
        .build_server(true)
        .compile_protos(protos, &["src"])?;

    for p in protos {
        println!("cargo:rerun-if-changed={}", p);
    }
    Ok(())
}

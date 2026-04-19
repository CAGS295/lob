#[cfg(feature = "grpc")]
fn main() {
    use std::{env, path::PathBuf};
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("limitorderbook_descriptor.bin"))
        .compile_protos(&["proto/limit_order_book.proto"], &["proto"])
        .unwrap();
}

#[cfg(not(feature = "grpc"))]
fn main() {}

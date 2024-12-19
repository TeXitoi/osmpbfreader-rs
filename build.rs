extern crate protobuf_codegen;

use std::io::Write;

static MOD_RS: &[u8] = b"
/// Generated from protobuf.
pub mod fileformat;

/// Generated from protobuf.
pub mod osmformat;
";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR")?;

    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir(&out_dir)
        .inputs(["protos/fileformat.proto", "protos/osmformat.proto"])
        .include("protos")
        .run()
        .expect("Codegen failed.");

    std::fs::File::create(out_dir + "/mod.rs")?.write_all(MOD_RS)?;

    Ok(())
}

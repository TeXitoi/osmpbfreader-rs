extern crate protobuf_codegen_pure;

use std::io::Write;

static MOD_RS: &[u8] = b"
/// Generated from protobuf.
pub mod fileformat;

/// Generated from protobuf.
pub mod osmformat;
";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR")?;
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: &out_dir,
        input: &["protos/fileformat.proto", "protos/osmformat.proto"],
        includes: &["protos"],
        customize: Default::default(),
    })?;

    std::fs::File::create(out_dir + "/mod.rs")?.write_all(MOD_RS)?;

    Ok(())
}

extern crate protobuf_codegen_pure;

fn main() {
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src",
        input: &["protos/fileformat.proto", "protos/osmformat.proto"],
        includes: &["protos"],
        customize: Default::default(),
    })
    .expect("protoc");
}

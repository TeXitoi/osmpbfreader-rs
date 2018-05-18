extern crate protoc_rust;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src",
        input: &["protos/fileformat.proto", "protos/osmformat.proto"],
        includes: &["protos"],
    }).expect("protoc");
}

extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &["protos/fileformat.proto", "protos/osmformat.proto"],
        &["protos/"],
    ).unwrap();
}

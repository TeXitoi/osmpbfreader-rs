extern crate osmpbfreader;

fn main() {
    let args = std::os::args();
    let filename = &args[1];
    let path = std::path::Path::new(filename.as_slice());
    let r = std::io::fs::File::open(&path).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::with_reader(r);
    let mut nb = 0u;
    for block in pbf.primitive_blocks().map(|r| r.unwrap()) {
        for _obj in osmpbfreader::blocks::iter(&block) {
            nb += 1;
        }
    }
    println!("{} objects in {}", nb, filename);
}

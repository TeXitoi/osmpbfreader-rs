extern crate osmpbfreader;

fn main() {
    let args = std::os::args();
    let r = std::io::fs::File::open(&std::path::Path::new(&*args[1])).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::with_reader(r);
    let nb = pbf.primitive_blocks().map(|r| r.unwrap()).count();
    println!("File readed, {} primitive blocks.", nb);
}

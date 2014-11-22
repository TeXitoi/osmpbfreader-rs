extern crate osmpbfreader;

fn main() {
    let args = std::os::args();
    let r = std::io::fs::File::open(&std::path::Path::new(&*args[1])).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::with_reader(r);
    loop {
        let header = pbf.read_header().unwrap();
        pbf.read_blob(&header).unwrap();
    }
}

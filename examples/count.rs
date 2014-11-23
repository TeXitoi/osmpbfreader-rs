extern crate osmpbfreader;

fn main() {
    let args = std::os::args();
    let r = std::io::fs::File::open(&std::path::Path::new(&*args[1])).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::with_reader(r);
    let mut nb_blocks = 0u;
    let mut nb_nodes = 0u;
    let mut sum_lon = 0.;
    let mut sum_lat = 0.;
    for block in pbf.primitive_blocks().map(|r| r.unwrap()) {
        nb_blocks += 1;
        for node in osmpbfreader::nodes(&block) {
            nb_nodes += 1;
            sum_lon += node.lon;
            sum_lat += node.lat;
        }
    }
    println!("File readed, {} primitive blocks, {} nodes, mean coord: {}, {}.",
             nb_blocks, nb_nodes, sum_lat / nb_nodes as f64, sum_lon / nb_nodes as f64);
}

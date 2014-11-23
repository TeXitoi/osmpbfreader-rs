extern crate osmpbfreader;

fn count(filter: |&osmpbfreader::Node| -> bool,
         filename: &str) {
    let r = std::io::fs::File::open(&std::path::Path::new(filename)).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::with_reader(r);
    let mut nb_nodes = 0u;
    let mut sum_lon = 0.;
    let mut sum_lat = 0.;
    for block in pbf.primitive_blocks().map(|r| r.unwrap()) {
        for node in osmpbfreader::nodes(&block) {
            if filter(&node) {
                nb_nodes += 1;
                sum_lon += node.lon;
                sum_lat += node.lat;
            }
        }
    }
    println!("File readed, {} nodes, mean coord: {}, {}.",
             nb_nodes, sum_lat / nb_nodes as f64, sum_lon / nb_nodes as f64);
}

fn main() {
    let args = std::os::args();
    match args.as_slice() {
        [_, ref f] => count(|_| true, f.as_slice()),
        [_, ref f, ref key] =>
            count(|n| n.tags.contains_key(key), f.as_slice()),
        [_, ref f, ref key, ref val] =>
            count(|n| n.tags.get(key).map(|v| v == val).unwrap_or(false), f.as_slice()),
        _ => println!("usage: count filename [key [value]]", ),
    };
}

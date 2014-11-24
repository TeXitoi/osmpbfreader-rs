extern crate osmpbfreader;

fn count(filter: |&osmpbfreader::Tags| -> bool, filename: &str) {
    let r = std::io::fs::File::open(&std::path::Path::new(filename)).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::with_reader(r);
    let mut nb_nodes = 0u;
    let mut sum_lon = 0.;
    let mut sum_lat = 0.;
    let mut nb_ways = 0u;
    let mut nb_way_nodes = 0;
    for block in pbf.primitive_blocks().map(|r| r.unwrap()) {
        for node in osmpbfreader::blocks::nodes(&block) {
            if filter(&node.tags) {
                nb_nodes += 1;
                sum_lon += node.lon;
                sum_lat += node.lat;
            }
        }
        for way in osmpbfreader::blocks::ways(&block) {
            if filter(&way.tags) {
                nb_ways += 1;
                nb_way_nodes += way.nodes.len();
            }
        }
    }
    println!("{} nodes, mean coord: {}, {}.",
             nb_nodes, sum_lat / nb_nodes as f64, sum_lon / nb_nodes as f64);
    println!("{} ways, mean |nodes|: {}",
             nb_ways, nb_way_nodes as f64 / nb_ways as f64);
}

fn main() {
    let args = std::os::args();
    match args.as_slice() {
        [_, ref f] => {
            println!("counting objects...");
            count(|_| true, f.as_slice());
        }
        [_, ref f, ref key] => {
            println!("counting objects with \"{}\" in tags...", key);
            count(|tags| tags.contains_key(key), f.as_slice());
        }
        [_, ref f, ref key, ref val] => {
            println!("counting objects with tags[\"{}\"] = \"{}\"...", key, val);
            count(|tags| tags.get(key).map(|v| v == val).unwrap_or(false), f.as_slice());
        }
        _ => println!("usage: count filename [key [value]]", ),
    };
}

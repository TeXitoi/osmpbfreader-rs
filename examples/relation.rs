extern crate osmpbfreader;

fn wanted(obj: &osmpbfreader::OsmObj) -> bool{
    match *obj {
        osmpbfreader::OsmObj::Relation(ref rel) => rel.id == 7444,//id of relation for Paris
        _ => false,
    }
}

fn main() {
    let filename = std::env::args_os().nth(1).unwrap();
    let path = std::path::Path::new(&filename);
    let r = std::fs::File::open(&path).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(r);
    let objects = osmpbfreader::get_objs_and_deps(&mut pbf, wanted).unwrap();
    println!("The relation Paris is composed of {:?} items", objects.len());
}


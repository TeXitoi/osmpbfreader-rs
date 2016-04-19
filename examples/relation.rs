// Copyright (c) 2014-2015 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.
//
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
    for (id, _) in objects{
        println!("{:?}", id);
    }
}


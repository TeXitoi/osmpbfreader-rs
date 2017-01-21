// Copyright (c) 2014-2015 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

#[macro_use]
extern crate log;
extern crate osmpbfreader;

fn count<F: Fn(&osmpbfreader::Tags) -> bool>(filter: F, filename: &std::ffi::OsStr) {
    let r = std::fs::File::open(&std::path::Path::new(filename)).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(r);
    let objs = pbf.get_objs_and_deps(|obj| filter(obj.tags())).unwrap();
    let mut nb_nodes = 0;
    let mut sum_lon = 0.;
    let mut sum_lat = 0.;
    let mut nb_ways = 0;
    let mut nb_way_nodes = 0;
    let mut nb_rels = 0;
    let mut nb_rel_refs = 0;
    for obj in objs.values() {
        info!("{:?}", obj);
        match *obj {
            osmpbfreader::OsmObj::Node(ref node) => {
                nb_nodes += 1;
                sum_lon += node.lon();
                sum_lat += node.lat();
            }
            osmpbfreader::OsmObj::Way(ref way) => {
                nb_ways += 1;
                nb_way_nodes += way.nodes.len();
            }
            osmpbfreader::OsmObj::Relation(ref rel) => {
                nb_rels += 1;
                nb_rel_refs += rel.refs.len();
            }
        }
    }
    println!("{} nodes, mean coord: {}, {}.",
             nb_nodes,
             sum_lat / nb_nodes as f64,
             sum_lon / nb_nodes as f64);
    println!("{} ways, mean |nodes|: {}",
             nb_ways,
             nb_way_nodes as f64 / nb_ways as f64);
    println!("{} relations, mean |references|: {}",
             nb_rels,
             nb_rel_refs as f64 / nb_rels as f64);
}

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    match args.len() {
        3 => {
            let key = args[2].to_str().unwrap();
            println!("counting objects with \"{}\" in tags and their depedencies...",
                     key);
            count(|tags| tags.contains_key(key), &args[1]);
        }
        4 => {
            let key = args[2].to_str().unwrap();
            let val = args[3].to_str().unwrap();
            println!("counting objects with tags[\"{}\"] = \"{}\" and their depedencies...",
                     key,
                     val);
            count(|tags| tags.get(key).map_or(false, |v| *v == val), &args[1]);
        }
        _ => println!("usage: count filename key [value]", ),
    };
}

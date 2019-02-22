// Copyright (c) 2014-2015 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate osmpbfreader;

fn count<F: Fn(&osmpbfreader::Tags) -> bool>(filter: F, filename: &std::ffi::OsStr) {
    let r = std::fs::File::open(&std::path::Path::new(filename)).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(r);
    let mut nb_nodes = 0;
    let mut sum_lon = 0.;
    let mut sum_lat = 0.;
    let mut nb_ways = 0;
    let mut nb_way_nodes = 0;
    let mut nb_rels = 0;
    let mut nb_rel_refs = 0;
    for obj in pbf.par_iter().map(Result::unwrap) {
        if !filter(obj.tags()) {
            continue;
        }
        info!("{:?}", obj);
        match obj {
            osmpbfreader::OsmObj::Node(node) => {
                nb_nodes += 1;
                sum_lon += node.lon();
                sum_lat += node.lat();
            }
            osmpbfreader::OsmObj::Way(way) => {
                nb_ways += 1;
                nb_way_nodes += way.nodes.len();
            }
            osmpbfreader::OsmObj::Relation(rel) => {
                nb_rels += 1;
                nb_rel_refs += rel.refs.len();
            }
        }
    }
    println!(
        "{} nodes, mean coord: {}, {}.",
        nb_nodes,
        sum_lat / nb_nodes as f64,
        sum_lon / nb_nodes as f64
    );
    println!(
        "{} ways, mean |nodes|: {}",
        nb_ways,
        nb_way_nodes as f64 / nb_ways as f64
    );
    println!(
        "{} relations, mean |references|: {}",
        nb_rels,
        nb_rel_refs as f64 / nb_rels as f64
    );
}

fn main() {
    env_logger::init();
    let args: Vec<_> = std::env::args_os().collect();
    match args.len() {
        2 => {
            println!("counting objects...");
            count(|_| true, &args[1]);
        }
        3 => {
            let key = args[2].to_str().unwrap();
            println!("counting objects with \"{}\" in tags...", key);
            count(|tags| tags.contains_key(key), &args[1]);
        }
        4 => {
            let key = args[2].to_str().unwrap();
            let val = args[3].to_str().unwrap();
            println!("counting objects with tags[\"{}\"] = \"{}\"...", key, val);
            count(|tags| tags.contains(key, val), &args[1]);
        }
        _ => println!("usage: count filename [key [value]]",),
    };
}

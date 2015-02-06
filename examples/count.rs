// Copyright (c) 2014 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

#![feature(os, path, io, core, env)]

#[macro_use] extern crate log;
extern crate osmpbfreader;

fn count<F: Fn(&osmpbfreader::Tags) -> bool>(filter: F, filename: &str) {
    let r = std::old_io::fs::File::open(&std::old_path::Path::new(filename)).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::with_reader(r);
    let mut nb_nodes = 0;
    let mut sum_lon = 0.;
    let mut sum_lat = 0.;
    let mut nb_ways = 0;
    let mut nb_way_nodes = 0;
    let mut nb_rels = 0;
    let mut nb_rel_refs = 0;
    for block in pbf.primitive_blocks().map(|r| r.unwrap()) {
        for obj in osmpbfreader::blocks::iter(&block) {
            if !filter(obj.tags()) { continue; }
            info!("{:?}", obj);
            match obj {
                osmpbfreader::OsmObj::Node(node) => {
                    nb_nodes += 1;
                    sum_lon += node.lon;
                    sum_lat += node.lat;
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
    }
    println!("{} nodes, mean coord: {}, {}.",
             nb_nodes, sum_lat / nb_nodes as f64, sum_lon / nb_nodes as f64);
    println!("{} ways, mean |nodes|: {}",
             nb_ways, nb_way_nodes as f64 / nb_ways as f64);
    println!("{} relations, mean |references|: {}",
             nb_rels, nb_rel_refs as f64 / nb_rels as f64);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    match &*args {
        [_, ref f] => {
            println!("counting objects...");
            count(|_| true, f.to_str().unwrap());
        }
        [_, ref f, ref key] => {
            let key = key.to_str().unwrap();
            println!("counting objects with \"{}\" in tags...", key);
            count(|tags| tags.contains_key(key), f.to_str().unwrap());
        }
        [_, ref f, ref key, ref val] => {
            let f = f.to_str().unwrap();
            let key = key.to_str().unwrap();
            let val = val.to_str().unwrap();
            println!("counting objects with tags[\"{}\"] = \"{}\"...", key, val);
            count(|tags| tags.get(key).map(|v| *v == val).unwrap_or(false), f);
        }
        _ => println!("usage: count filename [key [value]]", ),
    };
}

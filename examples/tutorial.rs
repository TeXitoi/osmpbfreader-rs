// Copyright (c) 2014 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

#![feature(os, path, io, core, env)]

extern crate osmpbfreader;

fn main() {
    let filename = std::env::args().nth(1)
        .and_then(|s| s.into_string().ok()).unwrap();
    let path = std::old_path::Path::new(&*filename);
    let r = std::old_io::fs::File::open(&path).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::with_reader(r);
    let mut nb = 0;
    for block in pbf.primitive_blocks().map(|r| r.unwrap()) {
        for _obj in osmpbfreader::blocks::iter(&block) {
            nb += 1;
        }
    }
    println!("{} objects in {}", nb, filename);
}

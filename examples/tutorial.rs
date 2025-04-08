// Copyright (c) 2014-2015 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

extern crate osmpbfreader;

fn main() {
    let filename = std::env::args_os().nth(1).unwrap();
    let path = std::path::Path::new(&filename);
    let r = std::fs::File::open(path).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(r);
    let mut nb = 0;
    for _obj in pbf.iter().map(Result::unwrap) {
        nb += 1;
    }
    println!("{} objects in {:?}", nb, filename);
}

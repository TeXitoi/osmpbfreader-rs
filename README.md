# osmpbfreader-rs [![Build Status](https://travis-ci.org/TeXitoi/osmpbfreader-rs.svg?branch=master)](https://travis-ci.org/TeXitoi/osmpbfreader-rs)

## Presentation

osmpbfreadder-rs is a rust library to read OpenStreetMap PBF files. The goal is to propose something
similar to https://github.com/CanalTP/libosmpbfreader.

## Tutorial

We'll see how to use this library to count the number of objects in an OSM PBF file.

First, install rust:
```
$ curl -s https://static.rust-lang.org/rustup.sh | sudo sh
```

Create a new cargo project:
```
$ cargo new --bin test-osmpbfreader
$ cd test-osmpbfreader/
```

Add dependency to `osmpbfreader-rs` by adding this to your `Cargo.toml`:
```toml
[dependencies.osmpbfreader]
git = "https://github.com/TeXitoi/osmpbfreader-rs"
```
and editing `src/main.rs`:
```rust
extern crate osmpbfreader;

fn main() {
    let args = std::os::args();
    let filename = &args[1];
    let path = std::path::Path::new(filename.as_slice());
    let r = std::io::fs::File::open(&path).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::with_reader(r);
    let mut nb = 0u;
    for block in pbf.primitive_blocks().map(|r| r.unwrap()) {
        for _obj in osmpbfreader::blocks::iter(&block) {
            nb += 1;
        }
    }
    println!("{} objects in {}", nb, filename);
}
```
and build and run:
```
$ cargo build --release
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling mdo v0.1.0 (https://github.com/TeXitoi/rust-mdo?ref=unboxed-closure#ccf4a31e)
   Compiling protobuf v0.0.5 (https://github.com/stepancheg/rust-protobuf.git#0bc890cd)
   Compiling gcc v0.1.0
   Compiling miniz-sys v0.1.0 (https://github.com/alexcrichton/flate2-rs#f9ab9da8)
   Compiling flate2 v0.1.0 (https://github.com/alexcrichton/flate2-rs#f9ab9da8)
   Compiling osmpbfreader v0.0.1 (https://github.com/TeXitoi/osmpbfreader-rs#57ba5182)
   Compiling test-osmpbfreader v0.0.1 (file:///home/gupinot/dev/test-osmpbfreader)
$ ./target/test-osmpbfreader picardie-latest.osm.pbf
9852873 objects in picardie-latest.osm.pbf
```

You can find OSM PBF files at [Geofabrik's free download server](http://download.geofabrik.de/).

## Perfs

Using the [count](examples/count.rs) example compiled in release mode:
```
$ cat /proc/cpuinfo | grep name | head -1
model name	: Intel(R) Core(TM) i7-4702HQ CPU @ 2.20GHz
$ rustc --version
rustc 0.13.0-nightly (fac5a0767 2014-11-26 22:37:06 +0000)
$ ls -sh france-latest.osm.pbf
2,9G france-latest.osm.pbf
$ time ./target/release/count-osm france-latest.osm.pbf admin_level 8
counting objects with tags["admin_level"] = "8"...
51 nodes, mean coord: 46.337052, 2.832865.
105306 ways, mean |nodes|: 75.646221
37229 relations, mean |references|: 8.220446

real	5m30.785s
user	5m18.264s
sys 	0m11.944s
```

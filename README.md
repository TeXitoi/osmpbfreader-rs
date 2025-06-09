# osmpbfreader-rs [![](https://img.shields.io/crates/v/osmpbfreader.svg)](https://crates.io/crates/osmpbfreader) [![](https://docs.rs/osmpbfreader/badge.svg)](https://docs.rs/osmpbfreader)

Read [OpenStreetMap PBF files](http://wiki.openstreetmap.org/wiki/PBF_Format) with
[rust](http://www.rust-lang.org) in an easy and effective way.
The main inspiration of this library is
[libosmpbfreader](https://github.com/CanalTP/libosmpbfreader).

## Documentation

Find it on [Docs.rs](https://docs.rs/osmpbfreader)

## Using this lib

This crate works with Cargo and is on
[crates.io](https://crates.io/crates/osmpbfreader). The package is regularly
updated.

For a complete example, you can see the [examples](examples/).

You can find OSM PBF files at [Geofabrik's free download server](http://download.geofabrik.de/).

## Installation 

```bash
cargo build --release --examples
```

## Performance

Download a PBF file first, then run these commands.
Using the different examples compiled in release mode:

```bash
$ grep CPU /proc/cpuinfo | uniq -c
8 model name	: Intel(R) Core(TM) i7-4702HQ CPU @ 2.20GHz

$ rustc --version
rustc 1.14.0 (e8a012324 2016-12-16)

$ ls -sh france-latest.osm.pbf
3,3G france-latest.osm.pbf

$ time ./target/release/examples/tutorial france-latest.osm.pbf
416483839 objects in "france-latest.osm.pbf"

real	4m24.784s
user	4m18.476s
sys	0m6.164s

$ time ./target/release/examples/count france-latest.osm.pbf admin_level 8
counting objects with tags["admin_level"] = "8"...
53 nodes, mean coord: 46.25862766415095, 2.9082348867924517.
108190 ways, mean |nodes|: 72.09304926518162
35984 relations, mean |references|: 8.705369052912406

real	1m10.117s
user	8m16.164s
sys	0m23.120s

$ time ./target/release/examples/count_with_deps france-latest.osm.pbf admin_level 8
counting objects with tags["admin_level"] = "8" and their depedencies...
9497221 nodes, mean coord: 46.69071931974348, 2.2632424769587915.
136950 ways, mean |nodes|: 70.35282949981745
36408 relations, mean |references|: 8.771121731487586

real	5m9.814s
user	33m52.820s
sys	0m28.624s
```

## License

This work is free. You can redistribute it and/or modify it under the
terms of the Do What The Fuck You Want To Public License, Version 2,
as published by Sam Hocevar. See the COPYING file for more details.

Note that `protos/fileformat.proto` and `protos/osmformat.proto` come from
[OSM-binary](https://github.com/scrosby/OSM-binary) under the MIT.

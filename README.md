# osmpbfreader-rs [![Build status](https://travis-ci.org/TeXitoi/osmpbfreader-rs.svg?branch=master)](https://travis-ci.org/TeXitoi/osmpbfreader-rs) [![](https://ci.appveyor.com/api/projects/status/0po5osv2wjb6v07m/branch/master?svg=true)](https://ci.appveyor.com/project/TeXitoi/osmpbfreader-rs/branch/master) [![](https://img.shields.io/crates/v/osmpbfreader.svg)](https://crates.io/crates/osmpbfreader) [![](https://docs.rs/osmpbfreader/badge.svg)](https://docs.rs/osmpbfreader)

Read [OpenStreetMap PBF
files](http://wiki.openstreetmap.org/wiki/PBF_Format) with
[rust](http://www.rust-lang.org) in an easy and effective way.
The main inspiration of this library is
[libosmpbfreader](https://github.com/CanalTP/libosmpbfreader).

## Documentation

Find it on [Docs.rs](https://docs.rs/osmpbfreader)

## Using this lib

This crate works with Cargo and is on
[crates.io](https://crates.io/crates/osmpbfreader). The package is regularly
updated.

For complete example, you can see the [examples](examples/).

You can find OSM PBF files at [Geofabrik's free download server](http://download.geofabrik.de/).

## Performances

Using the different examples compiled in release mode:
```
$ grep CPU /proc/cpuinfo | uniq -c
      4 model name	: Intel(R) Core(TM) i5-4200U CPU @ 1.60GHz
$ rustc --version
rustc 1.14.0 (e8a012324 2016-12-16)
$ ls -sh france-latest.osm.pbf
3,3G france-latest.osm.pbf
$ time ./target/release/examples/tutorial france-latest.osm.pbf
415229844 objects in "france-latest.osm.pbf"

real	4m51.018s
user	4m43.500s
sys	0m5.788s
$ time ./target/release/examples/count france-latest.osm.pbf admin_level 8
counting objects with tags["admin_level"] = "8"...
54 nodes, mean coord: 46.296170755555565, 2.8982805611111107.
107969 ways, mean |nodes|: 72.17152145523252
35989 relations, mean |references|: 8.701408763788935

real	3m3.350s
user	11m26.852s
sys	0m29.964s
$ time ./target/release/examples/count_with_deps france-latest.osm.pbf admin_level 8
counting objects with tags["admin_level"] = "8" and their depedencies...
9517850 nodes, mean coord: 46.694558024657404, 2.2541158816049762.
138463 ways, mean |nodes|: 69.74443714205239
36790 relations, mean |references|: 8.920549062245176

real	16m58.378s
user	64m31.564s
sys	0m41.624s
```

## License

This work is free. You can redistribute it and/or modify it under the
terms of the Do What The Fuck You Want To Public License, Version 2,
as published by Sam Hocevar. See the COPYING file for more details.

Note that `src/fileformat.proto` and `src/osmformat.proto` come from
[OSM-binary](https://github.com/scrosby/OSM-binary) under the LGPLv3.

## TODO

TODO list:
 - document until `#![deny(missing_docs)]` can be added;
 - read pbf header to check that we support all needed features;
 - do something better than `borrowed_iter`.

# osmpbfreader-rs [![Build status](https://travis-ci.org/TeXitoi/osmpbfreader-rs.svg?branch=master)](https://travis-ci.org/TeXitoi/osmpbfreader-rs) [![](http://meritbadge.herokuapp.com/osmpbfreader)](https://crates.io/crates/osmpbfreader)



## Presentation

Read [OpenStreetMap PBF
files](http://wiki.openstreetmap.org/wiki/PBF_Format) with
[rust](http://www.rust-lang.org).  The main inspiration of this
library is
[libosmpbfreader](https://github.com/CanalTP/libosmpbfreader).

## Documentation

[http://texitoi.github.io/osmpbfreader-rs/](http://texitoi.github.io/osmpbfreader-rs/)

## Using this lib

This crate works with Cargo and is on
[crates.io](https://crates.io/crates/osmpbfreader). The package is regularly
updated.  Add it to your `Cargo.toml` like so:

```toml
[dependencies]
osmpbfreader = "0.3"
```

For complete example, you can see the [examples](examples/).

You can find OSM PBF files at [Geofabrik's free download server](http://download.geofabrik.de/).

## Performances

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

## License

This work is free. You can redistribute it and/or modify it under the
terms of the Do What The Fuck You Want To Public License, Version 2,
as published by Sam Hocevar. See the COPYING file for more details.

Note that `src/fileformat.proto` and `src/osmformat.proto` come from
[OSM-binary](https://github.com/scrosby/OSM-binary) under the LGPLv3.

## TODO

TODO list:
 - document until `#![deny(missing_docs)]` can be added;
 - read header to check that we support all needed features.

# osmpbfreader-rs [![Build Status](https://travis-ci.org/TeXitoi/osmpbfreader-rs.svg?branch=master)](https://travis-ci.org/TeXitoi/osmpbfreader-rs)

## Presentation

Read OpenStreetMap PBF files in rust. The goal is to propose something
similar to https://github.com/CanalTP/libosmpbfreader for rust.

## Example

See [here](examples/count.rs) for an almost complete example.

## Perfs

Using the `count` example compiled in release mode:
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

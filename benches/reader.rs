use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use std::fs;
use std::io::Cursor;

const PBF: &str = "tests/clipperton.osm.pbf";

#[derive(Debug, Default, PartialEq)]
struct Stats {
    num_nodes: i64,
    num_ways: i64,
    num_relations: i64,
    sum_nodes_ids: i64,
    sum_ways_ids: i64,
    sum_relations_ids: i64,
}

const PBF_STATS_ALL: Stats = Stats {
    num_nodes: 3097,
    num_ways: 151,
    num_relations: 17,
    sum_nodes_ids: 22498900142500,
    sum_ways_ids: 155207265285,
    sum_relations_ids: 112821863,
};

const PBF_STATS_HIGHWAY: Stats = Stats {
    num_nodes: 9,
    num_ways: 3,
    num_relations: 0,
    sum_nodes_ids: 114394331047,
    sum_ways_ids: 4118264166,
    sum_relations_ids: 0,
};

fn load_bytes() -> Vec<u8> {
    fs::read(PBF).expect("read PBF fixture")
}

fn accumulate_stats(obj: &osmpbfreader::OsmObj, stats: &mut Stats) {
    match obj {
        osmpbfreader::OsmObj::Node(n) => {
            stats.num_nodes += 1;
            stats.sum_nodes_ids += n.id.0;
        }
        osmpbfreader::OsmObj::Way(w) => {
            stats.num_ways += 1;
            stats.sum_ways_ids += w.id.0;
        }
        osmpbfreader::OsmObj::Relation(r) => {
            stats.num_relations += 1;
            stats.sum_relations_ids += r.id.0;
        }
    }
}

fn bench_iter(c: &mut Criterion) {
    let bytes = load_bytes();
    c.bench_function("read_pbf_iter", |b| {
        b.iter(|| {
            let mut pbf = osmpbfreader::OsmPbfReader::new(Cursor::new(black_box(&bytes)));
            let mut stats = Stats::default();
            for obj in pbf.iter() {
                let obj = obj.unwrap();
                accumulate_stats(&obj, &mut stats);
            }
            assert_eq!(stats, PBF_STATS_ALL);
            black_box(stats)
        })
    });
}

fn bench_par_iter(c: &mut Criterion) {
    let bytes = load_bytes();
    c.bench_function("read_pbf_par_iter", |b| {
        b.iter(|| {
            let mut pbf = osmpbfreader::OsmPbfReader::new(Cursor::new(black_box(&bytes)));
            let mut stats = Stats::default();
            for obj in pbf.par_iter() {
                let obj = obj.unwrap();
                accumulate_stats(&obj, &mut stats);
            }
            assert_eq!(stats, PBF_STATS_ALL);
            black_box(stats)
        })
    });
}

fn bench_highway(c: &mut Criterion) {
    let bytes = load_bytes();
    c.bench_function("read_pbf_highway", |b| {
        b.iter(|| {
            let mut pbf = osmpbfreader::OsmPbfReader::new(Cursor::new(black_box(&bytes)));
            let mut stats = Stats::default();
            let objs = pbf
                .get_objs_and_deps(|obj| obj.is_way() && obj.tags().contains_key("highway"))
                .unwrap();
            for (_id, obj) in &objs {
                accumulate_stats(obj, &mut stats);
            }
            assert_eq!(stats, PBF_STATS_HIGHWAY);
            black_box(stats)
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_iter, bench_par_iter, bench_highway
}
criterion_main!(benches);

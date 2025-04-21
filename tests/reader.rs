use osmpbfreader;
use std::fs::File;

#[derive(Debug, Default, PartialEq)]
struct Stats {
    num_nodes: i64,
    num_ways: i64,
    num_relations: i64,
    sum_nodes_ids: i64,
    sum_ways_ids: i64,
    sum_relations_ids: i64,
}

const PBF: &str = "tests/clipperton.osm.pbf";
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

#[test]
fn read_pbf() {
    let file = File::open(PBF).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(file);
    let mut stats = Stats::default();

    for obj in pbf.iter() {
        let obj = obj.unwrap();
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
    assert_eq!(stats, PBF_STATS_ALL);
}

#[test]
fn read_pbf_par_iter() {
    let file = File::open(PBF).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(file);
    let mut stats = Stats::default();

    for obj in pbf.par_iter() {
        let obj = obj.unwrap();
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
    assert_eq!(stats, PBF_STATS_ALL);
}

#[test]
fn read_pbf_highway() {
    let file = File::open(PBF).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(file);
    let mut stats = Stats::default();
    let mut sum_ids = 0;

    let objs = pbf
        .get_objs_and_deps(|obj| obj.is_way() && obj.tags().contains_key("highway"))
        .unwrap();
    for (id, obj) in &objs {
        sum_ids += id.inner_id();
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
    assert_eq!(stats, PBF_STATS_HIGHWAY);
    let exp_sum_ids = PBF_STATS_HIGHWAY.sum_nodes_ids
        + PBF_STATS_HIGHWAY.sum_ways_ids
        + PBF_STATS_HIGHWAY.sum_relations_ids;
    assert_eq!(sum_ids, exp_sum_ids);
}

#[cfg(feature = "full-metadata")]
#[test]
fn read_pbf_full_metadata() {
    let file = File::open(PBF).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(file);
    let mut stats = Stats::default();
    let mut node_found = false;
    let mut way_found = false;
    let mut relation_found = false;

    for obj in pbf.iter_with_info() {
        let obj = obj.unwrap();
        match obj.obj {
            osmpbfreader::OsmObj::Node(n) => {
                stats.num_nodes += 1;
                stats.sum_nodes_ids += n.id.0;

                let info = obj.info.unwrap();
                assert_ne!(info.version, Some(0));
                if n.id.0 == 1724610081 {
                    node_found = true;
                    // Note that the tested pbf file has incomplete information, which is why we
                    // uid/user/changeset are compared to 0.
                    assert_eq!(info.version, Some(1));
                    assert_eq!(info.timestamp, Some(1334956915));
                    assert_eq!(info.uid, Some(0));
                    assert_eq!(info.user.unwrap(), "");
                    assert_eq!(info.changeset, Some(0));
                }
            }
            osmpbfreader::OsmObj::Way(w) => {
                stats.num_ways += 1;
                stats.sum_ways_ids += w.id.0;

                let info = obj.info.unwrap();
                assert_ne!(info.version, Some(0));
                if w.id.0 == 160415456 {
                    way_found = true;
                    assert_eq!(info.version, Some(46));
                    assert_eq!(info.timestamp, Some(1743278237));
                    assert_eq!(info.uid, Some(0));
                    assert_eq!(info.user.unwrap(), "");
                    assert_eq!(info.changeset, Some(0));
                }
            }
            osmpbfreader::OsmObj::Relation(r) => {
                stats.num_relations += 1;
                stats.sum_relations_ids += r.id.0;

                let info = obj.info.unwrap();
                assert_ne!(info.version, Some(0));
                if r.id.0 == 957715 {
                    relation_found = true;
                    assert_eq!(info.version, Some(7));
                    assert_eq!(info.timestamp, Some(1485088421));
                    assert_eq!(info.uid, Some(0));
                    assert_eq!(info.user.unwrap(), "");
                    assert_eq!(info.changeset, Some(0));
                }
            }
        }
    }
    assert_eq!(stats, PBF_STATS_ALL);

    assert_eq!(node_found, true);
    assert_eq!(way_found, true);
    assert_eq!(relation_found, true);
}

#[cfg(feature = "full-metadata")]
#[test]
fn read_pbf_par_full_metadata() {
    let file = File::open(PBF).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(file);
    let mut stats = Stats::default();

    for obj in pbf.par_iter_with_info() {
        let obj = obj.unwrap();
        match obj.obj {
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
    assert_eq!(stats, PBF_STATS_ALL);
}

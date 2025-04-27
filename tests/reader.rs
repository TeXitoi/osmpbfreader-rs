use osmpbfreader::objects::{Node, NodeId, OsmId, Ref, Relation, RelationId, Tags, Way, WayId};
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

fn get_expected_objects() -> (Vec<Node>, Vec<Way>, Vec<Relation>) {
    let nodes: Vec<Node> = vec![
        Node {
            id: NodeId(1724610081),
            tags: Tags::new(),
            decimicro_lat: (10.2871538 * 1e7) as i32,
            decimicro_lon: (-109.2143679 * 1e7) as i32,
        },
        Node {
            id: NodeId(1724610082),
            tags: Tags::new(),
            decimicro_lat: (10.2872043 * 1e7) as i32,
            decimicro_lon: (-109.2157633 * 1e7) as i32,
        },
        Node {
            id: NodeId(2710971331),
            tags: [
                ("historic".into(), "wreck".into()),
                ("seamark:type".into(), "wreck".into()),
            ]
            .into_iter()
            .collect(),
            decimicro_lat: (10.2873955 * 1e7) as i32,
            decimicro_lon: (-109.2123449 * 1e7) as i32,
        },
        Node {
            id: NodeId(12710481252),
            tags: Tags::new(),
            decimicro_lat: (10.2925456 * 1e7) as i32,
            decimicro_lon: (-109.2212479 * 1e7) as i32,
        },
    ];
    let ways: Vec<Way> = vec![
        Way {
            id: WayId(633118718),
            tags: [
                ("natural".into(), "wood".into()),
                ("trees".into(), "coconut_palms".into()),
            ]
            .into_iter()
            .collect(),
            nodes: vec![
                NodeId(5976744399),
                NodeId(5976744400),
                NodeId(5976744401),
                NodeId(5976744402),
                NodeId(5976744399),
            ],
        },
        Way {
            id: WayId(1253640658),
            tags: [("natural".into(), "wood".into())].into_iter().collect(),
            nodes: vec![
                NodeId(11653090668),
                NodeId(11653090669),
                NodeId(11653090670),
                NodeId(11653090668),
            ],
        },
        Way {
            id: WayId(1372754726),
            tags: {
                let mut tags = Tags::new();
                tags.insert("aeroway".into(), "taxiway".into());
                tags
            },
            nodes: vec![
                NodeId(12710481236),
                NodeId(12710481237),
                NodeId(12710481238),
                NodeId(12710481239),
            ],
        },
    ];
    let relations: Vec<Relation> = vec![
        Relation {
            id: RelationId(5883537),
            tags: [
                ("natural".into(), "beach".into()),
                ("type".into(), "multipolygon".into()),
            ]
            .into_iter()
            .collect(),
            refs: vec![
                Ref {
                    member: OsmId::Way(WayId(392464137)),
                    role: "inner".into(),
                },
                Ref {
                    member: OsmId::Way(WayId(160415456)),
                    role: "outer".into(),
                },
            ],
        },
        Relation {
            id: RelationId(8803113),
            tags: [
                ("place".into(), "islet".into()),
                ("type".into(), "multipolygon".into()),
            ]
            .into_iter()
            .collect(),
            refs: vec![Ref {
                member: OsmId::Way(WayId(633118720)),
                role: "outer".into(),
            }],
        },
    ];

    (nodes, ways, relations)
}

fn get_expected_objects_highway() -> (Vec<Node>, Vec<Way>, Vec<Relation>) {
    let nodes: Vec<Node> = vec![
        Node {
            id: NodeId(12710481223),
            tags: Tags::new(),
            decimicro_lat: (10.2901053 * 1e7) as i32,
            decimicro_lon: (-109.2103612 * 1e7) as i32,
        },
        Node {
            id: NodeId(12710481226),
            tags: Tags::new(),
            decimicro_lat: (10.2897482 * 1e7) as i32,
            decimicro_lon: (-109.2101919 * 1e7) as i32,
        },
    ];
    let ways: Vec<Way> = vec![Way {
        id: WayId(1372754721),
        tags: {
            let mut tags = Tags::new();
            tags.insert("highway".into(), "residential".into());
            tags.insert("name".into(), "Clipperton St".into());
            tags
        },
        nodes: vec![
            NodeId(12710481223),
            NodeId(12710481226),
            NodeId(12710481233),
            NodeId(12710481224),
            NodeId(12710481225),
        ],
    }];
    let relations: Vec<Relation> = vec![];

    (nodes, ways, relations)
}

#[test]
fn read_pbf() {
    let file = File::open(PBF).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(file);
    let mut stats = Stats::default();

    let (exp_nodes, exp_ways, exp_relations) = get_expected_objects();
    let mut num_found_nodes = 0;
    let mut num_found_ways = 0;
    let mut num_found_relations = 0;

    for obj in pbf.iter() {
        let obj = obj.unwrap();
        match obj {
            osmpbfreader::OsmObj::Node(n) => {
                stats.num_nodes += 1;
                stats.sum_nodes_ids += n.id.0;
                for exp_node in &exp_nodes {
                    if exp_node.id.0 == n.id.0 {
                        num_found_nodes += 1;
                        assert_eq!(&n, exp_node);
                    }
                }
            }
            osmpbfreader::OsmObj::Way(w) => {
                stats.num_ways += 1;
                stats.sum_ways_ids += w.id.0;
                for exp_way in &exp_ways {
                    if exp_way.id.0 == w.id.0 {
                        num_found_ways += 1;
                        assert_eq!(&w, exp_way);
                    }
                }
            }
            osmpbfreader::OsmObj::Relation(r) => {
                stats.num_relations += 1;
                stats.sum_relations_ids += r.id.0;
                for exp_relation in &exp_relations {
                    if exp_relation.id.0 == r.id.0 {
                        num_found_relations += 1;
                        assert_eq!(&r, exp_relation);
                    }
                }
            }
        }
    }
    assert_eq!(stats, PBF_STATS_ALL);
    assert_eq!(num_found_nodes, exp_nodes.len());
    assert_eq!(num_found_ways, exp_ways.len());
    assert_eq!(num_found_relations, exp_relations.len());
}

#[test]
fn read_pbf_par_iter() {
    let file = File::open(PBF).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(file);
    let mut stats = Stats::default();

    let (exp_nodes, exp_ways, exp_relations) = get_expected_objects();
    let mut num_found_nodes = 0;
    let mut num_found_ways = 0;
    let mut num_found_relations = 0;

    for obj in pbf.par_iter() {
        let obj = obj.unwrap();
        match obj {
            osmpbfreader::OsmObj::Node(n) => {
                stats.num_nodes += 1;
                stats.sum_nodes_ids += n.id.0;
                for exp_node in &exp_nodes {
                    if exp_node.id.0 == n.id.0 {
                        num_found_nodes += 1;
                        assert_eq!(&n, exp_node);
                    }
                }
            }
            osmpbfreader::OsmObj::Way(w) => {
                stats.num_ways += 1;
                stats.sum_ways_ids += w.id.0;
                for exp_way in &exp_ways {
                    if exp_way.id.0 == w.id.0 {
                        num_found_ways += 1;
                        assert_eq!(&w, exp_way);
                    }
                }
            }
            osmpbfreader::OsmObj::Relation(r) => {
                stats.num_relations += 1;
                stats.sum_relations_ids += r.id.0;
                for exp_relation in &exp_relations {
                    if exp_relation.id.0 == r.id.0 {
                        num_found_relations += 1;
                        assert_eq!(&r, exp_relation);
                    }
                }
            }
        }
    }
    assert_eq!(stats, PBF_STATS_ALL);
    assert_eq!(num_found_nodes, exp_nodes.len());
    assert_eq!(num_found_ways, exp_ways.len());
    assert_eq!(num_found_relations, exp_relations.len());
}

#[test]
fn read_pbf_highway() {
    let file = File::open(PBF).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(file);
    let mut stats = Stats::default();
    let mut sum_ids = 0;

    let (exp_nodes, exp_ways, exp_relations) = get_expected_objects_highway();
    let mut num_found_nodes = 0;
    let mut num_found_ways = 0;
    let mut num_found_relations = 0;

    let objs = pbf
        .get_objs_and_deps(|obj| obj.is_way() && obj.tags().contains_key("highway"))
        .unwrap();
    for (id, obj) in &objs {
        sum_ids += id.inner_id();
        match obj {
            osmpbfreader::OsmObj::Node(n) => {
                stats.num_nodes += 1;
                stats.sum_nodes_ids += n.id.0;
                for exp_node in &exp_nodes {
                    if exp_node.id.0 == n.id.0 {
                        num_found_nodes += 1;
                        assert_eq!(n, exp_node);
                    }
                }
            }
            osmpbfreader::OsmObj::Way(w) => {
                stats.num_ways += 1;
                stats.sum_ways_ids += w.id.0;
                assert!(w.tags.contains_key("highway"));
                for exp_way in &exp_ways {
                    if exp_way.id.0 == w.id.0 {
                        num_found_ways += 1;
                        assert_eq!(w, exp_way);
                    }
                }
            }
            osmpbfreader::OsmObj::Relation(r) => {
                stats.num_relations += 1;
                stats.sum_relations_ids += r.id.0;
                assert!(r.tags.contains_key("highway"));
                for exp_relation in &exp_relations {
                    if exp_relation.id.0 == r.id.0 {
                        num_found_relations += 1;
                        assert_eq!(r, exp_relation);
                    }
                }
            }
        }
    }
    assert_eq!(stats, PBF_STATS_HIGHWAY);
    let exp_sum_ids = PBF_STATS_HIGHWAY.sum_nodes_ids
        + PBF_STATS_HIGHWAY.sum_ways_ids
        + PBF_STATS_HIGHWAY.sum_relations_ids;
    assert_eq!(sum_ids, exp_sum_ids);
    assert_eq!(num_found_nodes, exp_nodes.len());
    assert_eq!(num_found_ways, exp_ways.len());
    assert_eq!(num_found_relations, exp_relations.len());
}

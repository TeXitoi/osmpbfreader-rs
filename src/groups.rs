// Copyright (c) 2014-2015 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

use crate::objects::*;
use crate::osmformat;
use crate::osmformat::{PrimitiveBlock, PrimitiveGroup};
use pub_iterator_type::pub_iterator_type;
use smartstring::alias::String;
use std::borrow::Cow;
use std::convert::From;
use std::iter::Chain;
use std::iter::Map;
use std::slice;

pub_iterator_type! {
    #[doc="Iterator on the `OsmObj` of a `PrimitiveGroup`."]
    OsmObjs['a] = Chain<Chain<Map<Nodes<'a>, fn(Node) -> OsmObj>,
                              Map<Ways<'a>, fn(Way) -> OsmObj>>,
                        Map<Relations<'a>, fn(Relation) -> OsmObj>>
}

pub fn iter<'a>(g: &'a PrimitiveGroup, b: &'a PrimitiveBlock) -> OsmObjs<'a> {
    let iter = nodes(g, b)
        .map(From::from as fn(Node) -> OsmObj)
        .chain(ways(g, b).map(From::from as fn(Way) -> OsmObj))
        .chain(relations(g, b).map(From::from as fn(Relation) -> OsmObj));
    OsmObjs(iter)
}

pub_iterator_type! {
    #[doc="Iterator on the `Node` of a `PrimitiveGroup`."]
    Nodes['a] = Chain<SimpleNodes<'a>, DenseNodes<'a>>
}

pub fn nodes<'a>(g: &'a PrimitiveGroup, b: &'a PrimitiveBlock) -> Nodes<'a> {
    Nodes(simple_nodes(g, b).chain(dense_nodes(g, b)))
}

pub fn simple_nodes<'a>(group: &'a PrimitiveGroup, block: &'a PrimitiveBlock) -> SimpleNodes<'a> {
    SimpleNodes {
        iter: group.nodes.iter(),
        block,
    }
}

pub struct SimpleNodes<'a> {
    iter: slice::Iter<'a, osmformat::Node>,
    block: &'a PrimitiveBlock,
}

impl<'a> Iterator for SimpleNodes<'a> {
    type Item = Node;
    fn next(&mut self) -> Option<Node> {
        self.iter.next().map(|n| Node {
            id: NodeId(n.id()),
            decimicro_lat: make_lat(n.lat(), self.block),
            decimicro_lon: make_lon(n.lon(), self.block),
            tags: make_tags(&n.keys, &n.vals, self.block),
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub fn dense_nodes<'a>(group: &'a PrimitiveGroup, block: &'a PrimitiveBlock) -> DenseNodes<'a> {
    let dense = &group.dense;
    DenseNodes {
        block,
        dids: dense.id.iter(),
        dlats: dense.lat.iter(),
        dlons: dense.lon.iter(),
        keys_vals: dense.keys_vals.iter(),
        cur_id: 0,
        cur_lat: 0,
        cur_lon: 0,
    }
}

pub struct DenseNodes<'a> {
    block: &'a PrimitiveBlock,
    dids: slice::Iter<'a, i64>,
    dlats: slice::Iter<'a, i64>,
    dlons: slice::Iter<'a, i64>,
    keys_vals: slice::Iter<'a, i32>,
    cur_id: i64,
    cur_lat: i64,
    cur_lon: i64,
}

impl<'a> Iterator for DenseNodes<'a> {
    type Item = Node;
    fn next(&mut self) -> Option<Node> {
        match (self.dids.next(), self.dlats.next(), self.dlons.next()) {
            (Some(&did), Some(&dlat), Some(&dlon)) => {
                self.cur_id += did;
                self.cur_lat += dlat;
                self.cur_lon += dlon;
            }
            _ => return None,
        }
        let mut tags = Tags::new();
        loop {
            let k = match self.keys_vals.next() {
                None | Some(&0) => break,
                Some(k) => make_string(*k as usize, self.block),
            };
            let v = match self.keys_vals.next() {
                None => break,
                Some(v) => make_string(*v as usize, self.block),
            };
            tags.insert(k, v);
        }
        tags.shrink_to_fit();
        Some(Node {
            id: NodeId(self.cur_id),
            decimicro_lat: make_lat(self.cur_lat, self.block),
            decimicro_lon: make_lon(self.cur_lon, self.block),
            tags,
        })
    }
}

pub fn ways<'a>(group: &'a PrimitiveGroup, block: &'a PrimitiveBlock) -> Ways<'a> {
    Ways {
        iter: group.ways.iter(),
        block,
    }
}

pub struct Ways<'a> {
    iter: slice::Iter<'a, osmformat::Way>,
    block: &'a PrimitiveBlock,
}

impl<'a> Iterator for Ways<'a> {
    type Item = Way;
    fn next(&mut self) -> Option<Way> {
        self.iter.next().map(|w| {
            let mut n = 0;
            let nodes = w
                .refs
                .iter()
                .map(|&dn| {
                    n += dn;
                    NodeId(n)
                })
                .collect();
            Way {
                id: WayId(w.id()),
                nodes,
                tags: make_tags(&w.keys, &w.vals, self.block),
            }
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub fn relations<'a>(group: &'a PrimitiveGroup, block: &'a PrimitiveBlock) -> Relations<'a> {
    Relations {
        iter: group.relations.iter(),
        block,
    }
}
pub struct Relations<'a> {
    iter: slice::Iter<'a, osmformat::Relation>,
    block: &'a PrimitiveBlock,
}

impl<'a> Iterator for Relations<'a> {
    type Item = Relation;
    fn next(&mut self) -> Option<Relation> {
        use osmformat::relation::MemberType::{NODE, RELATION, WAY};
        self.iter.next().map(|rel| {
            let mut m = 0;
            let refs = rel
                .memids
                .iter()
                .zip(rel.types.iter())
                .zip(rel.roles_sid.iter())
                .flat_map(|((&dm, &t), &role)| {
                    m += dm;
                    t.enum_value().map(|t| Ref {
                        member: match t {
                            NODE => NodeId(m).into(),
                            WAY => WayId(m).into(),
                            RELATION => RelationId(m).into(),
                        },
                        role: make_string(role as usize, self.block),
                    })
                })
                .collect();
            Relation {
                id: RelationId(rel.id()),
                refs,
                tags: make_tags(&rel.keys, &rel.vals, self.block),
            }
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

fn make_string(k: usize, block: &osmformat::PrimitiveBlock) -> String {
    let cow = std::string::String::from_utf8_lossy(&*block.stringtable.s[k]);
    match cow {
        Cow::Borrowed(s) => String::from(s),
        Cow::Owned(s) => String::from(s),
    }
}

fn make_lat(c: i64, b: &osmformat::PrimitiveBlock) -> i32 {
    let granularity = b.granularity() as i64;
    ((b.lat_offset() + granularity * c) / 100) as i32
}

fn make_lon(c: i64, b: &osmformat::PrimitiveBlock) -> i32 {
    let granularity = b.granularity() as i64;
    ((b.lon_offset() + granularity * c) / 100) as i32
}

fn make_tags(keys: &[u32], vals: &[u32], b: &PrimitiveBlock) -> Tags {
    let mut tags: Tags = keys
        .iter()
        .zip(vals.iter())
        .map(|(&k, &v)| (make_string(k as usize, b), make_string(v as usize, b)))
        .collect();
    tags.shrink_to_fit();
    tags
}

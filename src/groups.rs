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
use std::marker::PhantomData;
#[cfg(feature = "full-metadata")]
use std::num::NonZero;
use std::slice;

pub_iterator_type! {
    #[doc="Iterator on the `OsmObj` of a `PrimitiveGroup`."]
    OsmObjs['a] = Chain<Chain<Map<Nodes<'a, Node>, fn(Node) -> OsmObj>,
                              Map<Ways<'a, Way>, fn(Way) -> OsmObj>>,
                        Map<Relations<'a, Relation>, fn(Relation) -> OsmObj>>
}

pub fn iter<'a>(g: &'a PrimitiveGroup, b: &'a PrimitiveBlock) -> OsmObjs<'a> {
    let iter = nodes(g, b)
        .map(From::from as fn(Node) -> OsmObj)
        .chain(ways_generic::<Way>(g, b).map(From::from as fn(Way) -> OsmObj))
        .chain(relations(g, b).map(From::from as fn(Relation) -> OsmObj));
    OsmObjs(iter)
}

#[cfg(feature = "full-metadata")]
pub_iterator_type! {
    #[doc="Iterator on the `OsmObjInfo` of a `PrimitiveGroup`."]
    OsmObjInfos['a] = Chain<Chain<Map<Nodes<'a, NodeInfo>, fn(NodeInfo) -> OsmObjInfo>,
                              Map<Ways<'a, WayInfo>, fn(WayInfo) -> OsmObjInfo>>,
                        Map<Relations<'a, RelationInfo>, fn(RelationInfo) -> OsmObjInfo>>
}

#[cfg(feature = "full-metadata")]
pub fn iter_with_metadata<'a>(g: &'a PrimitiveGroup, b: &'a PrimitiveBlock) -> OsmObjInfos<'a> {
    let iter = nodes_generic::<NodeInfo>(g, b)
        .map(From::from as fn(NodeInfo) -> OsmObjInfo)
        .chain(ways_generic::<WayInfo>(g, b).map(From::from as fn(WayInfo) -> OsmObjInfo))
        .chain(
            relations_generic::<RelationInfo>(g, b)
                .map(From::from as fn(RelationInfo) -> OsmObjInfo),
        );
    OsmObjInfos(iter)
}

pub_iterator_type! {
    #[doc="Iterator on the `Node` of a `PrimitiveGroup`."]
    Nodes['a, N] = Chain<SimpleNodes<'a, N>, DenseNodes<'a, N>> where N: NodeT
}

pub fn nodes<'a>(g: &'a PrimitiveGroup, b: &'a PrimitiveBlock) -> Nodes<'a, Node> {
    nodes_generic::<Node>(g, b)
}

pub fn nodes_generic<'a, N: NodeT>(g: &'a PrimitiveGroup, b: &'a PrimitiveBlock) -> Nodes<'a, N> {
    Nodes::<N>(simple_nodes_generic::<N>(g, b).chain(dense_nodes_generic::<N>(g, b)))
}

pub fn simple_nodes<'a>(group: &'a PrimitiveGroup, block: &'a PrimitiveBlock) -> SimpleNodes<'a> {
    simple_nodes_generic::<Node>(group, block)
}

pub fn simple_nodes_generic<'a, N: NodeT>(
    group: &'a PrimitiveGroup,
    block: &'a PrimitiveBlock,
) -> SimpleNodes<'a, N> {
    SimpleNodes::<N> {
        iter: group.nodes.iter(),
        block,
        phantom: PhantomData,
    }
}

pub struct SimpleNodes<'a, N: NodeT = Node> {
    iter: slice::Iter<'a, osmformat::Node>,
    block: &'a PrimitiveBlock,
    phantom: PhantomData<N>,
}

impl<N: NodeT> Iterator for SimpleNodes<'_, N> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
        self.iter.next().map(|n| {
            N::get_node(
                Node {
                    id: NodeId(n.id()),
                    decimicro_lat: make_lat(n.lat(), self.block),
                    decimicro_lon: make_lon(n.lon(), self.block),
                    tags: make_tags(&n.keys, &n.vals, self.block),
                },
                #[cfg(feature = "full-metadata")]
                make_info(&n.info, self.block),
                #[cfg(not(feature = "full-metadata"))]
                None,
            )
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub fn dense_nodes<'a>(group: &'a PrimitiveGroup, block: &'a PrimitiveBlock) -> DenseNodes<'a> {
    dense_nodes_generic::<Node>(group, block)
}

pub fn dense_nodes_generic<'a, N: NodeT>(
    group: &'a PrimitiveGroup,
    block: &'a PrimitiveBlock,
) -> DenseNodes<'a, N> {
    let dense = &group.dense;
    DenseNodes::<N> {
        block,
        dids: dense.id.iter(),
        dlats: dense.lat.iter(),
        dlons: dense.lon.iter(),
        keys_vals: dense.keys_vals.iter(),
        cur_id: 0,
        cur_lat: 0,
        cur_lon: 0,
        #[cfg(feature = "full-metadata")]
        denseinfo: DenseInfo {
            block,
            version: dense.denseinfo.version.iter(),
            dtimestamp: dense.denseinfo.timestamp.iter(),
            dchangeset: dense.denseinfo.changeset.iter(),
            duid: dense.denseinfo.uid.iter(),
            duser_sid: dense.denseinfo.user_sid.iter(),
            visible: dense.denseinfo.visible.iter(),
            cur_timestamp: 0,
            cur_changeset: 0,
            cur_uid: 0,
            cur_user_sid: 0,
        },
        phantom: PhantomData,
    }
}

pub struct DenseNodes<'a, N: NodeT = Node> {
    block: &'a PrimitiveBlock,
    dids: slice::Iter<'a, i64>,
    dlats: slice::Iter<'a, i64>,
    dlons: slice::Iter<'a, i64>,
    keys_vals: slice::Iter<'a, i32>,
    cur_id: i64,
    cur_lat: i64,
    cur_lon: i64,
    #[cfg(feature = "full-metadata")]
    denseinfo: DenseInfo<'a>,
    phantom: PhantomData<N>,
}

#[cfg(feature = "full-metadata")]
pub struct DenseInfo<'a> {
    block: &'a PrimitiveBlock,
    version: slice::Iter<'a, i32>,
    dtimestamp: slice::Iter<'a, i64>,
    dchangeset: slice::Iter<'a, i64>,
    duid: slice::Iter<'a, i32>,
    duser_sid: slice::Iter<'a, i32>,
    visible: slice::Iter<'a, bool>,
    // Note that version and visible are not delta-coded
    cur_timestamp: i64,
    cur_changeset: i64,
    cur_uid: i32,
    cur_user_sid: i32,
}

impl<N: NodeT> Iterator for DenseNodes<'_, N> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
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

        #[cfg(feature = "full-metadata")]
        let info = {
            match (
                self.denseinfo.version.next(),
                self.denseinfo.dtimestamp.next(),
                self.denseinfo.dchangeset.next(),
                self.denseinfo.duid.next(),
                self.denseinfo.duser_sid.next(),
                self.denseinfo.visible.next(),
            ) {
                (
                    Some(&version),
                    Some(&dtimestamp),
                    Some(&dchangeset),
                    Some(&duid),
                    Some(&duser_sid),
                    visible,
                ) => {
                    self.denseinfo.cur_timestamp += dtimestamp;
                    self.denseinfo.cur_changeset += dchangeset;
                    self.denseinfo.cur_uid += duid;
                    self.denseinfo.cur_user_sid += duser_sid;

                    let user = Some(make_string(
                        self.denseinfo.cur_user_sid as usize,
                        self.denseinfo.block,
                    ));

                    Some(Info {
                        version: NonZero::new(version),
                        timestamp: NonZero::new(self.denseinfo.cur_timestamp),
                        changeset: NonZero::new(self.denseinfo.cur_changeset),
                        uid: NonZero::new(self.denseinfo.cur_uid),
                        user,
                        visible: *visible.unwrap_or(&true),
                    })
                }
                _ => None,
            }
        };

        Some(N::get_node(
            Node {
                id: NodeId(self.cur_id),
                decimicro_lat: make_lat(self.cur_lat, self.block),
                decimicro_lon: make_lon(self.cur_lon, self.block),
                tags,
            },
            #[cfg(feature = "full-metadata")]
            info,
            #[cfg(not(feature = "full-metadata"))]
            None,
        ))
    }
}

pub fn ways<'a>(group: &'a PrimitiveGroup, block: &'a PrimitiveBlock) -> Ways<'a, Way> {
    ways_generic::<Way>(group, block)
}

fn ways_generic<'a, W: WayT>(group: &'a PrimitiveGroup, block: &'a PrimitiveBlock) -> Ways<'a, W> {
    Ways::<W> {
        iter: group.ways.iter(),
        block,
        phantom: PhantomData,
    }
}

pub struct Ways<'a, W: WayT = Way> {
    iter: slice::Iter<'a, osmformat::Way>,
    block: &'a PrimitiveBlock,
    phantom: PhantomData<W>,
}

impl<W: WayT> Iterator for Ways<'_, W> {
    type Item = W;
    fn next(&mut self) -> Option<W> {
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
            W::get_way(
                Way {
                    id: WayId(w.id()),
                    nodes,
                    tags: make_tags(&w.keys, &w.vals, self.block),
                },
                #[cfg(feature = "full-metadata")]
                make_info(&w.info, self.block),
                #[cfg(not(feature = "full-metadata"))]
                None,
            )
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub fn relations<'a>(group: &'a PrimitiveGroup, block: &'a PrimitiveBlock) -> Relations<'a> {
    relations_generic::<Relation>(group, block)
}

pub fn relations_generic<'a, R: RelationT>(
    group: &'a PrimitiveGroup,
    block: &'a PrimitiveBlock,
) -> Relations<'a, R> {
    Relations::<R> {
        iter: group.relations.iter(),
        block,
        phantom: PhantomData,
    }
}
pub struct Relations<'a, R: RelationT = Relation> {
    iter: slice::Iter<'a, osmformat::Relation>,
    block: &'a PrimitiveBlock,
    phantom: PhantomData<R>,
}

impl<R: RelationT> Iterator for Relations<'_, R> {
    type Item = R;
    fn next(&mut self) -> Option<R> {
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
            R::get_relation(
                Relation {
                    id: RelationId(rel.id()),
                    refs,
                    tags: make_tags(&rel.keys, &rel.vals, self.block),
                },
                #[cfg(feature = "full-metadata")]
                make_info(&rel.info, self.block),
                #[cfg(not(feature = "full-metadata"))]
                None,
            )
        })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

fn make_string(k: usize, block: &osmformat::PrimitiveBlock) -> String {
    let cow = std::string::String::from_utf8_lossy(&block.stringtable.s[k]);
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

#[cfg(feature = "full-metadata")]
use protobuf::MessageField;

#[cfg(feature = "full-metadata")]
fn make_info(info: &MessageField<osmformat::Info>, b: &PrimitiveBlock) -> Option<Info> {
    if info.has_timestamp() {
        let user = if let Some(user_sid) = info.user_sid {
            Some(make_string(user_sid as usize, b))
        } else {
            None
        };
        Some(Info {
            version: NonZero::new(info.version.unwrap_or(0)),
            timestamp: NonZero::new(info.timestamp.unwrap_or(0)),
            changeset: NonZero::new(info.changeset.unwrap_or(0)),
            uid: NonZero::new(info.uid.unwrap_or(0)),
            user,
            visible: info.visible.unwrap_or(true),
        })
    } else {
        None
    }
}

pub trait NodeT {
    fn get_node(node: Node, info: Option<Info>) -> Self;
}
impl NodeT for Node {
    fn get_node(node: Node, _info: Option<Info>) -> Self {
        node
    }
}

#[cfg(feature = "full-metadata")]
impl NodeT for NodeInfo {
    fn get_node(node: Node, info: Option<Info>) -> Self {
        NodeInfo { node, info }
    }
}

pub trait WayT {
    fn get_way(way: Way, info: Option<Info>) -> Self;
}
impl WayT for Way {
    fn get_way(way: Way, _info: Option<Info>) -> Self {
        way
    }
}

#[cfg(feature = "full-metadata")]
impl WayT for WayInfo {
    fn get_way(way: Way, info: Option<Info>) -> Self {
        WayInfo { way, info }
    }
}

pub trait RelationT {
    fn get_relation(relation: Relation, info: Option<Info>) -> Self;
}
impl RelationT for Relation {
    fn get_relation(relation: Relation, _info: Option<Info>) -> Self {
        relation
    }
}

#[cfg(feature = "full-metadata")]
impl RelationT for RelationInfo {
    fn get_relation(relation: Relation, info: Option<Info>) -> Self {
        RelationInfo { relation, info }
    }
}

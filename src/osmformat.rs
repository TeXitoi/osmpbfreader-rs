//! Automatically generated rust module for 'osmformat.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]

pub mod mod_OSMPBF {

use std::io::Write;
use std::borrow::Cow;
use quick_protobuf::{MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct HeaderBlock<'a> {
    pub bbox: Option<HeaderBBox>,
    pub required_features: Vec<Cow<'a, str>>,
    pub optional_features: Vec<Cow<'a, str>>,
    pub writingprogram: Option<Cow<'a, str>>,
    pub source: Option<Cow<'a, str>>,
    pub osmosis_replication_timestamp: Option<i64>,
    pub osmosis_replication_sequence_number: Option<i64>,
    pub osmosis_replication_base_url: Option<Cow<'a, str>>,
}

impl<'a> HeaderBlock<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.bbox = Some(r.read_message(bytes, HeaderBBox::from_reader)?),
                Ok(34) => msg.required_features.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.optional_features.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(130) => msg.writingprogram = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(138) => msg.source = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(256) => msg.osmosis_replication_timestamp = Some(r.read_int64(bytes)?),
                Ok(264) => msg.osmosis_replication_sequence_number = Some(r.read_int64(bytes)?),
                Ok(274) => msg.osmosis_replication_base_url = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for HeaderBlock<'a> {
    fn get_size(&self) -> usize {
        0
        + self.bbox.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.required_features.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.optional_features.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.writingprogram.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.source.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
        + self.osmosis_replication_timestamp.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.osmosis_replication_sequence_number.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.osmosis_replication_base_url.as_ref().map_or(0, |m| 2 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.bbox { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.required_features { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        for s in &self.optional_features { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.writingprogram { w.write_with_tag(130, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.source { w.write_with_tag(138, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.osmosis_replication_timestamp { w.write_with_tag(256, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.osmosis_replication_sequence_number { w.write_with_tag(264, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.osmosis_replication_base_url { w.write_with_tag(274, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct HeaderBBox {
    pub left: i64,
    pub right: i64,
    pub top: i64,
    pub bottom: i64,
}

impl HeaderBBox {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.left = r.read_sint64(bytes)?,
                Ok(16) => msg.right = r.read_sint64(bytes)?,
                Ok(24) => msg.top = r.read_sint64(bytes)?,
                Ok(32) => msg.bottom = r.read_sint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for HeaderBBox {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_sint64(*(&self.left))
        + 1 + sizeof_sint64(*(&self.right))
        + 1 + sizeof_sint64(*(&self.top))
        + 1 + sizeof_sint64(*(&self.bottom))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_sint64(*&self.left))?;
        w.write_with_tag(16, |w| w.write_sint64(*&self.right))?;
        w.write_with_tag(24, |w| w.write_sint64(*&self.top))?;
        w.write_with_tag(32, |w| w.write_sint64(*&self.bottom))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PrimitiveBlock<'a> {
    pub stringtable: StringTable<'a>,
    pub primitivegroup: Vec<PrimitiveGroup>,
    pub granularity: i32,
    pub lat_offset: i64,
    pub lon_offset: i64,
    pub date_granularity: i32,
}

impl<'a> PrimitiveBlock<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = PrimitiveBlock {
            granularity: 100i32,
            date_granularity: 1000i32,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.stringtable = r.read_message(bytes, StringTable::from_reader)?,
                Ok(18) => msg.primitivegroup.push(r.read_message(bytes, PrimitiveGroup::from_reader)?),
                Ok(136) => msg.granularity = r.read_int32(bytes)?,
                Ok(152) => msg.lat_offset = r.read_int64(bytes)?,
                Ok(160) => msg.lon_offset = r.read_int64(bytes)?,
                Ok(144) => msg.date_granularity = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PrimitiveBlock<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.stringtable).get_size())
        + self.primitivegroup.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.granularity == 100i32 { 0 } else { 2 + sizeof_varint(*(&self.granularity) as u64) }
        + if self.lat_offset == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.lat_offset) as u64) }
        + if self.lon_offset == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.lon_offset) as u64) }
        + if self.date_granularity == 1000i32 { 0 } else { 2 + sizeof_varint(*(&self.date_granularity) as u64) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.stringtable))?;
        for s in &self.primitivegroup { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.granularity != 100i32 { w.write_with_tag(136, |w| w.write_int32(*&self.granularity))?; }
        if self.lat_offset != 0i64 { w.write_with_tag(152, |w| w.write_int64(*&self.lat_offset))?; }
        if self.lon_offset != 0i64 { w.write_with_tag(160, |w| w.write_int64(*&self.lon_offset))?; }
        if self.date_granularity != 1000i32 { w.write_with_tag(144, |w| w.write_int32(*&self.date_granularity))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PrimitiveGroup {
    pub nodes: Vec<Node>,
    pub dense: Option<DenseNodes>,
    pub ways: Vec<Way>,
    pub relations: Vec<Relation>,
    pub changesets: Vec<ChangeSet>,
}

impl PrimitiveGroup {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.nodes.push(r.read_message(bytes, Node::from_reader)?),
                Ok(18) => msg.dense = Some(r.read_message(bytes, DenseNodes::from_reader)?),
                Ok(26) => msg.ways.push(r.read_message(bytes, Way::from_reader)?),
                Ok(34) => msg.relations.push(r.read_message(bytes, Relation::from_reader)?),
                Ok(42) => msg.changesets.push(r.read_message(bytes, ChangeSet::from_reader)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PrimitiveGroup {
    fn get_size(&self) -> usize {
        0
        + self.nodes.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.dense.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.ways.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.relations.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.changesets.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.nodes { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.dense { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.ways { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.relations { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.changesets { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct StringTable<'a> {
    pub s: Vec<Cow<'a, [u8]>>,
}

impl<'a> StringTable<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.s.push(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StringTable<'a> {
    fn get_size(&self) -> usize {
        0
        + self.s.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.s { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Info {
    pub version: i32,
    pub timestamp: Option<i64>,
    pub changeset: Option<i64>,
    pub uid: Option<i32>,
    pub user_sid: Option<u32>,
    pub visible: Option<bool>,
}

impl Info {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Info {
            version: -1i32,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.version = r.read_int32(bytes)?,
                Ok(16) => msg.timestamp = Some(r.read_int64(bytes)?),
                Ok(24) => msg.changeset = Some(r.read_int64(bytes)?),
                Ok(32) => msg.uid = Some(r.read_int32(bytes)?),
                Ok(40) => msg.user_sid = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.visible = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Info {
    fn get_size(&self) -> usize {
        0
        + if self.version == -1i32 { 0 } else { 1 + sizeof_varint(*(&self.version) as u64) }
        + self.timestamp.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.changeset.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.user_sid.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.visible.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.version != -1i32 { w.write_with_tag(8, |w| w.write_int32(*&self.version))?; }
        if let Some(ref s) = self.timestamp { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.changeset { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.uid { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.user_sid { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.visible { w.write_with_tag(48, |w| w.write_bool(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DenseInfo {
    pub version: Vec<i32>,
    pub timestamp: Vec<i64>,
    pub changeset: Vec<i64>,
    pub uid: Vec<i32>,
    pub user_sid: Vec<i32>,
    pub visible: Vec<bool>,
}

impl DenseInfo {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.version = r.read_packed(bytes, |r, bytes| r.read_int32(bytes))?,
                Ok(18) => msg.timestamp = r.read_packed(bytes, |r, bytes| r.read_sint64(bytes))?,
                Ok(26) => msg.changeset = r.read_packed(bytes, |r, bytes| r.read_sint64(bytes))?,
                Ok(34) => msg.uid = r.read_packed(bytes, |r, bytes| r.read_sint32(bytes))?,
                Ok(42) => msg.user_sid = r.read_packed(bytes, |r, bytes| r.read_sint32(bytes))?,
                Ok(50) => msg.visible = r.read_packed(bytes, |r, bytes| r.read_bool(bytes))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DenseInfo {
    fn get_size(&self) -> usize {
        0
        + if self.version.is_empty() { 0 } else { 1 + sizeof_len(self.version.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.timestamp.is_empty() { 0 } else { 1 + sizeof_len(self.timestamp.iter().map(|s| sizeof_sint64(*(s))).sum::<usize>()) }
        + if self.changeset.is_empty() { 0 } else { 1 + sizeof_len(self.changeset.iter().map(|s| sizeof_sint64(*(s))).sum::<usize>()) }
        + if self.uid.is_empty() { 0 } else { 1 + sizeof_len(self.uid.iter().map(|s| sizeof_sint32(*(s))).sum::<usize>()) }
        + if self.user_sid.is_empty() { 0 } else { 1 + sizeof_len(self.user_sid.iter().map(|s| sizeof_sint32(*(s))).sum::<usize>()) }
        + if self.visible.is_empty() { 0 } else { 1 + sizeof_len(self.visible.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(10, &self.version, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        w.write_packed_with_tag(18, &self.timestamp, |w, m| w.write_sint64(*m), &|m| sizeof_sint64(*(m)))?;
        w.write_packed_with_tag(26, &self.changeset, |w, m| w.write_sint64(*m), &|m| sizeof_sint64(*(m)))?;
        w.write_packed_with_tag(34, &self.uid, |w, m| w.write_sint32(*m), &|m| sizeof_sint32(*(m)))?;
        w.write_packed_with_tag(42, &self.user_sid, |w, m| w.write_sint32(*m), &|m| sizeof_sint32(*(m)))?;
        w.write_packed_with_tag(50, &self.visible, |w, m| w.write_bool(*m), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChangeSet {
    pub id: i64,
}

impl ChangeSet {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChangeSet {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.id) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.id))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Node {
    pub id: i64,
    pub keys: Vec<u32>,
    pub vals: Vec<u32>,
    pub info: Option<Info>,
    pub lat: i64,
    pub lon: i64,
}

impl Node {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_sint64(bytes)?,
                Ok(18) => msg.keys = r.read_packed(bytes, |r, bytes| r.read_uint32(bytes))?,
                Ok(26) => msg.vals = r.read_packed(bytes, |r, bytes| r.read_uint32(bytes))?,
                Ok(34) => msg.info = Some(r.read_message(bytes, Info::from_reader)?),
                Ok(64) => msg.lat = r.read_sint64(bytes)?,
                Ok(72) => msg.lon = r.read_sint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Node {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_sint64(*(&self.id))
        + if self.keys.is_empty() { 0 } else { 1 + sizeof_len(self.keys.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.vals.is_empty() { 0 } else { 1 + sizeof_len(self.vals.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + self.info.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + 1 + sizeof_sint64(*(&self.lat))
        + 1 + sizeof_sint64(*(&self.lon))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_sint64(*&self.id))?;
        w.write_packed_with_tag(18, &self.keys, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        w.write_packed_with_tag(26, &self.vals, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        if let Some(ref s) = self.info { w.write_with_tag(34, |w| w.write_message(s))?; }
        w.write_with_tag(64, |w| w.write_sint64(*&self.lat))?;
        w.write_with_tag(72, |w| w.write_sint64(*&self.lon))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DenseNodes {
    pub id: Vec<i64>,
    pub denseinfo: Option<DenseInfo>,
    pub lat: Vec<i64>,
    pub lon: Vec<i64>,
    pub keys_vals: Vec<i32>,
}

impl DenseNodes {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_packed(bytes, |r, bytes| r.read_sint64(bytes))?,
                Ok(42) => msg.denseinfo = Some(r.read_message(bytes, DenseInfo::from_reader)?),
                Ok(66) => msg.lat = r.read_packed(bytes, |r, bytes| r.read_sint64(bytes))?,
                Ok(74) => msg.lon = r.read_packed(bytes, |r, bytes| r.read_sint64(bytes))?,
                Ok(82) => msg.keys_vals = r.read_packed(bytes, |r, bytes| r.read_int32(bytes))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DenseNodes {
    fn get_size(&self) -> usize {
        0
        + if self.id.is_empty() { 0 } else { 1 + sizeof_len(self.id.iter().map(|s| sizeof_sint64(*(s))).sum::<usize>()) }
        + self.denseinfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.lat.is_empty() { 0 } else { 1 + sizeof_len(self.lat.iter().map(|s| sizeof_sint64(*(s))).sum::<usize>()) }
        + if self.lon.is_empty() { 0 } else { 1 + sizeof_len(self.lon.iter().map(|s| sizeof_sint64(*(s))).sum::<usize>()) }
        + if self.keys_vals.is_empty() { 0 } else { 1 + sizeof_len(self.keys_vals.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(10, &self.id, |w, m| w.write_sint64(*m), &|m| sizeof_sint64(*(m)))?;
        if let Some(ref s) = self.denseinfo { w.write_with_tag(42, |w| w.write_message(s))?; }
        w.write_packed_with_tag(66, &self.lat, |w, m| w.write_sint64(*m), &|m| sizeof_sint64(*(m)))?;
        w.write_packed_with_tag(74, &self.lon, |w, m| w.write_sint64(*m), &|m| sizeof_sint64(*(m)))?;
        w.write_packed_with_tag(82, &self.keys_vals, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Way {
    pub id: i64,
    pub keys: Vec<u32>,
    pub vals: Vec<u32>,
    pub info: Option<Info>,
    pub refs: Vec<i64>,
}

impl Way {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_int64(bytes)?,
                Ok(18) => msg.keys = r.read_packed(bytes, |r, bytes| r.read_uint32(bytes))?,
                Ok(26) => msg.vals = r.read_packed(bytes, |r, bytes| r.read_uint32(bytes))?,
                Ok(34) => msg.info = Some(r.read_message(bytes, Info::from_reader)?),
                Ok(66) => msg.refs = r.read_packed(bytes, |r, bytes| r.read_sint64(bytes))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Way {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.id) as u64)
        + if self.keys.is_empty() { 0 } else { 1 + sizeof_len(self.keys.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.vals.is_empty() { 0 } else { 1 + sizeof_len(self.vals.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + self.info.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.refs.is_empty() { 0 } else { 1 + sizeof_len(self.refs.iter().map(|s| sizeof_sint64(*(s))).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.id))?;
        w.write_packed_with_tag(18, &self.keys, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        w.write_packed_with_tag(26, &self.vals, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        if let Some(ref s) = self.info { w.write_with_tag(34, |w| w.write_message(s))?; }
        w.write_packed_with_tag(66, &self.refs, |w, m| w.write_sint64(*m), &|m| sizeof_sint64(*(m)))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Relation {
    pub id: i64,
    pub keys: Vec<u32>,
    pub vals: Vec<u32>,
    pub info: Option<Info>,
    pub roles_sid: Vec<i32>,
    pub memids: Vec<i64>,
    pub types: Vec<mod_Relation::MemberType>,
}

impl Relation {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_int64(bytes)?,
                Ok(18) => msg.keys = r.read_packed(bytes, |r, bytes| r.read_uint32(bytes))?,
                Ok(26) => msg.vals = r.read_packed(bytes, |r, bytes| r.read_uint32(bytes))?,
                Ok(34) => msg.info = Some(r.read_message(bytes, Info::from_reader)?),
                Ok(66) => msg.roles_sid = r.read_packed(bytes, |r, bytes| r.read_int32(bytes))?,
                Ok(74) => msg.memids = r.read_packed(bytes, |r, bytes| r.read_sint64(bytes))?,
                Ok(82) => msg.types = r.read_packed(bytes, |r, bytes| r.read_enum(bytes))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Relation {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.id) as u64)
        + if self.keys.is_empty() { 0 } else { 1 + sizeof_len(self.keys.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.vals.is_empty() { 0 } else { 1 + sizeof_len(self.vals.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + self.info.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.roles_sid.is_empty() { 0 } else { 1 + sizeof_len(self.roles_sid.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.memids.is_empty() { 0 } else { 1 + sizeof_len(self.memids.iter().map(|s| sizeof_sint64(*(s))).sum::<usize>()) }
        + if self.types.is_empty() { 0 } else { 1 + sizeof_len(self.types.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.id))?;
        w.write_packed_with_tag(18, &self.keys, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        w.write_packed_with_tag(26, &self.vals, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        if let Some(ref s) = self.info { w.write_with_tag(34, |w| w.write_message(s))?; }
        w.write_packed_with_tag(66, &self.roles_sid, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        w.write_packed_with_tag(74, &self.memids, |w, m| w.write_sint64(*m), &|m| sizeof_sint64(*(m)))?;
        w.write_packed_with_tag(82, &self.types, |w, m| w.write_enum(*m as i32), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}

pub mod mod_Relation {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MemberType {
    NODE = 0,
    WAY = 1,
    RELATION = 2,
}

impl Default for MemberType {
    fn default() -> Self {
        MemberType::NODE
    }
}

impl From<i32> for MemberType {
    fn from(i: i32) -> Self {
        match i {
            0 => MemberType::NODE,
            1 => MemberType::WAY,
            2 => MemberType::RELATION,
            _ => Self::default(),
        }
    }
}

}

}

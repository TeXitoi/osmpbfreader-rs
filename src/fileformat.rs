//! Automatically generated rust module for 'fileformat.proto' file

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
pub struct Blob<'a> {
    pub raw: Option<Cow<'a, [u8]>>,
    pub raw_size: Option<i32>,
    pub zlib_data: Option<Cow<'a, [u8]>>,
    pub lzma_data: Option<Cow<'a, [u8]>>,
}

impl<'a> Blob<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.raw = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.raw_size = Some(r.read_int32(bytes)?),
                Ok(26) => msg.zlib_data = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.lzma_data = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Blob<'a> {
    fn get_size(&self) -> usize {
        0
        + self.raw.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.raw_size.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.zlib_data.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.lzma_data.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.raw { w.write_with_tag(10, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.raw_size { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.zlib_data { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.lzma_data { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BlobHeader<'a> {
    pub type_pb: Cow<'a, str>,
    pub indexdata: Option<Cow<'a, [u8]>>,
    pub datasize: i32,
}

impl<'a> BlobHeader<'a> {
    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.type_pb = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.indexdata = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.datasize = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BlobHeader<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.type_pb).len())
        + self.indexdata.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + 1 + sizeof_varint(*(&self.datasize) as u64)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.type_pb))?;
        if let Some(ref s) = self.indexdata { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        w.write_with_tag(24, |w| w.write_int32(*&self.datasize))?;
        Ok(())
    }
}

}

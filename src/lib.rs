extern crate protobuf;
extern crate flate2;

use std::error::Error;
use std::error::FromError;
use std::io::IoError;
use std::collections::BTreeMap;

pub type Tags = BTreeMap<String, String>;

#[allow(non_snake_case)]
pub mod fileformat;
pub mod osmformat;

#[deriving(Show)]
pub enum OsmPbfError {
    Io(IoError),
    Pbf(protobuf::ProtobufError),
    UnsupportedData,
}
impl Error for OsmPbfError {
    fn description(&self) -> &str {
        match *self {
            OsmPbfError::Io(ref e) => e.description(),
            OsmPbfError::Pbf(_) => "Protobuf Error",
            OsmPbfError::UnsupportedData => "Unsupported data",
        }
    }
    fn detail(&self) -> Option<String> {
        match *self {
            OsmPbfError::Io(ref e) => e.detail(),
            OsmPbfError::Pbf(_) => None,
            OsmPbfError::UnsupportedData => None,
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            OsmPbfError::Io(ref e) => Some(e as &Error),
            OsmPbfError::Pbf(_) => None,
            OsmPbfError::UnsupportedData => None,
        }
    }
}
impl FromError<IoError> for OsmPbfError {
    fn from_error(err: IoError) -> OsmPbfError {
        OsmPbfError::Io(err)
    }
}
impl FromError<protobuf::ProtobufError> for OsmPbfError {
    fn from_error(err: protobuf::ProtobufError) -> OsmPbfError {
        OsmPbfError::Pbf(err)
    }
}


pub struct OsmPbfReader<R> {
    buf: Vec<u8>,
    r: R,
    finished: bool,
}

impl<R: Reader> OsmPbfReader<R> {
    pub fn with_reader(r: R) -> OsmPbfReader<R> {
        OsmPbfReader {
            buf: vec![],
            r: r,
            finished: false,
        }
    }
    fn push(&mut self, sz: uint) -> Result<(), OsmPbfError> {
        self.buf.clear();
        let readed = try!(self.r.push_at_least(sz, sz, &mut self.buf));
        assert_eq!(sz, readed);
        Ok(())
    }
    fn read_primitive_block(&mut self, blob: fileformat::Blob)
                            -> Result<osmformat::PrimitiveBlock, OsmPbfError>
    {
        if blob.has_raw() {
            Ok(try!(protobuf::parse_from_bytes(blob.get_raw())))
        } else if blob.has_zlib_data() {
            use flate2::reader::ZlibDecoder;
            let r = std::io::BufReader::new(blob.get_zlib_data());
            let mut zr = ZlibDecoder::new(r);
            let buf = try!(zr.read_to_end());// TODO use self.buf
            Ok(try!(protobuf::parse_from_bytes(buf.as_slice())))
        } else {
            Err(OsmPbfError::UnsupportedData)
        }
    }
    fn try_primitive_block(&mut self, sz: uint)
                           -> Result<Option<osmformat::PrimitiveBlock>, OsmPbfError>
    {
        try!(self.push(sz));
        let header: fileformat::BlobHeader =
            try!(protobuf::parse_from_bytes(self.buf.as_slice()));
        let sz = header.get_datasize() as uint;
        try!(self.push(sz));
        let blob: fileformat::Blob = try!(protobuf::parse_from_bytes(self.buf.as_slice()));
        let primitive_opt = if header.get_field_type() == "OSMData" {
            Some(try!(self.read_primitive_block(blob)))
        } else if header.get_field_type() == "OSMHeader" {
            None
        } else {
            println!("Unknown type: {}", header.get_field_type());
            None
        };
        Ok(primitive_opt)
    }
    fn next_primitive_block(&mut self)
                            -> Option<Result<osmformat::PrimitiveBlock, OsmPbfError>>
    {
        use std::io::IoErrorKind;
        if self.finished { return None; }
        let sz = match self.r.read_be_u32() {
            Ok(sz) => sz,
            Err(ref e) if e.kind == IoErrorKind::EndOfFile => {
                self.finished = true;
                return None;
            }
            Err(e) => {
                self.finished = true;
                return Some(Err(FromError::from_error(e)));
            }
        } as uint;
        match self.try_primitive_block(sz) {
            Ok(Some(p)) => Some(Ok(p)),
            Ok(None) => self.next_primitive_block(),
            Err(e) => {
                self.finished = true;
                Some(Err(e))
            }
        }
    }
    pub fn primitive_blocks<'a>(&'a mut self) -> PrimitiveBlocks<'a, R> {
        PrimitiveBlocks { opr: self }
    }
}

pub struct PrimitiveBlocks<'a, R: 'a> {
    opr: &'a mut OsmPbfReader<R>
}
impl<'a, R: Reader> Iterator<Result<osmformat::PrimitiveBlock, OsmPbfError>>
    for PrimitiveBlocks<'a, R>
{
    fn next(&mut self) -> Option<Result<osmformat::PrimitiveBlock, OsmPbfError>> {
        self.opr.next_primitive_block()
    }
}

pub struct SimpleNodes<'a> {
    nodes: std::iter::FlatMap<'a, &'a osmformat::PrimitiveGroup, std::slice::Items<'a, osmformat::PrimitiveGroup>, std::slice::Items<'a, osmformat::Node>>,
    block: &'a osmformat::PrimitiveBlock,
}
impl<'a> SimpleNodes<'a> {
    pub fn with_block(block: &'a osmformat::PrimitiveBlock) -> SimpleNodes<'a> {
        SimpleNodes {
            nodes: block.get_primitivegroup().iter().flat_map(|g| g.get_nodes().iter()),
            block: block,
        }
    }
}
impl<'a> Iterator<Node> for SimpleNodes<'a> {
    fn next(&mut self) -> Option<Node> {
        self.nodes.next().map(|n| Node::with_node(n, self.block))
    }
    fn size_hint(&self) -> (uint, Option<uint>) {
        self.nodes.size_hint()
    }
}

pub struct Node {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
    pub tags: Tags,
}
impl Node {
    fn make_lat(c: i64, b: &osmformat::PrimitiveBlock) -> f64 {
        let granularity = b.get_granularity() as i64;
        1e-9 * (b.get_lat_offset() + granularity * c) as f64
    }
    fn make_lon(c: i64, b: &osmformat::PrimitiveBlock) -> f64 {
        let granularity = b.get_granularity() as i64;
        1e-9 * (b.get_lon_offset() + granularity * c) as f64
    }
    pub fn with_node(n: &osmformat::Node, b: &osmformat::PrimitiveBlock) -> Node {
        Node {
            id: n.get_id(),
            lat: Node::make_lat(n.get_lat(), b),
            lon: Node::make_lat(n.get_lon(), b),
            tags: make_tags(n, b),
        }
    }
}
fn make_string(k: uint, block: &osmformat::PrimitiveBlock) -> String {
    String::from_utf8_lossy(block.get_stringtable().get_s()[k].as_slice())
        .into_string()
}
fn make_tags(n: &osmformat::Node, b: &osmformat::PrimitiveBlock) -> Tags {
    let mut tags = BTreeMap::new();
    for (&k, &v) in n.get_keys().iter().zip(n.get_vals().iter()) {
        let k = make_string(k as uint, b);
        let v = make_string(v as uint, b);
        tags.insert(k, v);
    }
    tags
}

type RawDenseNodes<'a> = std::iter::Map<'a, (&'a i64, (&'a i64, &'a i64)), RawDenseNode, std::iter::Zip<std::slice::Items<'a, i64>, std::iter::Zip<std::slice::Items<'a, i64>, std::slice::Items<'a, i64>>>>;

pub struct DenseNodes<'a> {
    block: &'a osmformat::PrimitiveBlock,
    groups: std::slice::Items<'a, osmformat::PrimitiveGroup>,
    denses: Option<(RawDenseNodes<'a>, std::slice::Items<'a, i32>)>,
    cur_id: i64,
    cur_lat: i64,
    cur_lon: i64,
}
impl<'a> DenseNodes<'a> {
    pub fn with_block(b: &'a osmformat::PrimitiveBlock) -> DenseNodes<'a> {
        DenseNodes {
            block: b,
            groups: b.get_primitivegroup().iter(),
            denses: None,
            cur_id: 0,
            cur_lat: 0,
            cur_lon: 0,
        }
    }
}
impl<'a> Iterator<Node> for DenseNodes<'a> {
    fn next(&mut self) -> Option<Node> {
        loop {
            for &(ref mut dense, ref mut kvs) in self.denses.iter_mut() {
                for d in *dense {
                    self.cur_id += d.did;
                    self.cur_lat += d.dlat;
                    self.cur_lon += d.dlon;
                    let mut tags = BTreeMap::new();
                    loop {
                        let k = match kvs.next() {
                            None | Some(&0) => break,
                            Some(k) => make_string(*k as uint, self.block),
                        };
                        let v = match kvs.next() {
                            None => break,
                            Some(v) => make_string(*v as uint, self.block),
                        };
                        tags.insert(k, v);
                    }
                    return Some(Node {
                        id: self.cur_id,
                        lat: Node::make_lat(self.cur_lat, self.block),
                        lon: Node::make_lon(self.cur_lon, self.block),
                        tags: tags,
                    });
                }
            }
            self.cur_id = 0;
            self.cur_lat = 0;
            self.cur_lon = 0;
            let next_chunk = self.groups.next().map(|g| {
                let d = g.get_dense();
                (d.get_id().iter().zip(d.get_lat().iter().zip(d.get_lon().iter()))
                 .map(RawDenseNode::from_tuple),
                 g.get_dense())
            });
            match next_chunk {
                None => return None,
                Some((next, dense)) =>
                    self.denses = Some((next, dense.get_keys_vals().iter())),
            }
        }
    }
}

struct RawDenseNode {
    did: i64,
    dlat: i64,
    dlon: i64,
}
impl RawDenseNode {
    fn from_tuple((&did, (&dlat, &dlon)): (&i64, (&i64, &i64)))
                  -> RawDenseNode
    {
        RawDenseNode {
            did: did,
            dlat: dlat,
            dlon: dlon,
        }
    }
}

pub fn nodes<'a>(block: &'a osmformat::PrimitiveBlock)
                 -> std::iter::Chain<SimpleNodes<'a>, DenseNodes<'a>>
{
    SimpleNodes::with_block(block).chain(DenseNodes::with_block(block))
}

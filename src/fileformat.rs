// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]


#[deriving(Clone,PartialEq,Default,Show)]
pub struct Blob {
    raw: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    raw_size: ::std::option::Option<i32>,
    zlib_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    lzma_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    OBSOLETE_bzip2_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Blob {
    pub fn new() -> Blob {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Blob {
        static mut instance: ::protobuf::lazy::Lazy<Blob> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const Blob };
        unsafe {
            instance.get(|| {
                Blob {
                    raw: ::protobuf::SingularField::none(),
                    raw_size: ::std::option::None,
                    zlib_data: ::protobuf::SingularField::none(),
                    lzma_data: ::protobuf::SingularField::none(),
                    OBSOLETE_bzip2_data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional bytes raw = 1;

    pub fn clear_raw(&mut self) {
        self.raw.clear();
    }

    pub fn has_raw(&self) -> bool {
        self.raw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raw(&mut self, v: ::std::vec::Vec<u8>) {
        self.raw = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_raw(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.raw.is_none() {
            self.raw.set_default();
        };
        self.raw.as_mut().unwrap()
    }

    pub fn get_raw(&'a self) -> &'a [u8] {
        match self.raw.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional int32 raw_size = 2;

    pub fn clear_raw_size(&mut self) {
        self.raw_size = None;
    }

    pub fn has_raw_size(&self) -> bool {
        self.raw_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raw_size(&mut self, v: i32) {
        self.raw_size = Some(v);
    }

    pub fn get_raw_size(&self) -> i32 {
        self.raw_size.unwrap_or(0)
    }

    // optional bytes zlib_data = 3;

    pub fn clear_zlib_data(&mut self) {
        self.zlib_data.clear();
    }

    pub fn has_zlib_data(&self) -> bool {
        self.zlib_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_zlib_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.zlib_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_zlib_data(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.zlib_data.is_none() {
            self.zlib_data.set_default();
        };
        self.zlib_data.as_mut().unwrap()
    }

    pub fn get_zlib_data(&'a self) -> &'a [u8] {
        match self.zlib_data.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional bytes lzma_data = 4;

    pub fn clear_lzma_data(&mut self) {
        self.lzma_data.clear();
    }

    pub fn has_lzma_data(&self) -> bool {
        self.lzma_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lzma_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.lzma_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lzma_data(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.lzma_data.is_none() {
            self.lzma_data.set_default();
        };
        self.lzma_data.as_mut().unwrap()
    }

    pub fn get_lzma_data(&'a self) -> &'a [u8] {
        match self.lzma_data.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional bytes OBSOLETE_bzip2_data = 5;

    pub fn clear_OBSOLETE_bzip2_data(&mut self) {
        self.OBSOLETE_bzip2_data.clear();
    }

    pub fn has_OBSOLETE_bzip2_data(&self) -> bool {
        self.OBSOLETE_bzip2_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_OBSOLETE_bzip2_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.OBSOLETE_bzip2_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_OBSOLETE_bzip2_data(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.OBSOLETE_bzip2_data.is_none() {
            self.OBSOLETE_bzip2_data.set_default();
        };
        self.OBSOLETE_bzip2_data.as_mut().unwrap()
    }

    pub fn get_OBSOLETE_bzip2_data(&'a self) -> &'a [u8] {
        match self.OBSOLETE_bzip2_data.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }
}

impl ::protobuf::Message for Blob {
    fn new() -> Blob {
        Blob::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.raw.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.raw_size = Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.zlib_data.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.lzma_data.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.OBSOLETE_bzip2_data.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.raw.iter() {
            my_size += ::protobuf::rt::bytes_size(1, value.as_slice());
        };
        for value in self.raw_size.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.zlib_data.iter() {
            my_size += ::protobuf::rt::bytes_size(3, value.as_slice());
        };
        for value in self.lzma_data.iter() {
            my_size += ::protobuf::rt::bytes_size(4, value.as_slice());
        };
        for value in self.OBSOLETE_bzip2_data.iter() {
            my_size += ::protobuf::rt::bytes_size(5, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.raw.as_ref() {
            Some(v) => {
                try!(os.write_bytes(1, v.as_slice()));
            },
            None => {},
        };
        match self.raw_size {
            Some(v) => {
                try!(os.write_int32(2, v));
            },
            None => {},
        };
        match self.zlib_data.as_ref() {
            Some(v) => {
                try!(os.write_bytes(3, v.as_slice()));
            },
            None => {},
        };
        match self.lzma_data.as_ref() {
            Some(v) => {
                try!(os.write_bytes(4, v.as_slice()));
            },
            None => {},
        };
        match self.OBSOLETE_bzip2_data.as_ref() {
            Some(v) => {
                try!(os.write_bytes(5, v.as_slice()));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Blob>()
    }
}

impl ::protobuf::Clear for Blob {
    fn clear(&mut self) {
        self.clear_raw();
        self.clear_raw_size();
        self.clear_zlib_data();
        self.clear_lzma_data();
        self.clear_OBSOLETE_bzip2_data();
        self.unknown_fields.clear();
    }
}

#[deriving(Clone,PartialEq,Default,Show)]
pub struct BlobHeader {
    field_type: ::protobuf::SingularField<::std::string::String>,
    indexdata: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    datasize: ::std::option::Option<i32>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> BlobHeader {
    pub fn new() -> BlobHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlobHeader {
        static mut instance: ::protobuf::lazy::Lazy<BlobHeader> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const BlobHeader };
        unsafe {
            instance.get(|| {
                BlobHeader {
                    field_type: ::protobuf::SingularField::none(),
                    indexdata: ::protobuf::SingularField::none(),
                    datasize: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&'a mut self) -> &'a mut ::std::string::String {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    pub fn get_field_type(&'a self) -> &'a str {
        match self.field_type.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional bytes indexdata = 2;

    pub fn clear_indexdata(&mut self) {
        self.indexdata.clear();
    }

    pub fn has_indexdata(&self) -> bool {
        self.indexdata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_indexdata(&mut self, v: ::std::vec::Vec<u8>) {
        self.indexdata = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_indexdata(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.indexdata.is_none() {
            self.indexdata.set_default();
        };
        self.indexdata.as_mut().unwrap()
    }

    pub fn get_indexdata(&'a self) -> &'a [u8] {
        match self.indexdata.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // required int32 datasize = 3;

    pub fn clear_datasize(&mut self) {
        self.datasize = None;
    }

    pub fn has_datasize(&self) -> bool {
        self.datasize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_datasize(&mut self, v: i32) {
        self.datasize = Some(v);
    }

    pub fn get_datasize(&self) -> i32 {
        self.datasize.unwrap_or(0)
    }
}

impl ::protobuf::Message for BlobHeader {
    fn new() -> BlobHeader {
        BlobHeader::new()
    }

    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        if self.datasize.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.field_type.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.indexdata.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.datasize = Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        for value in self.indexdata.iter() {
            my_size += ::protobuf::rt::bytes_size(2, value.as_slice());
        };
        for value in self.datasize.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.field_type.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        match self.indexdata.as_ref() {
            Some(v) => {
                try!(os.write_bytes(2, v.as_slice()));
            },
            None => {},
        };
        match self.datasize {
            Some(v) => {
                try!(os.write_int32(3, v));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<BlobHeader>()
    }
}

impl ::protobuf::Clear for BlobHeader {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_indexdata();
        self.clear_datasize();
        self.unknown_fields.clear();
    }
}

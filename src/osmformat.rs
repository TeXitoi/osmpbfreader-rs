// This file is generated. Do not edit
// @generated

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default,Debug)]
pub struct HeaderBlock {
    // message fields
    bbox: ::protobuf::SingularPtrField<HeaderBBox>,
    required_features: ::protobuf::RepeatedField<::std::string::String>,
    optional_features: ::protobuf::RepeatedField<::std::string::String>,
    writingprogram: ::protobuf::SingularField<::std::string::String>,
    source: ::protobuf::SingularField<::std::string::String>,
    osmosis_replication_timestamp: ::std::option::Option<i64>,
    osmosis_replication_sequence_number: ::std::option::Option<i64>,
    osmosis_replication_base_url: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HeaderBlock {}

impl HeaderBlock {
    pub fn new() -> HeaderBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HeaderBlock {
        static mut instance: ::protobuf::lazy::Lazy<HeaderBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HeaderBlock,
        };
        unsafe {
            instance.get(|| {
                HeaderBlock {
                    bbox: ::protobuf::SingularPtrField::none(),
                    required_features: ::protobuf::RepeatedField::new(),
                    optional_features: ::protobuf::RepeatedField::new(),
                    writingprogram: ::protobuf::SingularField::none(),
                    source: ::protobuf::SingularField::none(),
                    osmosis_replication_timestamp: ::std::option::Option::None,
                    osmosis_replication_sequence_number: ::std::option::Option::None,
                    osmosis_replication_base_url: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .OSMPBF.HeaderBBox bbox = 1;

    pub fn clear_bbox(&mut self) {
        self.bbox.clear();
    }

    pub fn has_bbox(&self) -> bool {
        self.bbox.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bbox(&mut self, v: HeaderBBox) {
        self.bbox = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bbox<'a>(&'a mut self) -> &'a mut HeaderBBox {
        if self.bbox.is_none() {
            self.bbox.set_default();
        };
        self.bbox.as_mut().unwrap()
    }

    // Take field
    pub fn take_bbox(&mut self) -> HeaderBBox {
        self.bbox.take().unwrap_or_else(|| HeaderBBox::new())
    }

    pub fn get_bbox<'a>(&'a self) -> &'a HeaderBBox {
        self.bbox.as_ref().unwrap_or_else(|| HeaderBBox::default_instance())
    }

    // repeated string required_features = 4;

    pub fn clear_required_features(&mut self) {
        self.required_features.clear();
    }

    // Param is passed by value, moved
    pub fn set_required_features(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.required_features = v;
    }

    // Mutable pointer to the field.
    pub fn mut_required_features<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.required_features
    }

    // Take field
    pub fn take_required_features(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.required_features, ::protobuf::RepeatedField::new())
    }

    pub fn get_required_features<'a>(&'a self) -> &'a [::std::string::String] {
        &self.required_features
    }

    // repeated string optional_features = 5;

    pub fn clear_optional_features(&mut self) {
        self.optional_features.clear();
    }

    // Param is passed by value, moved
    pub fn set_optional_features(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.optional_features = v;
    }

    // Mutable pointer to the field.
    pub fn mut_optional_features<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.optional_features
    }

    // Take field
    pub fn take_optional_features(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.optional_features, ::protobuf::RepeatedField::new())
    }

    pub fn get_optional_features<'a>(&'a self) -> &'a [::std::string::String] {
        &self.optional_features
    }

    // optional string writingprogram = 16;

    pub fn clear_writingprogram(&mut self) {
        self.writingprogram.clear();
    }

    pub fn has_writingprogram(&self) -> bool {
        self.writingprogram.is_some()
    }

    // Param is passed by value, moved
    pub fn set_writingprogram(&mut self, v: ::std::string::String) {
        self.writingprogram = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_writingprogram<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.writingprogram.is_none() {
            self.writingprogram.set_default();
        };
        self.writingprogram.as_mut().unwrap()
    }

    // Take field
    pub fn take_writingprogram(&mut self) -> ::std::string::String {
        self.writingprogram.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_writingprogram<'a>(&'a self) -> &'a str {
        match self.writingprogram.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string source = 17;

    pub fn clear_source(&mut self) {
        self.source.clear();
    }

    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: ::std::string::String) {
        self.source = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.source.is_none() {
            self.source.set_default();
        };
        self.source.as_mut().unwrap()
    }

    // Take field
    pub fn take_source(&mut self) -> ::std::string::String {
        self.source.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_source<'a>(&'a self) -> &'a str {
        match self.source.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int64 osmosis_replication_timestamp = 32;

    pub fn clear_osmosis_replication_timestamp(&mut self) {
        self.osmosis_replication_timestamp = ::std::option::Option::None;
    }

    pub fn has_osmosis_replication_timestamp(&self) -> bool {
        self.osmosis_replication_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_osmosis_replication_timestamp(&mut self, v: i64) {
        self.osmosis_replication_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_osmosis_replication_timestamp<'a>(&self) -> i64 {
        self.osmosis_replication_timestamp.unwrap_or(0)
    }

    // optional int64 osmosis_replication_sequence_number = 33;

    pub fn clear_osmosis_replication_sequence_number(&mut self) {
        self.osmosis_replication_sequence_number = ::std::option::Option::None;
    }

    pub fn has_osmosis_replication_sequence_number(&self) -> bool {
        self.osmosis_replication_sequence_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_osmosis_replication_sequence_number(&mut self, v: i64) {
        self.osmosis_replication_sequence_number = ::std::option::Option::Some(v);
    }

    pub fn get_osmosis_replication_sequence_number<'a>(&self) -> i64 {
        self.osmosis_replication_sequence_number.unwrap_or(0)
    }

    // optional string osmosis_replication_base_url = 34;

    pub fn clear_osmosis_replication_base_url(&mut self) {
        self.osmosis_replication_base_url.clear();
    }

    pub fn has_osmosis_replication_base_url(&self) -> bool {
        self.osmosis_replication_base_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_osmosis_replication_base_url(&mut self, v: ::std::string::String) {
        self.osmosis_replication_base_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_osmosis_replication_base_url<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.osmosis_replication_base_url.is_none() {
            self.osmosis_replication_base_url.set_default();
        };
        self.osmosis_replication_base_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_osmosis_replication_base_url(&mut self) -> ::std::string::String {
        self.osmosis_replication_base_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_osmosis_replication_base_url<'a>(&'a self) -> &'a str {
        match self.osmosis_replication_base_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for HeaderBlock {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.bbox));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.required_features));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.optional_features));
                },
                16 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.writingprogram));
                },
                17 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.source));
                },
                32 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.osmosis_replication_timestamp = ::std::option::Option::Some(tmp);
                },
                33 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.osmosis_replication_sequence_number = ::std::option::Option::Some(tmp);
                },
                34 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.osmosis_replication_base_url));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.bbox.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.required_features.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in self.optional_features.iter() {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in self.writingprogram.iter() {
            my_size += ::protobuf::rt::string_size(16, &value);
        };
        for value in self.source.iter() {
            my_size += ::protobuf::rt::string_size(17, &value);
        };
        for value in self.osmosis_replication_timestamp.iter() {
            my_size += ::protobuf::rt::value_size(32, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.osmosis_replication_sequence_number.iter() {
            my_size += ::protobuf::rt::value_size(33, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.osmosis_replication_base_url.iter() {
            my_size += ::protobuf::rt::string_size(34, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bbox.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.required_features.iter() {
            try!(os.write_string(4, &v));
        };
        for v in self.optional_features.iter() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.writingprogram.as_ref() {
            try!(os.write_string(16, &v));
        };
        if let Some(v) = self.source.as_ref() {
            try!(os.write_string(17, &v));
        };
        if let Some(v) = self.osmosis_replication_timestamp {
            try!(os.write_int64(32, v));
        };
        if let Some(v) = self.osmosis_replication_sequence_number {
            try!(os.write_int64(33, v));
        };
        if let Some(v) = self.osmosis_replication_base_url.as_ref() {
            try!(os.write_string(34, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<HeaderBlock>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for HeaderBlock {
    fn new() -> HeaderBlock {
        HeaderBlock::new()
    }
}

impl ::protobuf::Clear for HeaderBlock {
    fn clear(&mut self) {
        self.clear_bbox();
        self.clear_required_features();
        self.clear_optional_features();
        self.clear_writingprogram();
        self.clear_source();
        self.clear_osmosis_replication_timestamp();
        self.clear_osmosis_replication_sequence_number();
        self.clear_osmosis_replication_base_url();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for HeaderBlock {
    fn eq(&self, other: &HeaderBlock) -> bool {
        self.bbox == other.bbox &&
        self.required_features == other.required_features &&
        self.optional_features == other.optional_features &&
        self.writingprogram == other.writingprogram &&
        self.source == other.source &&
        self.osmosis_replication_timestamp == other.osmosis_replication_timestamp &&
        self.osmosis_replication_sequence_number == other.osmosis_replication_sequence_number &&
        self.osmosis_replication_base_url == other.osmosis_replication_base_url &&
        self.unknown_fields == other.unknown_fields
    }
}

#[derive(Clone,Default,Debug)]
pub struct HeaderBBox {
    // message fields
    left: ::std::option::Option<i64>,
    right: ::std::option::Option<i64>,
    top: ::std::option::Option<i64>,
    bottom: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HeaderBBox {}

impl HeaderBBox {
    pub fn new() -> HeaderBBox {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HeaderBBox {
        static mut instance: ::protobuf::lazy::Lazy<HeaderBBox> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HeaderBBox,
        };
        unsafe {
            instance.get(|| {
                HeaderBBox {
                    left: ::std::option::Option::None,
                    right: ::std::option::Option::None,
                    top: ::std::option::Option::None,
                    bottom: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required sint64 left = 1;

    pub fn clear_left(&mut self) {
        self.left = ::std::option::Option::None;
    }

    pub fn has_left(&self) -> bool {
        self.left.is_some()
    }

    // Param is passed by value, moved
    pub fn set_left(&mut self, v: i64) {
        self.left = ::std::option::Option::Some(v);
    }

    pub fn get_left<'a>(&self) -> i64 {
        self.left.unwrap_or(0)
    }

    // required sint64 right = 2;

    pub fn clear_right(&mut self) {
        self.right = ::std::option::Option::None;
    }

    pub fn has_right(&self) -> bool {
        self.right.is_some()
    }

    // Param is passed by value, moved
    pub fn set_right(&mut self, v: i64) {
        self.right = ::std::option::Option::Some(v);
    }

    pub fn get_right<'a>(&self) -> i64 {
        self.right.unwrap_or(0)
    }

    // required sint64 top = 3;

    pub fn clear_top(&mut self) {
        self.top = ::std::option::Option::None;
    }

    pub fn has_top(&self) -> bool {
        self.top.is_some()
    }

    // Param is passed by value, moved
    pub fn set_top(&mut self, v: i64) {
        self.top = ::std::option::Option::Some(v);
    }

    pub fn get_top<'a>(&self) -> i64 {
        self.top.unwrap_or(0)
    }

    // required sint64 bottom = 4;

    pub fn clear_bottom(&mut self) {
        self.bottom = ::std::option::Option::None;
    }

    pub fn has_bottom(&self) -> bool {
        self.bottom.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bottom(&mut self, v: i64) {
        self.bottom = ::std::option::Option::Some(v);
    }

    pub fn get_bottom<'a>(&self) -> i64 {
        self.bottom.unwrap_or(0)
    }
}

impl ::protobuf::Message for HeaderBBox {
    fn is_initialized(&self) -> bool {
        if self.left.is_none() {
            return false;
        };
        if self.right.is_none() {
            return false;
        };
        if self.top.is_none() {
            return false;
        };
        if self.bottom.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint64());
                    self.left = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint64());
                    self.right = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint64());
                    self.top = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint64());
                    self.bottom = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.left.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.right.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.top.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.bottom.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.left {
            try!(os.write_sint64(1, v));
        };
        if let Some(v) = self.right {
            try!(os.write_sint64(2, v));
        };
        if let Some(v) = self.top {
            try!(os.write_sint64(3, v));
        };
        if let Some(v) = self.bottom {
            try!(os.write_sint64(4, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<HeaderBBox>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for HeaderBBox {
    fn new() -> HeaderBBox {
        HeaderBBox::new()
    }
}

impl ::protobuf::Clear for HeaderBBox {
    fn clear(&mut self) {
        self.clear_left();
        self.clear_right();
        self.clear_top();
        self.clear_bottom();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for HeaderBBox {
    fn eq(&self, other: &HeaderBBox) -> bool {
        self.left == other.left &&
        self.right == other.right &&
        self.top == other.top &&
        self.bottom == other.bottom &&
        self.unknown_fields == other.unknown_fields
    }
}

#[derive(Clone,Default,Debug)]
pub struct PrimitiveBlock {
    // message fields
    stringtable: ::protobuf::SingularPtrField<StringTable>,
    primitivegroup: ::protobuf::RepeatedField<PrimitiveGroup>,
    granularity: ::std::option::Option<i32>,
    lat_offset: ::std::option::Option<i64>,
    lon_offset: ::std::option::Option<i64>,
    date_granularity: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PrimitiveBlock {}

impl PrimitiveBlock {
    pub fn new() -> PrimitiveBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PrimitiveBlock {
        static mut instance: ::protobuf::lazy::Lazy<PrimitiveBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PrimitiveBlock,
        };
        unsafe {
            instance.get(|| {
                PrimitiveBlock {
                    stringtable: ::protobuf::SingularPtrField::none(),
                    primitivegroup: ::protobuf::RepeatedField::new(),
                    granularity: ::std::option::Option::None,
                    lat_offset: ::std::option::Option::None,
                    lon_offset: ::std::option::Option::None,
                    date_granularity: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .OSMPBF.StringTable stringtable = 1;

    pub fn clear_stringtable(&mut self) {
        self.stringtable.clear();
    }

    pub fn has_stringtable(&self) -> bool {
        self.stringtable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stringtable(&mut self, v: StringTable) {
        self.stringtable = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stringtable<'a>(&'a mut self) -> &'a mut StringTable {
        if self.stringtable.is_none() {
            self.stringtable.set_default();
        };
        self.stringtable.as_mut().unwrap()
    }

    // Take field
    pub fn take_stringtable(&mut self) -> StringTable {
        self.stringtable.take().unwrap_or_else(|| StringTable::new())
    }

    pub fn get_stringtable<'a>(&'a self) -> &'a StringTable {
        self.stringtable.as_ref().unwrap_or_else(|| StringTable::default_instance())
    }

    // repeated .OSMPBF.PrimitiveGroup primitivegroup = 2;

    pub fn clear_primitivegroup(&mut self) {
        self.primitivegroup.clear();
    }

    // Param is passed by value, moved
    pub fn set_primitivegroup(&mut self, v: ::protobuf::RepeatedField<PrimitiveGroup>) {
        self.primitivegroup = v;
    }

    // Mutable pointer to the field.
    pub fn mut_primitivegroup<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<PrimitiveGroup> {
        &mut self.primitivegroup
    }

    // Take field
    pub fn take_primitivegroup(&mut self) -> ::protobuf::RepeatedField<PrimitiveGroup> {
        ::std::mem::replace(&mut self.primitivegroup, ::protobuf::RepeatedField::new())
    }

    pub fn get_primitivegroup<'a>(&'a self) -> &'a [PrimitiveGroup] {
        &self.primitivegroup
    }

    // optional int32 granularity = 17;

    pub fn clear_granularity(&mut self) {
        self.granularity = ::std::option::Option::None;
    }

    pub fn has_granularity(&self) -> bool {
        self.granularity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_granularity(&mut self, v: i32) {
        self.granularity = ::std::option::Option::Some(v);
    }

    pub fn get_granularity<'a>(&self) -> i32 {
        self.granularity.unwrap_or(100i32)
    }

    // optional int64 lat_offset = 19;

    pub fn clear_lat_offset(&mut self) {
        self.lat_offset = ::std::option::Option::None;
    }

    pub fn has_lat_offset(&self) -> bool {
        self.lat_offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lat_offset(&mut self, v: i64) {
        self.lat_offset = ::std::option::Option::Some(v);
    }

    pub fn get_lat_offset<'a>(&self) -> i64 {
        self.lat_offset.unwrap_or(0i64)
    }

    // optional int64 lon_offset = 20;

    pub fn clear_lon_offset(&mut self) {
        self.lon_offset = ::std::option::Option::None;
    }

    pub fn has_lon_offset(&self) -> bool {
        self.lon_offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lon_offset(&mut self, v: i64) {
        self.lon_offset = ::std::option::Option::Some(v);
    }

    pub fn get_lon_offset<'a>(&self) -> i64 {
        self.lon_offset.unwrap_or(0i64)
    }

    // optional int32 date_granularity = 18;

    pub fn clear_date_granularity(&mut self) {
        self.date_granularity = ::std::option::Option::None;
    }

    pub fn has_date_granularity(&self) -> bool {
        self.date_granularity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_date_granularity(&mut self, v: i32) {
        self.date_granularity = ::std::option::Option::Some(v);
    }

    pub fn get_date_granularity<'a>(&self) -> i32 {
        self.date_granularity.unwrap_or(1000i32)
    }
}

impl ::protobuf::Message for PrimitiveBlock {
    fn is_initialized(&self) -> bool {
        if self.stringtable.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stringtable));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.primitivegroup));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.granularity = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.lat_offset = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.lon_offset = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.date_granularity = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.stringtable.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.primitivegroup.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.granularity.iter() {
            my_size += ::protobuf::rt::value_size(17, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.lat_offset.iter() {
            my_size += ::protobuf::rt::value_size(19, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.lon_offset.iter() {
            my_size += ::protobuf::rt::value_size(20, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.date_granularity.iter() {
            my_size += ::protobuf::rt::value_size(18, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.stringtable.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.primitivegroup.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.granularity {
            try!(os.write_int32(17, v));
        };
        if let Some(v) = self.lat_offset {
            try!(os.write_int64(19, v));
        };
        if let Some(v) = self.lon_offset {
            try!(os.write_int64(20, v));
        };
        if let Some(v) = self.date_granularity {
            try!(os.write_int32(18, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PrimitiveBlock>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PrimitiveBlock {
    fn new() -> PrimitiveBlock {
        PrimitiveBlock::new()
    }
}

impl ::protobuf::Clear for PrimitiveBlock {
    fn clear(&mut self) {
        self.clear_stringtable();
        self.clear_primitivegroup();
        self.clear_granularity();
        self.clear_lat_offset();
        self.clear_lon_offset();
        self.clear_date_granularity();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PrimitiveBlock {
    fn eq(&self, other: &PrimitiveBlock) -> bool {
        self.stringtable == other.stringtable &&
        self.primitivegroup == other.primitivegroup &&
        self.granularity == other.granularity &&
        self.lat_offset == other.lat_offset &&
        self.lon_offset == other.lon_offset &&
        self.date_granularity == other.date_granularity &&
        self.unknown_fields == other.unknown_fields
    }
}

#[derive(Clone,Default,Debug)]
pub struct PrimitiveGroup {
    // message fields
    nodes: ::protobuf::RepeatedField<Node>,
    dense: ::protobuf::SingularPtrField<DenseNodes>,
    ways: ::protobuf::RepeatedField<Way>,
    relations: ::protobuf::RepeatedField<Relation>,
    changesets: ::protobuf::RepeatedField<ChangeSet>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PrimitiveGroup {}

impl PrimitiveGroup {
    pub fn new() -> PrimitiveGroup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PrimitiveGroup {
        static mut instance: ::protobuf::lazy::Lazy<PrimitiveGroup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PrimitiveGroup,
        };
        unsafe {
            instance.get(|| {
                PrimitiveGroup {
                    nodes: ::protobuf::RepeatedField::new(),
                    dense: ::protobuf::SingularPtrField::none(),
                    ways: ::protobuf::RepeatedField::new(),
                    relations: ::protobuf::RepeatedField::new(),
                    changesets: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .OSMPBF.Node nodes = 1;

    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: ::protobuf::RepeatedField<Node>) {
        self.nodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodes<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Node> {
        &mut self.nodes
    }

    // Take field
    pub fn take_nodes(&mut self) -> ::protobuf::RepeatedField<Node> {
        ::std::mem::replace(&mut self.nodes, ::protobuf::RepeatedField::new())
    }

    pub fn get_nodes<'a>(&'a self) -> &'a [Node] {
        &self.nodes
    }

    // optional .OSMPBF.DenseNodes dense = 2;

    pub fn clear_dense(&mut self) {
        self.dense.clear();
    }

    pub fn has_dense(&self) -> bool {
        self.dense.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dense(&mut self, v: DenseNodes) {
        self.dense = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dense<'a>(&'a mut self) -> &'a mut DenseNodes {
        if self.dense.is_none() {
            self.dense.set_default();
        };
        self.dense.as_mut().unwrap()
    }

    // Take field
    pub fn take_dense(&mut self) -> DenseNodes {
        self.dense.take().unwrap_or_else(|| DenseNodes::new())
    }

    pub fn get_dense<'a>(&'a self) -> &'a DenseNodes {
        self.dense.as_ref().unwrap_or_else(|| DenseNodes::default_instance())
    }

    // repeated .OSMPBF.Way ways = 3;

    pub fn clear_ways(&mut self) {
        self.ways.clear();
    }

    // Param is passed by value, moved
    pub fn set_ways(&mut self, v: ::protobuf::RepeatedField<Way>) {
        self.ways = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ways<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Way> {
        &mut self.ways
    }

    // Take field
    pub fn take_ways(&mut self) -> ::protobuf::RepeatedField<Way> {
        ::std::mem::replace(&mut self.ways, ::protobuf::RepeatedField::new())
    }

    pub fn get_ways<'a>(&'a self) -> &'a [Way] {
        &self.ways
    }

    // repeated .OSMPBF.Relation relations = 4;

    pub fn clear_relations(&mut self) {
        self.relations.clear();
    }

    // Param is passed by value, moved
    pub fn set_relations(&mut self, v: ::protobuf::RepeatedField<Relation>) {
        self.relations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_relations<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Relation> {
        &mut self.relations
    }

    // Take field
    pub fn take_relations(&mut self) -> ::protobuf::RepeatedField<Relation> {
        ::std::mem::replace(&mut self.relations, ::protobuf::RepeatedField::new())
    }

    pub fn get_relations<'a>(&'a self) -> &'a [Relation] {
        &self.relations
    }

    // repeated .OSMPBF.ChangeSet changesets = 5;

    pub fn clear_changesets(&mut self) {
        self.changesets.clear();
    }

    // Param is passed by value, moved
    pub fn set_changesets(&mut self, v: ::protobuf::RepeatedField<ChangeSet>) {
        self.changesets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_changesets<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<ChangeSet> {
        &mut self.changesets
    }

    // Take field
    pub fn take_changesets(&mut self) -> ::protobuf::RepeatedField<ChangeSet> {
        ::std::mem::replace(&mut self.changesets, ::protobuf::RepeatedField::new())
    }

    pub fn get_changesets<'a>(&'a self) -> &'a [ChangeSet] {
        &self.changesets
    }
}

impl ::protobuf::Message for PrimitiveGroup {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.nodes));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dense));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ways));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.relations));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.changesets));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.nodes.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.dense.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.ways.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.relations.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.changesets.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.nodes.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.dense.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.ways.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.relations.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.changesets.iter() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PrimitiveGroup>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PrimitiveGroup {
    fn new() -> PrimitiveGroup {
        PrimitiveGroup::new()
    }
}

impl ::protobuf::Clear for PrimitiveGroup {
    fn clear(&mut self) {
        self.clear_nodes();
        self.clear_dense();
        self.clear_ways();
        self.clear_relations();
        self.clear_changesets();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PrimitiveGroup {
    fn eq(&self, other: &PrimitiveGroup) -> bool {
        self.nodes == other.nodes &&
        self.dense == other.dense &&
        self.ways == other.ways &&
        self.relations == other.relations &&
        self.changesets == other.changesets &&
        self.unknown_fields == other.unknown_fields
    }
}

#[derive(Clone,Default,Debug)]
pub struct StringTable {
    // message fields
    s: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StringTable {}

impl StringTable {
    pub fn new() -> StringTable {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StringTable {
        static mut instance: ::protobuf::lazy::Lazy<StringTable> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StringTable,
        };
        unsafe {
            instance.get(|| {
                StringTable {
                    s: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated bytes s = 1;

    pub fn clear_s(&mut self) {
        self.s.clear();
    }

    // Param is passed by value, moved
    pub fn set_s(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.s = v;
    }

    // Mutable pointer to the field.
    pub fn mut_s<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.s
    }

    // Take field
    pub fn take_s(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.s, ::protobuf::RepeatedField::new())
    }

    pub fn get_s<'a>(&'a self) -> &'a [::std::vec::Vec<u8>] {
        &self.s
    }
}

impl ::protobuf::Message for StringTable {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.s));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.s.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.s.iter() {
            try!(os.write_bytes(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StringTable>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StringTable {
    fn new() -> StringTable {
        StringTable::new()
    }
}

impl ::protobuf::Clear for StringTable {
    fn clear(&mut self) {
        self.clear_s();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StringTable {
    fn eq(&self, other: &StringTable) -> bool {
        self.s == other.s &&
        self.unknown_fields == other.unknown_fields
    }
}

#[derive(Clone,Default,Debug)]
pub struct Info {
    // message fields
    version: ::std::option::Option<i32>,
    timestamp: ::std::option::Option<i64>,
    changeset: ::std::option::Option<i64>,
    uid: ::std::option::Option<i32>,
    user_sid: ::std::option::Option<u32>,
    visible: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Info {}

impl Info {
    pub fn new() -> Info {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Info {
        static mut instance: ::protobuf::lazy::Lazy<Info> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Info,
        };
        unsafe {
            instance.get(|| {
                Info {
                    version: ::std::option::Option::None,
                    timestamp: ::std::option::Option::None,
                    changeset: ::std::option::Option::None,
                    uid: ::std::option::Option::None,
                    user_sid: ::std::option::Option::None,
                    visible: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version<'a>(&self) -> i32 {
        self.version.unwrap_or(-1i32)
    }

    // optional int64 timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp<'a>(&self) -> i64 {
        self.timestamp.unwrap_or(0)
    }

    // optional int64 changeset = 3;

    pub fn clear_changeset(&mut self) {
        self.changeset = ::std::option::Option::None;
    }

    pub fn has_changeset(&self) -> bool {
        self.changeset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_changeset(&mut self, v: i64) {
        self.changeset = ::std::option::Option::Some(v);
    }

    pub fn get_changeset<'a>(&self) -> i64 {
        self.changeset.unwrap_or(0)
    }

    // optional int32 uid = 4;

    pub fn clear_uid(&mut self) {
        self.uid = ::std::option::Option::None;
    }

    pub fn has_uid(&self) -> bool {
        self.uid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uid(&mut self, v: i32) {
        self.uid = ::std::option::Option::Some(v);
    }

    pub fn get_uid<'a>(&self) -> i32 {
        self.uid.unwrap_or(0)
    }

    // optional uint32 user_sid = 5;

    pub fn clear_user_sid(&mut self) {
        self.user_sid = ::std::option::Option::None;
    }

    pub fn has_user_sid(&self) -> bool {
        self.user_sid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_sid(&mut self, v: u32) {
        self.user_sid = ::std::option::Option::Some(v);
    }

    pub fn get_user_sid<'a>(&self) -> u32 {
        self.user_sid.unwrap_or(0)
    }

    // optional bool visible = 6;

    pub fn clear_visible(&mut self) {
        self.visible = ::std::option::Option::None;
    }

    pub fn has_visible(&self) -> bool {
        self.visible.is_some()
    }

    // Param is passed by value, moved
    pub fn set_visible(&mut self, v: bool) {
        self.visible = ::std::option::Option::Some(v);
    }

    pub fn get_visible<'a>(&self) -> bool {
        self.visible.unwrap_or(false)
    }
}

impl ::protobuf::Message for Info {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.changeset = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.uid = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.user_sid = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.visible = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.version.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.timestamp.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.changeset.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.uid.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.user_sid.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.visible.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.timestamp {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.changeset {
            try!(os.write_int64(3, v));
        };
        if let Some(v) = self.uid {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.user_sid {
            try!(os.write_uint32(5, v));
        };
        if let Some(v) = self.visible {
            try!(os.write_bool(6, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Info>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Info {
    fn new() -> Info {
        Info::new()
    }
}

impl ::protobuf::Clear for Info {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_timestamp();
        self.clear_changeset();
        self.clear_uid();
        self.clear_user_sid();
        self.clear_visible();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Info {
    fn eq(&self, other: &Info) -> bool {
        self.version == other.version &&
        self.timestamp == other.timestamp &&
        self.changeset == other.changeset &&
        self.uid == other.uid &&
        self.user_sid == other.user_sid &&
        self.visible == other.visible &&
        self.unknown_fields == other.unknown_fields
    }
}

#[derive(Clone,Default,Debug)]
pub struct DenseInfo {
    // message fields
    version: ::std::vec::Vec<i32>,
    timestamp: ::std::vec::Vec<i64>,
    changeset: ::std::vec::Vec<i64>,
    uid: ::std::vec::Vec<i32>,
    user_sid: ::std::vec::Vec<i32>,
    visible: ::std::vec::Vec<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DenseInfo {}

impl DenseInfo {
    pub fn new() -> DenseInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DenseInfo {
        static mut instance: ::protobuf::lazy::Lazy<DenseInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DenseInfo,
        };
        unsafe {
            instance.get(|| {
                DenseInfo {
                    version: ::std::vec::Vec::new(),
                    timestamp: ::std::vec::Vec::new(),
                    changeset: ::std::vec::Vec::new(),
                    uid: ::std::vec::Vec::new(),
                    user_sid: ::std::vec::Vec::new(),
                    visible: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated int32 version = 1;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::vec::Vec<i32>) {
        self.version = v;
    }

    // Mutable pointer to the field.
    pub fn mut_version<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.version
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.version, ::std::vec::Vec::new())
    }

    pub fn get_version<'a>(&'a self) -> &'a [i32] {
        &self.version
    }

    // repeated sint64 timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp.clear();
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: ::std::vec::Vec<i64>) {
        self.timestamp = v;
    }

    // Mutable pointer to the field.
    pub fn mut_timestamp<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.timestamp
    }

    // Take field
    pub fn take_timestamp(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.timestamp, ::std::vec::Vec::new())
    }

    pub fn get_timestamp<'a>(&'a self) -> &'a [i64] {
        &self.timestamp
    }

    // repeated sint64 changeset = 3;

    pub fn clear_changeset(&mut self) {
        self.changeset.clear();
    }

    // Param is passed by value, moved
    pub fn set_changeset(&mut self, v: ::std::vec::Vec<i64>) {
        self.changeset = v;
    }

    // Mutable pointer to the field.
    pub fn mut_changeset<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.changeset
    }

    // Take field
    pub fn take_changeset(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.changeset, ::std::vec::Vec::new())
    }

    pub fn get_changeset<'a>(&'a self) -> &'a [i64] {
        &self.changeset
    }

    // repeated sint32 uid = 4;

    pub fn clear_uid(&mut self) {
        self.uid.clear();
    }

    // Param is passed by value, moved
    pub fn set_uid(&mut self, v: ::std::vec::Vec<i32>) {
        self.uid = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uid<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.uid
    }

    // Take field
    pub fn take_uid(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.uid, ::std::vec::Vec::new())
    }

    pub fn get_uid<'a>(&'a self) -> &'a [i32] {
        &self.uid
    }

    // repeated sint32 user_sid = 5;

    pub fn clear_user_sid(&mut self) {
        self.user_sid.clear();
    }

    // Param is passed by value, moved
    pub fn set_user_sid(&mut self, v: ::std::vec::Vec<i32>) {
        self.user_sid = v;
    }

    // Mutable pointer to the field.
    pub fn mut_user_sid<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.user_sid
    }

    // Take field
    pub fn take_user_sid(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.user_sid, ::std::vec::Vec::new())
    }

    pub fn get_user_sid<'a>(&'a self) -> &'a [i32] {
        &self.user_sid
    }

    // repeated bool visible = 6;

    pub fn clear_visible(&mut self) {
        self.visible.clear();
    }

    // Param is passed by value, moved
    pub fn set_visible(&mut self, v: ::std::vec::Vec<bool>) {
        self.visible = v;
    }

    // Mutable pointer to the field.
    pub fn mut_visible<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<bool> {
        &mut self.visible
    }

    // Take field
    pub fn take_visible(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.visible, ::std::vec::Vec::new())
    }

    pub fn get_visible<'a>(&'a self) -> &'a [bool] {
        &self.visible
    }
}

impl ::protobuf::Message for DenseInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.version));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_sint64_into(wire_type, is, &mut self.timestamp));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_sint64_into(wire_type, is, &mut self.changeset));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_sint32_into(wire_type, is, &mut self.uid));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_sint32_into(wire_type, is, &mut self.user_sid));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.visible));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.version.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(1, &self.version);
        };
        if !self.timestamp.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(2, &self.timestamp);
        };
        if !self.changeset.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(3, &self.changeset);
        };
        if !self.uid.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(4, &self.uid);
        };
        if !self.user_sid.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(5, &self.user_sid);
        };
        if !self.visible.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.visible.len() as u32) + (self.visible.len() * 1) as u32;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.version.is_empty() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.version)));
            for v in self.version.iter() {
                try!(os.write_int32_no_tag(*v));
            };
        };
        if !self.timestamp.is_empty() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.timestamp)));
            for v in self.timestamp.iter() {
                try!(os.write_sint64_no_tag(*v));
            };
        };
        if !self.changeset.is_empty() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.changeset)));
            for v in self.changeset.iter() {
                try!(os.write_sint64_no_tag(*v));
            };
        };
        if !self.uid.is_empty() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.uid)));
            for v in self.uid.iter() {
                try!(os.write_sint32_no_tag(*v));
            };
        };
        if !self.user_sid.is_empty() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.user_sid)));
            for v in self.user_sid.iter() {
                try!(os.write_sint32_no_tag(*v));
            };
        };
        if !self.visible.is_empty() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.visible.len() * 1) as u32));
            for v in self.visible.iter() {
                try!(os.write_bool_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DenseInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DenseInfo {
    fn new() -> DenseInfo {
        DenseInfo::new()
    }
}

impl ::protobuf::Clear for DenseInfo {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_timestamp();
        self.clear_changeset();
        self.clear_uid();
        self.clear_user_sid();
        self.clear_visible();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DenseInfo {
    fn eq(&self, other: &DenseInfo) -> bool {
        self.version == other.version &&
        self.timestamp == other.timestamp &&
        self.changeset == other.changeset &&
        self.uid == other.uid &&
        self.user_sid == other.user_sid &&
        self.visible == other.visible &&
        self.unknown_fields == other.unknown_fields
    }
}

#[derive(Clone,Default,Debug)]
pub struct ChangeSet {
    // message fields
    id: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChangeSet {}

impl ChangeSet {
    pub fn new() -> ChangeSet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChangeSet {
        static mut instance: ::protobuf::lazy::Lazy<ChangeSet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChangeSet,
        };
        unsafe {
            instance.get(|| {
                ChangeSet {
                    id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id<'a>(&self) -> i64 {
        self.id.unwrap_or(0)
    }
}

impl ::protobuf::Message for ChangeSet {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_int64(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ChangeSet>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ChangeSet {
    fn new() -> ChangeSet {
        ChangeSet::new()
    }
}

impl ::protobuf::Clear for ChangeSet {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ChangeSet {
    fn eq(&self, other: &ChangeSet) -> bool {
        self.id == other.id &&
        self.unknown_fields == other.unknown_fields
    }
}

#[derive(Clone,Default,Debug)]
pub struct Node {
    // message fields
    id: ::std::option::Option<i64>,
    keys: ::std::vec::Vec<u32>,
    vals: ::std::vec::Vec<u32>,
    info: ::protobuf::SingularPtrField<Info>,
    lat: ::std::option::Option<i64>,
    lon: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Node {}

impl Node {
    pub fn new() -> Node {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Node {
        static mut instance: ::protobuf::lazy::Lazy<Node> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Node,
        };
        unsafe {
            instance.get(|| {
                Node {
                    id: ::std::option::Option::None,
                    keys: ::std::vec::Vec::new(),
                    vals: ::std::vec::Vec::new(),
                    info: ::protobuf::SingularPtrField::none(),
                    lat: ::std::option::Option::None,
                    lon: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required sint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id<'a>(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    // repeated uint32 keys = 2;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::std::vec::Vec<u32>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u32> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }

    pub fn get_keys<'a>(&'a self) -> &'a [u32] {
        &self.keys
    }

    // repeated uint32 vals = 3;

    pub fn clear_vals(&mut self) {
        self.vals.clear();
    }

    // Param is passed by value, moved
    pub fn set_vals(&mut self, v: ::std::vec::Vec<u32>) {
        self.vals = v;
    }

    // Mutable pointer to the field.
    pub fn mut_vals<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u32> {
        &mut self.vals
    }

    // Take field
    pub fn take_vals(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.vals, ::std::vec::Vec::new())
    }

    pub fn get_vals<'a>(&'a self) -> &'a [u32] {
        &self.vals
    }

    // optional .OSMPBF.Info info = 4;

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: Info) {
        self.info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info<'a>(&'a mut self) -> &'a mut Info {
        if self.info.is_none() {
            self.info.set_default();
        };
        self.info.as_mut().unwrap()
    }

    // Take field
    pub fn take_info(&mut self) -> Info {
        self.info.take().unwrap_or_else(|| Info::new())
    }

    pub fn get_info<'a>(&'a self) -> &'a Info {
        self.info.as_ref().unwrap_or_else(|| Info::default_instance())
    }

    // required sint64 lat = 8;

    pub fn clear_lat(&mut self) {
        self.lat = ::std::option::Option::None;
    }

    pub fn has_lat(&self) -> bool {
        self.lat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lat(&mut self, v: i64) {
        self.lat = ::std::option::Option::Some(v);
    }

    pub fn get_lat<'a>(&self) -> i64 {
        self.lat.unwrap_or(0)
    }

    // required sint64 lon = 9;

    pub fn clear_lon(&mut self) {
        self.lon = ::std::option::Option::None;
    }

    pub fn has_lon(&self) -> bool {
        self.lon.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lon(&mut self, v: i64) {
        self.lon = ::std::option::Option::Some(v);
    }

    pub fn get_lon<'a>(&self) -> i64 {
        self.lon.unwrap_or(0)
    }
}

impl ::protobuf::Message for Node {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.lat.is_none() {
            return false;
        };
        if self.lon.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.keys));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.vals));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.info));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint64());
                    self.lat = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint64());
                    self.lon = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.keys.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(2, &self.keys);
        };
        if !self.vals.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(3, &self.vals);
        };
        for value in self.info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.lat.iter() {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.lon.iter() {
            my_size += ::protobuf::rt::value_size(9, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_sint64(1, v));
        };
        if !self.keys.is_empty() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.keys)));
            for v in self.keys.iter() {
                try!(os.write_uint32_no_tag(*v));
            };
        };
        if !self.vals.is_empty() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.vals)));
            for v in self.vals.iter() {
                try!(os.write_uint32_no_tag(*v));
            };
        };
        if let Some(v) = self.info.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.lat {
            try!(os.write_sint64(8, v));
        };
        if let Some(v) = self.lon {
            try!(os.write_sint64(9, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Node>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Node {
    fn new() -> Node {
        Node::new()
    }
}

impl ::protobuf::Clear for Node {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_keys();
        self.clear_vals();
        self.clear_info();
        self.clear_lat();
        self.clear_lon();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.id == other.id &&
        self.keys == other.keys &&
        self.vals == other.vals &&
        self.info == other.info &&
        self.lat == other.lat &&
        self.lon == other.lon &&
        self.unknown_fields == other.unknown_fields
    }
}

#[derive(Clone,Default,Debug)]
pub struct DenseNodes {
    // message fields
    id: ::std::vec::Vec<i64>,
    denseinfo: ::protobuf::SingularPtrField<DenseInfo>,
    lat: ::std::vec::Vec<i64>,
    lon: ::std::vec::Vec<i64>,
    keys_vals: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DenseNodes {}

impl DenseNodes {
    pub fn new() -> DenseNodes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DenseNodes {
        static mut instance: ::protobuf::lazy::Lazy<DenseNodes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DenseNodes,
        };
        unsafe {
            instance.get(|| {
                DenseNodes {
                    id: ::std::vec::Vec::new(),
                    denseinfo: ::protobuf::SingularPtrField::none(),
                    lat: ::std::vec::Vec::new(),
                    lon: ::std::vec::Vec::new(),
                    keys_vals: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated sint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::vec::Vec<i64>) {
        self.id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_id<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.id, ::std::vec::Vec::new())
    }

    pub fn get_id<'a>(&'a self) -> &'a [i64] {
        &self.id
    }

    // optional .OSMPBF.DenseInfo denseinfo = 5;

    pub fn clear_denseinfo(&mut self) {
        self.denseinfo.clear();
    }

    pub fn has_denseinfo(&self) -> bool {
        self.denseinfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_denseinfo(&mut self, v: DenseInfo) {
        self.denseinfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_denseinfo<'a>(&'a mut self) -> &'a mut DenseInfo {
        if self.denseinfo.is_none() {
            self.denseinfo.set_default();
        };
        self.denseinfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_denseinfo(&mut self) -> DenseInfo {
        self.denseinfo.take().unwrap_or_else(|| DenseInfo::new())
    }

    pub fn get_denseinfo<'a>(&'a self) -> &'a DenseInfo {
        self.denseinfo.as_ref().unwrap_or_else(|| DenseInfo::default_instance())
    }

    // repeated sint64 lat = 8;

    pub fn clear_lat(&mut self) {
        self.lat.clear();
    }

    // Param is passed by value, moved
    pub fn set_lat(&mut self, v: ::std::vec::Vec<i64>) {
        self.lat = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lat<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.lat
    }

    // Take field
    pub fn take_lat(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.lat, ::std::vec::Vec::new())
    }

    pub fn get_lat<'a>(&'a self) -> &'a [i64] {
        &self.lat
    }

    // repeated sint64 lon = 9;

    pub fn clear_lon(&mut self) {
        self.lon.clear();
    }

    // Param is passed by value, moved
    pub fn set_lon(&mut self, v: ::std::vec::Vec<i64>) {
        self.lon = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lon<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.lon
    }

    // Take field
    pub fn take_lon(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.lon, ::std::vec::Vec::new())
    }

    pub fn get_lon<'a>(&'a self) -> &'a [i64] {
        &self.lon
    }

    // repeated int32 keys_vals = 10;

    pub fn clear_keys_vals(&mut self) {
        self.keys_vals.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys_vals(&mut self, v: ::std::vec::Vec<i32>) {
        self.keys_vals = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys_vals<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.keys_vals
    }

    // Take field
    pub fn take_keys_vals(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.keys_vals, ::std::vec::Vec::new())
    }

    pub fn get_keys_vals<'a>(&'a self) -> &'a [i32] {
        &self.keys_vals
    }
}

impl ::protobuf::Message for DenseNodes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_sint64_into(wire_type, is, &mut self.id));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.denseinfo));
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_sint64_into(wire_type, is, &mut self.lat));
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_sint64_into(wire_type, is, &mut self.lon));
                },
                10 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.keys_vals));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(1, &self.id);
        };
        for value in self.denseinfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.lat.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(8, &self.lat);
        };
        if !self.lon.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(9, &self.lon);
        };
        if !self.keys_vals.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(10, &self.keys_vals);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.id)));
            for v in self.id.iter() {
                try!(os.write_sint64_no_tag(*v));
            };
        };
        if let Some(v) = self.denseinfo.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if !self.lat.is_empty() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.lat)));
            for v in self.lat.iter() {
                try!(os.write_sint64_no_tag(*v));
            };
        };
        if !self.lon.is_empty() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.lon)));
            for v in self.lon.iter() {
                try!(os.write_sint64_no_tag(*v));
            };
        };
        if !self.keys_vals.is_empty() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.keys_vals)));
            for v in self.keys_vals.iter() {
                try!(os.write_int32_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DenseNodes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DenseNodes {
    fn new() -> DenseNodes {
        DenseNodes::new()
    }
}

impl ::protobuf::Clear for DenseNodes {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_denseinfo();
        self.clear_lat();
        self.clear_lon();
        self.clear_keys_vals();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DenseNodes {
    fn eq(&self, other: &DenseNodes) -> bool {
        self.id == other.id &&
        self.denseinfo == other.denseinfo &&
        self.lat == other.lat &&
        self.lon == other.lon &&
        self.keys_vals == other.keys_vals &&
        self.unknown_fields == other.unknown_fields
    }
}

#[derive(Clone,Default,Debug)]
pub struct Way {
    // message fields
    id: ::std::option::Option<i64>,
    keys: ::std::vec::Vec<u32>,
    vals: ::std::vec::Vec<u32>,
    info: ::protobuf::SingularPtrField<Info>,
    refs: ::std::vec::Vec<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Way {}

impl Way {
    pub fn new() -> Way {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Way {
        static mut instance: ::protobuf::lazy::Lazy<Way> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Way,
        };
        unsafe {
            instance.get(|| {
                Way {
                    id: ::std::option::Option::None,
                    keys: ::std::vec::Vec::new(),
                    vals: ::std::vec::Vec::new(),
                    info: ::protobuf::SingularPtrField::none(),
                    refs: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id<'a>(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    // repeated uint32 keys = 2;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::std::vec::Vec<u32>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u32> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }

    pub fn get_keys<'a>(&'a self) -> &'a [u32] {
        &self.keys
    }

    // repeated uint32 vals = 3;

    pub fn clear_vals(&mut self) {
        self.vals.clear();
    }

    // Param is passed by value, moved
    pub fn set_vals(&mut self, v: ::std::vec::Vec<u32>) {
        self.vals = v;
    }

    // Mutable pointer to the field.
    pub fn mut_vals<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u32> {
        &mut self.vals
    }

    // Take field
    pub fn take_vals(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.vals, ::std::vec::Vec::new())
    }

    pub fn get_vals<'a>(&'a self) -> &'a [u32] {
        &self.vals
    }

    // optional .OSMPBF.Info info = 4;

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: Info) {
        self.info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info<'a>(&'a mut self) -> &'a mut Info {
        if self.info.is_none() {
            self.info.set_default();
        };
        self.info.as_mut().unwrap()
    }

    // Take field
    pub fn take_info(&mut self) -> Info {
        self.info.take().unwrap_or_else(|| Info::new())
    }

    pub fn get_info<'a>(&'a self) -> &'a Info {
        self.info.as_ref().unwrap_or_else(|| Info::default_instance())
    }

    // repeated sint64 refs = 8;

    pub fn clear_refs(&mut self) {
        self.refs.clear();
    }

    // Param is passed by value, moved
    pub fn set_refs(&mut self, v: ::std::vec::Vec<i64>) {
        self.refs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_refs<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.refs
    }

    // Take field
    pub fn take_refs(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.refs, ::std::vec::Vec::new())
    }

    pub fn get_refs<'a>(&'a self) -> &'a [i64] {
        &self.refs
    }
}

impl ::protobuf::Message for Way {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.keys));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.vals));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.info));
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_sint64_into(wire_type, is, &mut self.refs));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.keys.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(2, &self.keys);
        };
        if !self.vals.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(3, &self.vals);
        };
        for value in self.info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.refs.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(8, &self.refs);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_int64(1, v));
        };
        if !self.keys.is_empty() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.keys)));
            for v in self.keys.iter() {
                try!(os.write_uint32_no_tag(*v));
            };
        };
        if !self.vals.is_empty() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.vals)));
            for v in self.vals.iter() {
                try!(os.write_uint32_no_tag(*v));
            };
        };
        if let Some(v) = self.info.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if !self.refs.is_empty() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.refs)));
            for v in self.refs.iter() {
                try!(os.write_sint64_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Way>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Way {
    fn new() -> Way {
        Way::new()
    }
}

impl ::protobuf::Clear for Way {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_keys();
        self.clear_vals();
        self.clear_info();
        self.clear_refs();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Way {
    fn eq(&self, other: &Way) -> bool {
        self.id == other.id &&
        self.keys == other.keys &&
        self.vals == other.vals &&
        self.info == other.info &&
        self.refs == other.refs &&
        self.unknown_fields == other.unknown_fields
    }
}

#[derive(Clone,Default,Debug)]
pub struct Relation {
    // message fields
    id: ::std::option::Option<i64>,
    keys: ::std::vec::Vec<u32>,
    vals: ::std::vec::Vec<u32>,
    info: ::protobuf::SingularPtrField<Info>,
    roles_sid: ::std::vec::Vec<i32>,
    memids: ::std::vec::Vec<i64>,
    types: ::std::vec::Vec<Relation_MemberType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Relation {}

impl Relation {
    pub fn new() -> Relation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Relation {
        static mut instance: ::protobuf::lazy::Lazy<Relation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Relation,
        };
        unsafe {
            instance.get(|| {
                Relation {
                    id: ::std::option::Option::None,
                    keys: ::std::vec::Vec::new(),
                    vals: ::std::vec::Vec::new(),
                    info: ::protobuf::SingularPtrField::none(),
                    roles_sid: ::std::vec::Vec::new(),
                    memids: ::std::vec::Vec::new(),
                    types: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id<'a>(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    // repeated uint32 keys = 2;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::std::vec::Vec<u32>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u32> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }

    pub fn get_keys<'a>(&'a self) -> &'a [u32] {
        &self.keys
    }

    // repeated uint32 vals = 3;

    pub fn clear_vals(&mut self) {
        self.vals.clear();
    }

    // Param is passed by value, moved
    pub fn set_vals(&mut self, v: ::std::vec::Vec<u32>) {
        self.vals = v;
    }

    // Mutable pointer to the field.
    pub fn mut_vals<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u32> {
        &mut self.vals
    }

    // Take field
    pub fn take_vals(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.vals, ::std::vec::Vec::new())
    }

    pub fn get_vals<'a>(&'a self) -> &'a [u32] {
        &self.vals
    }

    // optional .OSMPBF.Info info = 4;

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: Info) {
        self.info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info<'a>(&'a mut self) -> &'a mut Info {
        if self.info.is_none() {
            self.info.set_default();
        };
        self.info.as_mut().unwrap()
    }

    // Take field
    pub fn take_info(&mut self) -> Info {
        self.info.take().unwrap_or_else(|| Info::new())
    }

    pub fn get_info<'a>(&'a self) -> &'a Info {
        self.info.as_ref().unwrap_or_else(|| Info::default_instance())
    }

    // repeated int32 roles_sid = 8;

    pub fn clear_roles_sid(&mut self) {
        self.roles_sid.clear();
    }

    // Param is passed by value, moved
    pub fn set_roles_sid(&mut self, v: ::std::vec::Vec<i32>) {
        self.roles_sid = v;
    }

    // Mutable pointer to the field.
    pub fn mut_roles_sid<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.roles_sid
    }

    // Take field
    pub fn take_roles_sid(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.roles_sid, ::std::vec::Vec::new())
    }

    pub fn get_roles_sid<'a>(&'a self) -> &'a [i32] {
        &self.roles_sid
    }

    // repeated sint64 memids = 9;

    pub fn clear_memids(&mut self) {
        self.memids.clear();
    }

    // Param is passed by value, moved
    pub fn set_memids(&mut self, v: ::std::vec::Vec<i64>) {
        self.memids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_memids<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.memids
    }

    // Take field
    pub fn take_memids(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.memids, ::std::vec::Vec::new())
    }

    pub fn get_memids<'a>(&'a self) -> &'a [i64] {
        &self.memids
    }

    // repeated .OSMPBF.Relation.MemberType types = 10;

    pub fn clear_types(&mut self) {
        self.types.clear();
    }

    // Param is passed by value, moved
    pub fn set_types(&mut self, v: ::std::vec::Vec<Relation_MemberType>) {
        self.types = v;
    }

    // Mutable pointer to the field.
    pub fn mut_types<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<Relation_MemberType> {
        &mut self.types
    }

    // Take field
    pub fn take_types(&mut self) -> ::std::vec::Vec<Relation_MemberType> {
        ::std::mem::replace(&mut self.types, ::std::vec::Vec::new())
    }

    pub fn get_types<'a>(&'a self) -> &'a [Relation_MemberType] {
        &self.types
    }
}

impl ::protobuf::Message for Relation {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.keys));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.vals));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.info));
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.roles_sid));
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_sint64_into(wire_type, is, &mut self.memids));
                },
                10 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.types));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.keys.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(2, &self.keys);
        };
        if !self.vals.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(3, &self.vals);
        };
        for value in self.info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.roles_sid.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(8, &self.roles_sid);
        };
        if !self.memids.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(9, &self.memids);
        };
        if !self.types.is_empty() {
            my_size += ::protobuf::rt::vec_packed_enum_size(10, &self.types);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_int64(1, v));
        };
        if !self.keys.is_empty() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.keys)));
            for v in self.keys.iter() {
                try!(os.write_uint32_no_tag(*v));
            };
        };
        if !self.vals.is_empty() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.vals)));
            for v in self.vals.iter() {
                try!(os.write_uint32_no_tag(*v));
            };
        };
        if let Some(v) = self.info.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if !self.roles_sid.is_empty() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.roles_sid)));
            for v in self.roles_sid.iter() {
                try!(os.write_int32_no_tag(*v));
            };
        };
        if !self.memids.is_empty() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.memids)));
            for v in self.memids.iter() {
                try!(os.write_sint64_no_tag(*v));
            };
        };
        if !self.types.is_empty() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_enum_data_size(&self.types)));
            for v in self.types.iter() {
                try!(os.write_enum_no_tag(v.value()));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Relation>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Relation {
    fn new() -> Relation {
        Relation::new()
    }
}

impl ::protobuf::Clear for Relation {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_keys();
        self.clear_vals();
        self.clear_info();
        self.clear_roles_sid();
        self.clear_memids();
        self.clear_types();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Relation {
    fn eq(&self, other: &Relation) -> bool {
        self.id == other.id &&
        self.keys == other.keys &&
        self.vals == other.vals &&
        self.info == other.info &&
        self.roles_sid == other.roles_sid &&
        self.memids == other.memids &&
        self.types == other.types &&
        self.unknown_fields == other.unknown_fields
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Relation_MemberType {
    NODE = 0,
    WAY = 1,
    RELATION = 2,
}

impl ::protobuf::ProtobufEnum for Relation_MemberType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Relation_MemberType> {
        match value {
            0 => ::std::option::Option::Some(Relation_MemberType::NODE),
            1 => ::std::option::Option::Some(Relation_MemberType::WAY),
            2 => ::std::option::Option::Some(Relation_MemberType::RELATION),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Relation_MemberType] = &[
            Relation_MemberType::NODE,
            Relation_MemberType::WAY,
            Relation_MemberType::RELATION,
        ];
        values
    }
}

impl ::std::marker::Copy for Relation_MemberType {
}

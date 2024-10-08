// This file is generated by rust-protobuf 3.6.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `PlayerDataNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_6_0;

// @@protoc_insertion_point(message:PlayerDataNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerDataNotify {
    // message fields
    // @@protoc_insertion_point(field:PlayerDataNotify.nick_name)
    pub nick_name: ::std::string::String,
    // @@protoc_insertion_point(field:PlayerDataNotify.is_first_login_today)
    pub is_first_login_today: bool,
    // @@protoc_insertion_point(field:PlayerDataNotify.region_id)
    pub region_id: u32,
    // @@protoc_insertion_point(field:PlayerDataNotify.prop_map)
    pub prop_map: ::std::collections::HashMap<u32, super::PropValue::PropValue>,
    // @@protoc_insertion_point(field:PlayerDataNotify.server_time)
    pub server_time: u64,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerDataNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerDataNotify {
    fn default() -> &'a PlayerDataNotify {
        <PlayerDataNotify as ::protobuf::Message>::default_instance()
    }
}

impl PlayerDataNotify {
    pub fn new() -> PlayerDataNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "nick_name",
            |m: &PlayerDataNotify| { &m.nick_name },
            |m: &mut PlayerDataNotify| { &mut m.nick_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_first_login_today",
            |m: &PlayerDataNotify| { &m.is_first_login_today },
            |m: &mut PlayerDataNotify| { &mut m.is_first_login_today },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "region_id",
            |m: &PlayerDataNotify| { &m.region_id },
            |m: &mut PlayerDataNotify| { &mut m.region_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "prop_map",
            |m: &PlayerDataNotify| { &m.prop_map },
            |m: &mut PlayerDataNotify| { &mut m.prop_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "server_time",
            |m: &PlayerDataNotify| { &m.server_time },
            |m: &mut PlayerDataNotify| { &mut m.server_time },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerDataNotify>(
            "PlayerDataNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerDataNotify {
    const NAME: &'static str = "PlayerDataNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.nick_name = is.read_string()?;
                },
                16 => {
                    self.is_first_login_today = is.read_bool()?;
                },
                40 => {
                    self.region_id = is.read_uint32()?;
                },
                66 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.prop_map.insert(key, value);
                },
                88 => {
                    self.server_time = is.read_uint64()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.nick_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.nick_name);
        }
        if self.is_first_login_today != false {
            my_size += 1 + 1;
        }
        if self.region_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.region_id);
        }
        for (k, v) in &self.prop_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.server_time != 0 {
            my_size += ::protobuf::rt::uint64_size(11, self.server_time);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.nick_name.is_empty() {
            os.write_string(1, &self.nick_name)?;
        }
        if self.is_first_login_today != false {
            os.write_bool(2, self.is_first_login_today)?;
        }
        if self.region_id != 0 {
            os.write_uint32(5, self.region_id)?;
        }
        for (k, v) in &self.prop_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(66)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.server_time != 0 {
            os.write_uint64(11, self.server_time)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PlayerDataNotify {
        PlayerDataNotify::new()
    }

    fn clear(&mut self) {
        self.nick_name.clear();
        self.is_first_login_today = false;
        self.region_id = 0;
        self.prop_map.clear();
        self.server_time = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerDataNotify {
        static instance: ::protobuf::rt::Lazy<PlayerDataNotify> = ::protobuf::rt::Lazy::new();
        instance.get(PlayerDataNotify::new)
    }
}

impl ::protobuf::MessageFull for PlayerDataNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerDataNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerDataNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerDataNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16PlayerDataNotify.proto\x1a\x0fPropValue.proto\"\xa1\x02\n\x10Playe\
    rDataNotify\x12\x1b\n\tnick_name\x18\x01\x20\x01(\tR\x08nickName\x12/\n\
    \x14is_first_login_today\x18\x02\x20\x01(\x08R\x11isFirstLoginToday\x12\
    \x1b\n\tregion_id\x18\x05\x20\x01(\rR\x08regionId\x129\n\x08prop_map\x18\
    \x08\x20\x03(\x0b2\x1e.PlayerDataNotify.PropMapEntryR\x07propMap\x12\x1f\
    \n\x0bserver_time\x18\x0b\x20\x01(\x04R\nserverTime\x1aF\n\x0cPropMapEnt\
    ry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x20\n\x05value\x18\
    \x02\x20\x01(\x0b2\n.PropValueR\x05value:\x028\x01B\x1b\n\x19emu.grasscu\
    tter.net.protob\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::PropValue::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PlayerDataNotify::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}

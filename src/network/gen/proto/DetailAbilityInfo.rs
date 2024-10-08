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

//! Generated file from `DetailAbilityInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_6_0;

// @@protoc_insertion_point(message:DetailAbilityInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DetailAbilityInfo {
    // message fields
    // @@protoc_insertion_point(field:DetailAbilityInfo.instanced_ability_id)
    pub instanced_ability_id: u32,
    // @@protoc_insertion_point(field:DetailAbilityInfo.caster_id)
    pub caster_id: u32,
    // @@protoc_insertion_point(field:DetailAbilityInfo.modifier_local_id)
    pub modifier_local_id: i32,
    // @@protoc_insertion_point(field:DetailAbilityInfo.local_id)
    pub local_id: i32,
    // @@protoc_insertion_point(field:DetailAbilityInfo.parent_ability_name)
    pub parent_ability_name: ::protobuf::MessageField<super::AbilityString::AbilityString>,
    // @@protoc_insertion_point(field:DetailAbilityInfo.instanced_modifier_id)
    pub instanced_modifier_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DetailAbilityInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DetailAbilityInfo {
    fn default() -> &'a DetailAbilityInfo {
        <DetailAbilityInfo as ::protobuf::Message>::default_instance()
    }
}

impl DetailAbilityInfo {
    pub fn new() -> DetailAbilityInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "instanced_ability_id",
            |m: &DetailAbilityInfo| { &m.instanced_ability_id },
            |m: &mut DetailAbilityInfo| { &mut m.instanced_ability_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "caster_id",
            |m: &DetailAbilityInfo| { &m.caster_id },
            |m: &mut DetailAbilityInfo| { &mut m.caster_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "modifier_local_id",
            |m: &DetailAbilityInfo| { &m.modifier_local_id },
            |m: &mut DetailAbilityInfo| { &mut m.modifier_local_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "local_id",
            |m: &DetailAbilityInfo| { &m.local_id },
            |m: &mut DetailAbilityInfo| { &mut m.local_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AbilityString::AbilityString>(
            "parent_ability_name",
            |m: &DetailAbilityInfo| { &m.parent_ability_name },
            |m: &mut DetailAbilityInfo| { &mut m.parent_ability_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "instanced_modifier_id",
            |m: &DetailAbilityInfo| { &m.instanced_modifier_id },
            |m: &mut DetailAbilityInfo| { &mut m.instanced_modifier_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DetailAbilityInfo>(
            "DetailAbilityInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DetailAbilityInfo {
    const NAME: &'static str = "DetailAbilityInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.instanced_ability_id = is.read_uint32()?;
                },
                24 => {
                    self.caster_id = is.read_uint32()?;
                },
                48 => {
                    self.modifier_local_id = is.read_int32()?;
                },
                64 => {
                    self.local_id = is.read_int32()?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.parent_ability_name)?;
                },
                96 => {
                    self.instanced_modifier_id = is.read_uint32()?;
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
        if self.instanced_ability_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.instanced_ability_id);
        }
        if self.caster_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.caster_id);
        }
        if self.modifier_local_id != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.modifier_local_id);
        }
        if self.local_id != 0 {
            my_size += ::protobuf::rt::int32_size(8, self.local_id);
        }
        if let Some(v) = self.parent_ability_name.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.instanced_modifier_id != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.instanced_modifier_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.instanced_ability_id != 0 {
            os.write_uint32(1, self.instanced_ability_id)?;
        }
        if self.caster_id != 0 {
            os.write_uint32(3, self.caster_id)?;
        }
        if self.modifier_local_id != 0 {
            os.write_int32(6, self.modifier_local_id)?;
        }
        if self.local_id != 0 {
            os.write_int32(8, self.local_id)?;
        }
        if let Some(v) = self.parent_ability_name.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if self.instanced_modifier_id != 0 {
            os.write_uint32(12, self.instanced_modifier_id)?;
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

    fn new() -> DetailAbilityInfo {
        DetailAbilityInfo::new()
    }

    fn clear(&mut self) {
        self.instanced_ability_id = 0;
        self.caster_id = 0;
        self.modifier_local_id = 0;
        self.local_id = 0;
        self.parent_ability_name.clear();
        self.instanced_modifier_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DetailAbilityInfo {
        static instance: DetailAbilityInfo = DetailAbilityInfo {
            instanced_ability_id: 0,
            caster_id: 0,
            modifier_local_id: 0,
            local_id: 0,
            parent_ability_name: ::protobuf::MessageField::none(),
            instanced_modifier_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DetailAbilityInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DetailAbilityInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DetailAbilityInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DetailAbilityInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17DetailAbilityInfo.proto\x1a\x13AbilityString.proto\"\x9d\x02\n\x11\
    DetailAbilityInfo\x120\n\x14instanced_ability_id\x18\x01\x20\x01(\rR\x12\
    instancedAbilityId\x12\x1b\n\tcaster_id\x18\x03\x20\x01(\rR\x08casterId\
    \x12*\n\x11modifier_local_id\x18\x06\x20\x01(\x05R\x0fmodifierLocalId\
    \x12\x19\n\x08local_id\x18\x08\x20\x01(\x05R\x07localId\x12>\n\x13parent\
    _ability_name\x18\n\x20\x01(\x0b2\x0e.AbilityStringR\x11parentAbilityNam\
    e\x122\n\x15instanced_modifier_id\x18\x0c\x20\x01(\rR\x13instancedModifi\
    erIdB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::AbilityString::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DetailAbilityInfo::generated_message_descriptor_data());
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

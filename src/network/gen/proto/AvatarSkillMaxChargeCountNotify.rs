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

//! Generated file from `AvatarSkillMaxChargeCountNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_6_0;

// @@protoc_insertion_point(message:AvatarSkillMaxChargeCountNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AvatarSkillMaxChargeCountNotify {
    // message fields
    // @@protoc_insertion_point(field:AvatarSkillMaxChargeCountNotify.skill_id)
    pub skill_id: u32,
    // @@protoc_insertion_point(field:AvatarSkillMaxChargeCountNotify.max_charge_count)
    pub max_charge_count: u32,
    // @@protoc_insertion_point(field:AvatarSkillMaxChargeCountNotify.avatar_guid)
    pub avatar_guid: u64,
    // special fields
    // @@protoc_insertion_point(special_field:AvatarSkillMaxChargeCountNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AvatarSkillMaxChargeCountNotify {
    fn default() -> &'a AvatarSkillMaxChargeCountNotify {
        <AvatarSkillMaxChargeCountNotify as ::protobuf::Message>::default_instance()
    }
}

impl AvatarSkillMaxChargeCountNotify {
    pub fn new() -> AvatarSkillMaxChargeCountNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "skill_id",
            |m: &AvatarSkillMaxChargeCountNotify| { &m.skill_id },
            |m: &mut AvatarSkillMaxChargeCountNotify| { &mut m.skill_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "max_charge_count",
            |m: &AvatarSkillMaxChargeCountNotify| { &m.max_charge_count },
            |m: &mut AvatarSkillMaxChargeCountNotify| { &mut m.max_charge_count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_guid",
            |m: &AvatarSkillMaxChargeCountNotify| { &m.avatar_guid },
            |m: &mut AvatarSkillMaxChargeCountNotify| { &mut m.avatar_guid },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AvatarSkillMaxChargeCountNotify>(
            "AvatarSkillMaxChargeCountNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AvatarSkillMaxChargeCountNotify {
    const NAME: &'static str = "AvatarSkillMaxChargeCountNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.skill_id = is.read_uint32()?;
                },
                88 => {
                    self.max_charge_count = is.read_uint32()?;
                },
                48 => {
                    self.avatar_guid = is.read_uint64()?;
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
        if self.skill_id != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.skill_id);
        }
        if self.max_charge_count != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.max_charge_count);
        }
        if self.avatar_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(6, self.avatar_guid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.skill_id != 0 {
            os.write_uint32(12, self.skill_id)?;
        }
        if self.max_charge_count != 0 {
            os.write_uint32(11, self.max_charge_count)?;
        }
        if self.avatar_guid != 0 {
            os.write_uint64(6, self.avatar_guid)?;
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

    fn new() -> AvatarSkillMaxChargeCountNotify {
        AvatarSkillMaxChargeCountNotify::new()
    }

    fn clear(&mut self) {
        self.skill_id = 0;
        self.max_charge_count = 0;
        self.avatar_guid = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AvatarSkillMaxChargeCountNotify {
        static instance: AvatarSkillMaxChargeCountNotify = AvatarSkillMaxChargeCountNotify {
            skill_id: 0,
            max_charge_count: 0,
            avatar_guid: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AvatarSkillMaxChargeCountNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AvatarSkillMaxChargeCountNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AvatarSkillMaxChargeCountNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AvatarSkillMaxChargeCountNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%AvatarSkillMaxChargeCountNotify.proto\"\x87\x01\n\x1fAvatarSkillMaxCh\
    argeCountNotify\x12\x19\n\x08skill_id\x18\x0c\x20\x01(\rR\x07skillId\x12\
    (\n\x10max_charge_count\x18\x0b\x20\x01(\rR\x0emaxChargeCount\x12\x1f\n\
    \x0bavatar_guid\x18\x06\x20\x01(\x04R\navatarGuidB\x1b\n\x19emu.grasscut\
    ter.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AvatarSkillMaxChargeCountNotify::generated_message_descriptor_data());
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

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

//! Generated file from `SocialShowAvatarInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_6_0;

// @@protoc_insertion_point(message:SocialShowAvatarInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SocialShowAvatarInfo {
    // message fields
    // @@protoc_insertion_point(field:SocialShowAvatarInfo.avatar_id)
    pub avatar_id: u32,
    // @@protoc_insertion_point(field:SocialShowAvatarInfo.level)
    pub level: u32,
    // @@protoc_insertion_point(field:SocialShowAvatarInfo.costume_id)
    pub costume_id: u32,
    // @@protoc_insertion_point(field:SocialShowAvatarInfo.AEPNHMCDBFL)
    pub AEPNHMCDBFL: u32,
    // @@protoc_insertion_point(field:SocialShowAvatarInfo.CMAFEKGPACG)
    pub CMAFEKGPACG: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SocialShowAvatarInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SocialShowAvatarInfo {
    fn default() -> &'a SocialShowAvatarInfo {
        <SocialShowAvatarInfo as ::protobuf::Message>::default_instance()
    }
}

impl SocialShowAvatarInfo {
    pub fn new() -> SocialShowAvatarInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id",
            |m: &SocialShowAvatarInfo| { &m.avatar_id },
            |m: &mut SocialShowAvatarInfo| { &mut m.avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &SocialShowAvatarInfo| { &m.level },
            |m: &mut SocialShowAvatarInfo| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "costume_id",
            |m: &SocialShowAvatarInfo| { &m.costume_id },
            |m: &mut SocialShowAvatarInfo| { &mut m.costume_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AEPNHMCDBFL",
            |m: &SocialShowAvatarInfo| { &m.AEPNHMCDBFL },
            |m: &mut SocialShowAvatarInfo| { &mut m.AEPNHMCDBFL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CMAFEKGPACG",
            |m: &SocialShowAvatarInfo| { &m.CMAFEKGPACG },
            |m: &mut SocialShowAvatarInfo| { &mut m.CMAFEKGPACG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SocialShowAvatarInfo>(
            "SocialShowAvatarInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SocialShowAvatarInfo {
    const NAME: &'static str = "SocialShowAvatarInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.avatar_id = is.read_uint32()?;
                },
                16 => {
                    self.level = is.read_uint32()?;
                },
                24 => {
                    self.costume_id = is.read_uint32()?;
                },
                32 => {
                    self.AEPNHMCDBFL = is.read_uint32()?;
                },
                40 => {
                    self.CMAFEKGPACG = is.read_uint32()?;
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
        if self.avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.avatar_id);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.level);
        }
        if self.costume_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.costume_id);
        }
        if self.AEPNHMCDBFL != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.AEPNHMCDBFL);
        }
        if self.CMAFEKGPACG != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.CMAFEKGPACG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.avatar_id != 0 {
            os.write_uint32(1, self.avatar_id)?;
        }
        if self.level != 0 {
            os.write_uint32(2, self.level)?;
        }
        if self.costume_id != 0 {
            os.write_uint32(3, self.costume_id)?;
        }
        if self.AEPNHMCDBFL != 0 {
            os.write_uint32(4, self.AEPNHMCDBFL)?;
        }
        if self.CMAFEKGPACG != 0 {
            os.write_uint32(5, self.CMAFEKGPACG)?;
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

    fn new() -> SocialShowAvatarInfo {
        SocialShowAvatarInfo::new()
    }

    fn clear(&mut self) {
        self.avatar_id = 0;
        self.level = 0;
        self.costume_id = 0;
        self.AEPNHMCDBFL = 0;
        self.CMAFEKGPACG = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SocialShowAvatarInfo {
        static instance: SocialShowAvatarInfo = SocialShowAvatarInfo {
            avatar_id: 0,
            level: 0,
            costume_id: 0,
            AEPNHMCDBFL: 0,
            CMAFEKGPACG: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SocialShowAvatarInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SocialShowAvatarInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SocialShowAvatarInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SocialShowAvatarInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aSocialShowAvatarInfo.proto\"\xac\x01\n\x14SocialShowAvatarInfo\x12\
    \x1b\n\tavatar_id\x18\x01\x20\x01(\rR\x08avatarId\x12\x14\n\x05level\x18\
    \x02\x20\x01(\rR\x05level\x12\x1d\n\ncostume_id\x18\x03\x20\x01(\rR\tcos\
    tumeId\x12\x20\n\x0bAEPNHMCDBFL\x18\x04\x20\x01(\rR\x0bAEPNHMCDBFL\x12\
    \x20\n\x0bCMAFEKGPACG\x18\x05\x20\x01(\rR\x0bCMAFEKGPACGB\x1b\n\x19emu.g\
    rasscutter.net.protob\x06proto3\
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
            messages.push(SocialShowAvatarInfo::generated_message_descriptor_data());
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

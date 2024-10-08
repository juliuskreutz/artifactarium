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

//! Generated file from `CustomDungeonResultInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_6_0;

// @@protoc_insertion_point(message:CustomDungeonResultInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CustomDungeonResultInfo {
    // message fields
    // @@protoc_insertion_point(field:CustomDungeonResultInfo.child_challenge_list)
    pub child_challenge_list: ::std::vec::Vec<super::ChallengeBrief::ChallengeBrief>,
    // @@protoc_insertion_point(field:CustomDungeonResultInfo.got_coin_num)
    pub got_coin_num: u32,
    // @@protoc_insertion_point(field:CustomDungeonResultInfo.FOLIKAAIKIE)
    pub FOLIKAAIKIE: bool,
    // @@protoc_insertion_point(field:CustomDungeonResultInfo.GBLHFAEONKM)
    pub GBLHFAEONKM: bool,
    // @@protoc_insertion_point(field:CustomDungeonResultInfo.finish_type)
    pub finish_type: ::protobuf::EnumOrUnknown<super::CustomDungeonFinishType::CustomDungeonFinishType>,
    // @@protoc_insertion_point(field:CustomDungeonResultInfo.time_cost)
    pub time_cost: u32,
    // @@protoc_insertion_point(field:CustomDungeonResultInfo.LHNFIIEJBEM)
    pub LHNFIIEJBEM: bool,
    // @@protoc_insertion_point(field:CustomDungeonResultInfo.dungeon_guid)
    pub dungeon_guid: u64,
    // special fields
    // @@protoc_insertion_point(special_field:CustomDungeonResultInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CustomDungeonResultInfo {
    fn default() -> &'a CustomDungeonResultInfo {
        <CustomDungeonResultInfo as ::protobuf::Message>::default_instance()
    }
}

impl CustomDungeonResultInfo {
    pub fn new() -> CustomDungeonResultInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "child_challenge_list",
            |m: &CustomDungeonResultInfo| { &m.child_challenge_list },
            |m: &mut CustomDungeonResultInfo| { &mut m.child_challenge_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "got_coin_num",
            |m: &CustomDungeonResultInfo| { &m.got_coin_num },
            |m: &mut CustomDungeonResultInfo| { &mut m.got_coin_num },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FOLIKAAIKIE",
            |m: &CustomDungeonResultInfo| { &m.FOLIKAAIKIE },
            |m: &mut CustomDungeonResultInfo| { &mut m.FOLIKAAIKIE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GBLHFAEONKM",
            |m: &CustomDungeonResultInfo| { &m.GBLHFAEONKM },
            |m: &mut CustomDungeonResultInfo| { &mut m.GBLHFAEONKM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "finish_type",
            |m: &CustomDungeonResultInfo| { &m.finish_type },
            |m: &mut CustomDungeonResultInfo| { &mut m.finish_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "time_cost",
            |m: &CustomDungeonResultInfo| { &m.time_cost },
            |m: &mut CustomDungeonResultInfo| { &mut m.time_cost },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LHNFIIEJBEM",
            |m: &CustomDungeonResultInfo| { &m.LHNFIIEJBEM },
            |m: &mut CustomDungeonResultInfo| { &mut m.LHNFIIEJBEM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dungeon_guid",
            |m: &CustomDungeonResultInfo| { &m.dungeon_guid },
            |m: &mut CustomDungeonResultInfo| { &mut m.dungeon_guid },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CustomDungeonResultInfo>(
            "CustomDungeonResultInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CustomDungeonResultInfo {
    const NAME: &'static str = "CustomDungeonResultInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    self.child_challenge_list.push(is.read_message()?);
                },
                24 => {
                    self.got_coin_num = is.read_uint32()?;
                },
                40 => {
                    self.FOLIKAAIKIE = is.read_bool()?;
                },
                48 => {
                    self.GBLHFAEONKM = is.read_bool()?;
                },
                80 => {
                    self.finish_type = is.read_enum_or_unknown()?;
                },
                88 => {
                    self.time_cost = is.read_uint32()?;
                },
                112 => {
                    self.LHNFIIEJBEM = is.read_bool()?;
                },
                120 => {
                    self.dungeon_guid = is.read_uint64()?;
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
        for value in &self.child_challenge_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.got_coin_num != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.got_coin_num);
        }
        if self.FOLIKAAIKIE != false {
            my_size += 1 + 1;
        }
        if self.GBLHFAEONKM != false {
            my_size += 1 + 1;
        }
        if self.finish_type != ::protobuf::EnumOrUnknown::new(super::CustomDungeonFinishType::CustomDungeonFinishType::CUSTOM_DUNGEON_FINISH_PLAY_NORMAL) {
            my_size += ::protobuf::rt::int32_size(10, self.finish_type.value());
        }
        if self.time_cost != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.time_cost);
        }
        if self.LHNFIIEJBEM != false {
            my_size += 1 + 1;
        }
        if self.dungeon_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(15, self.dungeon_guid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.child_challenge_list {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.got_coin_num != 0 {
            os.write_uint32(3, self.got_coin_num)?;
        }
        if self.FOLIKAAIKIE != false {
            os.write_bool(5, self.FOLIKAAIKIE)?;
        }
        if self.GBLHFAEONKM != false {
            os.write_bool(6, self.GBLHFAEONKM)?;
        }
        if self.finish_type != ::protobuf::EnumOrUnknown::new(super::CustomDungeonFinishType::CustomDungeonFinishType::CUSTOM_DUNGEON_FINISH_PLAY_NORMAL) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.finish_type))?;
        }
        if self.time_cost != 0 {
            os.write_uint32(11, self.time_cost)?;
        }
        if self.LHNFIIEJBEM != false {
            os.write_bool(14, self.LHNFIIEJBEM)?;
        }
        if self.dungeon_guid != 0 {
            os.write_uint64(15, self.dungeon_guid)?;
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

    fn new() -> CustomDungeonResultInfo {
        CustomDungeonResultInfo::new()
    }

    fn clear(&mut self) {
        self.child_challenge_list.clear();
        self.got_coin_num = 0;
        self.FOLIKAAIKIE = false;
        self.GBLHFAEONKM = false;
        self.finish_type = ::protobuf::EnumOrUnknown::new(super::CustomDungeonFinishType::CustomDungeonFinishType::CUSTOM_DUNGEON_FINISH_PLAY_NORMAL);
        self.time_cost = 0;
        self.LHNFIIEJBEM = false;
        self.dungeon_guid = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CustomDungeonResultInfo {
        static instance: CustomDungeonResultInfo = CustomDungeonResultInfo {
            child_challenge_list: ::std::vec::Vec::new(),
            got_coin_num: 0,
            FOLIKAAIKIE: false,
            GBLHFAEONKM: false,
            finish_type: ::protobuf::EnumOrUnknown::from_i32(0),
            time_cost: 0,
            LHNFIIEJBEM: false,
            dungeon_guid: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CustomDungeonResultInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CustomDungeonResultInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CustomDungeonResultInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CustomDungeonResultInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dCustomDungeonResultInfo.proto\x1a\x14ChallengeBrief.proto\x1a\x1dC\
    ustomDungeonFinishType.proto\"\xdf\x02\n\x17CustomDungeonResultInfo\x12A\
    \n\x14child_challenge_list\x18\x02\x20\x03(\x0b2\x0f.ChallengeBriefR\x12\
    childChallengeList\x12\x20\n\x0cgot_coin_num\x18\x03\x20\x01(\rR\ngotCoi\
    nNum\x12\x20\n\x0bFOLIKAAIKIE\x18\x05\x20\x01(\x08R\x0bFOLIKAAIKIE\x12\
    \x20\n\x0bGBLHFAEONKM\x18\x06\x20\x01(\x08R\x0bGBLHFAEONKM\x129\n\x0bfin\
    ish_type\x18\n\x20\x01(\x0e2\x18.CustomDungeonFinishTypeR\nfinishType\
    \x12\x1b\n\ttime_cost\x18\x0b\x20\x01(\rR\x08timeCost\x12\x20\n\x0bLHNFI\
    IEJBEM\x18\x0e\x20\x01(\x08R\x0bLHNFIIEJBEM\x12!\n\x0cdungeon_guid\x18\
    \x0f\x20\x01(\x04R\x0bdungeonGuidB\x1b\n\x19emu.grasscutter.net.protob\
    \x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::ChallengeBrief::file_descriptor().clone());
            deps.push(super::CustomDungeonFinishType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CustomDungeonResultInfo::generated_message_descriptor_data());
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

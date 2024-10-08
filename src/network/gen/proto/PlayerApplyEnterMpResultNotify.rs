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

//! Generated file from `PlayerApplyEnterMpResultNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_6_0;

// @@protoc_insertion_point(message:PlayerApplyEnterMpResultNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerApplyEnterMpResultNotify {
    // message fields
    // @@protoc_insertion_point(field:PlayerApplyEnterMpResultNotify.reason)
    pub reason: ::protobuf::EnumOrUnknown<player_apply_enter_mp_result_notify::Reason>,
    // @@protoc_insertion_point(field:PlayerApplyEnterMpResultNotify.target_nickname)
    pub target_nickname: ::std::string::String,
    // @@protoc_insertion_point(field:PlayerApplyEnterMpResultNotify.target_uid)
    pub target_uid: u32,
    // @@protoc_insertion_point(field:PlayerApplyEnterMpResultNotify.is_agreed)
    pub is_agreed: bool,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerApplyEnterMpResultNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerApplyEnterMpResultNotify {
    fn default() -> &'a PlayerApplyEnterMpResultNotify {
        <PlayerApplyEnterMpResultNotify as ::protobuf::Message>::default_instance()
    }
}

impl PlayerApplyEnterMpResultNotify {
    pub fn new() -> PlayerApplyEnterMpResultNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "reason",
            |m: &PlayerApplyEnterMpResultNotify| { &m.reason },
            |m: &mut PlayerApplyEnterMpResultNotify| { &mut m.reason },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_nickname",
            |m: &PlayerApplyEnterMpResultNotify| { &m.target_nickname },
            |m: &mut PlayerApplyEnterMpResultNotify| { &mut m.target_nickname },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_uid",
            |m: &PlayerApplyEnterMpResultNotify| { &m.target_uid },
            |m: &mut PlayerApplyEnterMpResultNotify| { &mut m.target_uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_agreed",
            |m: &PlayerApplyEnterMpResultNotify| { &m.is_agreed },
            |m: &mut PlayerApplyEnterMpResultNotify| { &mut m.is_agreed },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerApplyEnterMpResultNotify>(
            "PlayerApplyEnterMpResultNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerApplyEnterMpResultNotify {
    const NAME: &'static str = "PlayerApplyEnterMpResultNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.reason = is.read_enum_or_unknown()?;
                },
                66 => {
                    self.target_nickname = is.read_string()?;
                },
                8 => {
                    self.target_uid = is.read_uint32()?;
                },
                32 => {
                    self.is_agreed = is.read_bool()?;
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
        if self.reason != ::protobuf::EnumOrUnknown::new(player_apply_enter_mp_result_notify::Reason::REASON_PLAYER_JUDGE) {
            my_size += ::protobuf::rt::int32_size(6, self.reason.value());
        }
        if !self.target_nickname.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.target_nickname);
        }
        if self.target_uid != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.target_uid);
        }
        if self.is_agreed != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.reason != ::protobuf::EnumOrUnknown::new(player_apply_enter_mp_result_notify::Reason::REASON_PLAYER_JUDGE) {
            os.write_enum(6, ::protobuf::EnumOrUnknown::value(&self.reason))?;
        }
        if !self.target_nickname.is_empty() {
            os.write_string(8, &self.target_nickname)?;
        }
        if self.target_uid != 0 {
            os.write_uint32(1, self.target_uid)?;
        }
        if self.is_agreed != false {
            os.write_bool(4, self.is_agreed)?;
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

    fn new() -> PlayerApplyEnterMpResultNotify {
        PlayerApplyEnterMpResultNotify::new()
    }

    fn clear(&mut self) {
        self.reason = ::protobuf::EnumOrUnknown::new(player_apply_enter_mp_result_notify::Reason::REASON_PLAYER_JUDGE);
        self.target_nickname.clear();
        self.target_uid = 0;
        self.is_agreed = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerApplyEnterMpResultNotify {
        static instance: PlayerApplyEnterMpResultNotify = PlayerApplyEnterMpResultNotify {
            reason: ::protobuf::EnumOrUnknown::from_i32(0),
            target_nickname: ::std::string::String::new(),
            target_uid: 0,
            is_agreed: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerApplyEnterMpResultNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerApplyEnterMpResultNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerApplyEnterMpResultNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerApplyEnterMpResultNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `PlayerApplyEnterMpResultNotify`
pub mod player_apply_enter_mp_result_notify {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:PlayerApplyEnterMpResultNotify.Reason)
    pub enum Reason {
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterMpResultNotify.Reason.REASON_PLAYER_JUDGE)
        REASON_PLAYER_JUDGE = 0,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterMpResultNotify.Reason.REASON_SCENE_CANNOT_ENTER)
        REASON_SCENE_CANNOT_ENTER = 1,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterMpResultNotify.Reason.REASON_PLAYER_CANNOT_ENTER_MP)
        REASON_PLAYER_CANNOT_ENTER_MP = 2,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterMpResultNotify.Reason.REASON_SYSTEM_JUDGE)
        REASON_SYSTEM_JUDGE = 3,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterMpResultNotify.Reason.REASON_ALLOW_ENTER_PLAYER_FULL)
        REASON_ALLOW_ENTER_PLAYER_FULL = 4,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterMpResultNotify.Reason.REASON_WORLD_LEVEL_LOWER_THAN_HOST)
        REASON_WORLD_LEVEL_LOWER_THAN_HOST = 5,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterMpResultNotify.Reason.REASON_HOST_IN_MATCH)
        REASON_HOST_IN_MATCH = 6,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterMpResultNotify.Reason.REASON_PLAYER_IN_BLACKLIST)
        REASON_PLAYER_IN_BLACKLIST = 7,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterMpResultNotify.Reason.REASON_PS_PLAYER_NOT_ACCEPT_OTHERS)
        REASON_PS_PLAYER_NOT_ACCEPT_OTHERS = 8,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterMpResultNotify.Reason.REASON_HOST_IS_BLOCKED)
        REASON_HOST_IS_BLOCKED = 9,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterMpResultNotify.Reason.REASON_OTHER_DATA_VERSION_NOT_LATEST)
        REASON_OTHER_DATA_VERSION_NOT_LATEST = 10,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterMpResultNotify.Reason.REASON_DATA_VERSION_NOT_LATEST)
        REASON_DATA_VERSION_NOT_LATEST = 11,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterMpResultNotify.Reason.REASON_PLAYER_NOT_IN_PLAYER_WORLD)
        REASON_PLAYER_NOT_IN_PLAYER_WORLD = 12,
        // @@protoc_insertion_point(enum_value:PlayerApplyEnterMpResultNotify.Reason.REASON_MAX_PLAYER)
        REASON_MAX_PLAYER = 13,
    }

    impl ::protobuf::Enum for Reason {
        const NAME: &'static str = "Reason";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<Reason> {
            match value {
                0 => ::std::option::Option::Some(Reason::REASON_PLAYER_JUDGE),
                1 => ::std::option::Option::Some(Reason::REASON_SCENE_CANNOT_ENTER),
                2 => ::std::option::Option::Some(Reason::REASON_PLAYER_CANNOT_ENTER_MP),
                3 => ::std::option::Option::Some(Reason::REASON_SYSTEM_JUDGE),
                4 => ::std::option::Option::Some(Reason::REASON_ALLOW_ENTER_PLAYER_FULL),
                5 => ::std::option::Option::Some(Reason::REASON_WORLD_LEVEL_LOWER_THAN_HOST),
                6 => ::std::option::Option::Some(Reason::REASON_HOST_IN_MATCH),
                7 => ::std::option::Option::Some(Reason::REASON_PLAYER_IN_BLACKLIST),
                8 => ::std::option::Option::Some(Reason::REASON_PS_PLAYER_NOT_ACCEPT_OTHERS),
                9 => ::std::option::Option::Some(Reason::REASON_HOST_IS_BLOCKED),
                10 => ::std::option::Option::Some(Reason::REASON_OTHER_DATA_VERSION_NOT_LATEST),
                11 => ::std::option::Option::Some(Reason::REASON_DATA_VERSION_NOT_LATEST),
                12 => ::std::option::Option::Some(Reason::REASON_PLAYER_NOT_IN_PLAYER_WORLD),
                13 => ::std::option::Option::Some(Reason::REASON_MAX_PLAYER),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<Reason> {
            match str {
                "REASON_PLAYER_JUDGE" => ::std::option::Option::Some(Reason::REASON_PLAYER_JUDGE),
                "REASON_SCENE_CANNOT_ENTER" => ::std::option::Option::Some(Reason::REASON_SCENE_CANNOT_ENTER),
                "REASON_PLAYER_CANNOT_ENTER_MP" => ::std::option::Option::Some(Reason::REASON_PLAYER_CANNOT_ENTER_MP),
                "REASON_SYSTEM_JUDGE" => ::std::option::Option::Some(Reason::REASON_SYSTEM_JUDGE),
                "REASON_ALLOW_ENTER_PLAYER_FULL" => ::std::option::Option::Some(Reason::REASON_ALLOW_ENTER_PLAYER_FULL),
                "REASON_WORLD_LEVEL_LOWER_THAN_HOST" => ::std::option::Option::Some(Reason::REASON_WORLD_LEVEL_LOWER_THAN_HOST),
                "REASON_HOST_IN_MATCH" => ::std::option::Option::Some(Reason::REASON_HOST_IN_MATCH),
                "REASON_PLAYER_IN_BLACKLIST" => ::std::option::Option::Some(Reason::REASON_PLAYER_IN_BLACKLIST),
                "REASON_PS_PLAYER_NOT_ACCEPT_OTHERS" => ::std::option::Option::Some(Reason::REASON_PS_PLAYER_NOT_ACCEPT_OTHERS),
                "REASON_HOST_IS_BLOCKED" => ::std::option::Option::Some(Reason::REASON_HOST_IS_BLOCKED),
                "REASON_OTHER_DATA_VERSION_NOT_LATEST" => ::std::option::Option::Some(Reason::REASON_OTHER_DATA_VERSION_NOT_LATEST),
                "REASON_DATA_VERSION_NOT_LATEST" => ::std::option::Option::Some(Reason::REASON_DATA_VERSION_NOT_LATEST),
                "REASON_PLAYER_NOT_IN_PLAYER_WORLD" => ::std::option::Option::Some(Reason::REASON_PLAYER_NOT_IN_PLAYER_WORLD),
                "REASON_MAX_PLAYER" => ::std::option::Option::Some(Reason::REASON_MAX_PLAYER),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [Reason] = &[
            Reason::REASON_PLAYER_JUDGE,
            Reason::REASON_SCENE_CANNOT_ENTER,
            Reason::REASON_PLAYER_CANNOT_ENTER_MP,
            Reason::REASON_SYSTEM_JUDGE,
            Reason::REASON_ALLOW_ENTER_PLAYER_FULL,
            Reason::REASON_WORLD_LEVEL_LOWER_THAN_HOST,
            Reason::REASON_HOST_IN_MATCH,
            Reason::REASON_PLAYER_IN_BLACKLIST,
            Reason::REASON_PS_PLAYER_NOT_ACCEPT_OTHERS,
            Reason::REASON_HOST_IS_BLOCKED,
            Reason::REASON_OTHER_DATA_VERSION_NOT_LATEST,
            Reason::REASON_DATA_VERSION_NOT_LATEST,
            Reason::REASON_PLAYER_NOT_IN_PLAYER_WORLD,
            Reason::REASON_MAX_PLAYER,
        ];
    }

    impl ::protobuf::EnumFull for Reason {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("PlayerApplyEnterMpResultNotify.Reason").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for Reason {
        fn default() -> Self {
            Reason::REASON_PLAYER_JUDGE
        }
    }

    impl Reason {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Reason>("PlayerApplyEnterMpResultNotify.Reason")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$PlayerApplyEnterMpResultNotify.proto\"\x9a\x05\n\x1ePlayerApplyEnterM\
    pResultNotify\x12>\n\x06reason\x18\x06\x20\x01(\x0e2&.PlayerApplyEnterMp\
    ResultNotify.ReasonR\x06reason\x12'\n\x0ftarget_nickname\x18\x08\x20\x01\
    (\tR\x0etargetNickname\x12\x1d\n\ntarget_uid\x18\x01\x20\x01(\rR\ttarget\
    Uid\x12\x1b\n\tis_agreed\x18\x04\x20\x01(\x08R\x08isAgreed\"\xd2\x03\n\
    \x06Reason\x12\x17\n\x13REASON_PLAYER_JUDGE\x10\0\x12\x1d\n\x19REASON_SC\
    ENE_CANNOT_ENTER\x10\x01\x12!\n\x1dREASON_PLAYER_CANNOT_ENTER_MP\x10\x02\
    \x12\x17\n\x13REASON_SYSTEM_JUDGE\x10\x03\x12\"\n\x1eREASON_ALLOW_ENTER_\
    PLAYER_FULL\x10\x04\x12&\n\"REASON_WORLD_LEVEL_LOWER_THAN_HOST\x10\x05\
    \x12\x18\n\x14REASON_HOST_IN_MATCH\x10\x06\x12\x1e\n\x1aREASON_PLAYER_IN\
    _BLACKLIST\x10\x07\x12&\n\"REASON_PS_PLAYER_NOT_ACCEPT_OTHERS\x10\x08\
    \x12\x1a\n\x16REASON_HOST_IS_BLOCKED\x10\t\x12(\n$REASON_OTHER_DATA_VERS\
    ION_NOT_LATEST\x10\n\x12\"\n\x1eREASON_DATA_VERSION_NOT_LATEST\x10\x0b\
    \x12%\n!REASON_PLAYER_NOT_IN_PLAYER_WORLD\x10\x0c\x12\x15\n\x11REASON_MA\
    X_PLAYER\x10\rB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(PlayerApplyEnterMpResultNotify::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(player_apply_enter_mp_result_notify::Reason::generated_enum_descriptor_data());
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

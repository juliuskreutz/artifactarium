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

//! Generated file from `LockedPersonallineData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_6_0;

// @@protoc_insertion_point(message:LockedPersonallineData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LockedPersonallineData {
    // message fields
    // @@protoc_insertion_point(field:LockedPersonallineData.lock_reason)
    pub lock_reason: ::protobuf::EnumOrUnknown<locked_personalline_data::LockReason>,
    // @@protoc_insertion_point(field:LockedPersonallineData.personal_line_id)
    pub personal_line_id: u32,
    // message oneof groups
    pub param: ::std::option::Option<locked_personalline_data::Param>,
    // special fields
    // @@protoc_insertion_point(special_field:LockedPersonallineData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LockedPersonallineData {
    fn default() -> &'a LockedPersonallineData {
        <LockedPersonallineData as ::protobuf::Message>::default_instance()
    }
}

impl LockedPersonallineData {
    pub fn new() -> LockedPersonallineData {
        ::std::default::Default::default()
    }

    // uint32 chapter_id = 11;

    pub fn chapter_id(&self) -> u32 {
        match self.param {
            ::std::option::Option::Some(locked_personalline_data::Param::ChapterId(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_chapter_id(&mut self) {
        self.param = ::std::option::Option::None;
    }

    pub fn has_chapter_id(&self) -> bool {
        match self.param {
            ::std::option::Option::Some(locked_personalline_data::Param::ChapterId(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_chapter_id(&mut self, v: u32) {
        self.param = ::std::option::Option::Some(locked_personalline_data::Param::ChapterId(v))
    }

    // uint32 level = 2;

    pub fn level(&self) -> u32 {
        match self.param {
            ::std::option::Option::Some(locked_personalline_data::Param::Level(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_level(&mut self) {
        self.param = ::std::option::Option::None;
    }

    pub fn has_level(&self) -> bool {
        match self.param {
            ::std::option::Option::Some(locked_personalline_data::Param::Level(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_level(&mut self, v: u32) {
        self.param = ::std::option::Option::Some(locked_personalline_data::Param::Level(v))
    }

    // .LockedPersonallineData.QuestParam quest_param = 1;

    pub fn quest_param(&self) -> &locked_personalline_data::QuestParam {
        match self.param {
            ::std::option::Option::Some(locked_personalline_data::Param::QuestParam(ref v)) => v,
            _ => <locked_personalline_data::QuestParam as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_quest_param(&mut self) {
        self.param = ::std::option::Option::None;
    }

    pub fn has_quest_param(&self) -> bool {
        match self.param {
            ::std::option::Option::Some(locked_personalline_data::Param::QuestParam(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_quest_param(&mut self, v: locked_personalline_data::QuestParam) {
        self.param = ::std::option::Option::Some(locked_personalline_data::Param::QuestParam(v))
    }

    // Mutable pointer to the field.
    pub fn mut_quest_param(&mut self) -> &mut locked_personalline_data::QuestParam {
        if let ::std::option::Option::Some(locked_personalline_data::Param::QuestParam(_)) = self.param {
        } else {
            self.param = ::std::option::Option::Some(locked_personalline_data::Param::QuestParam(locked_personalline_data::QuestParam::new()));
        }
        match self.param {
            ::std::option::Option::Some(locked_personalline_data::Param::QuestParam(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_quest_param(&mut self) -> locked_personalline_data::QuestParam {
        if self.has_quest_param() {
            match self.param.take() {
                ::std::option::Option::Some(locked_personalline_data::Param::QuestParam(v)) => v,
                _ => panic!(),
            }
        } else {
            locked_personalline_data::QuestParam::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "lock_reason",
            |m: &LockedPersonallineData| { &m.lock_reason },
            |m: &mut LockedPersonallineData| { &mut m.lock_reason },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "personal_line_id",
            |m: &LockedPersonallineData| { &m.personal_line_id },
            |m: &mut LockedPersonallineData| { &mut m.personal_line_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "chapter_id",
            LockedPersonallineData::has_chapter_id,
            LockedPersonallineData::chapter_id,
            LockedPersonallineData::set_chapter_id,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "level",
            LockedPersonallineData::has_level,
            LockedPersonallineData::level,
            LockedPersonallineData::set_level,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, locked_personalline_data::QuestParam>(
            "quest_param",
            LockedPersonallineData::has_quest_param,
            LockedPersonallineData::quest_param,
            LockedPersonallineData::mut_quest_param,
            LockedPersonallineData::set_quest_param,
        ));
        oneofs.push(locked_personalline_data::Param::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LockedPersonallineData>(
            "LockedPersonallineData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LockedPersonallineData {
    const NAME: &'static str = "LockedPersonallineData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.lock_reason = is.read_enum_or_unknown()?;
                },
                40 => {
                    self.personal_line_id = is.read_uint32()?;
                },
                88 => {
                    self.param = ::std::option::Option::Some(locked_personalline_data::Param::ChapterId(is.read_uint32()?));
                },
                16 => {
                    self.param = ::std::option::Option::Some(locked_personalline_data::Param::Level(is.read_uint32()?));
                },
                10 => {
                    self.param = ::std::option::Option::Some(locked_personalline_data::Param::QuestParam(is.read_message()?));
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
        if self.lock_reason != ::protobuf::EnumOrUnknown::new(locked_personalline_data::LockReason::LockReason_LEVEL) {
            my_size += ::protobuf::rt::int32_size(3, self.lock_reason.value());
        }
        if self.personal_line_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.personal_line_id);
        }
        if let ::std::option::Option::Some(ref v) = self.param {
            match v {
                &locked_personalline_data::Param::ChapterId(v) => {
                    my_size += ::protobuf::rt::uint32_size(11, v);
                },
                &locked_personalline_data::Param::Level(v) => {
                    my_size += ::protobuf::rt::uint32_size(2, v);
                },
                &locked_personalline_data::Param::QuestParam(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.lock_reason != ::protobuf::EnumOrUnknown::new(locked_personalline_data::LockReason::LockReason_LEVEL) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.lock_reason))?;
        }
        if self.personal_line_id != 0 {
            os.write_uint32(5, self.personal_line_id)?;
        }
        if let ::std::option::Option::Some(ref v) = self.param {
            match v {
                &locked_personalline_data::Param::ChapterId(v) => {
                    os.write_uint32(11, v)?;
                },
                &locked_personalline_data::Param::Level(v) => {
                    os.write_uint32(2, v)?;
                },
                &locked_personalline_data::Param::QuestParam(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                },
            };
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

    fn new() -> LockedPersonallineData {
        LockedPersonallineData::new()
    }

    fn clear(&mut self) {
        self.lock_reason = ::protobuf::EnumOrUnknown::new(locked_personalline_data::LockReason::LockReason_LEVEL);
        self.personal_line_id = 0;
        self.param = ::std::option::Option::None;
        self.param = ::std::option::Option::None;
        self.param = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LockedPersonallineData {
        static instance: LockedPersonallineData = LockedPersonallineData {
            lock_reason: ::protobuf::EnumOrUnknown::from_i32(0),
            personal_line_id: 0,
            param: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LockedPersonallineData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LockedPersonallineData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LockedPersonallineData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LockedPersonallineData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `LockedPersonallineData`
pub mod locked_personalline_data {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:LockedPersonallineData.param)
    pub enum Param {
        // @@protoc_insertion_point(oneof_field:LockedPersonallineData.chapter_id)
        ChapterId(u32),
        // @@protoc_insertion_point(oneof_field:LockedPersonallineData.level)
        Level(u32),
        // @@protoc_insertion_point(oneof_field:LockedPersonallineData.quest_param)
        QuestParam(QuestParam),
    }

    impl ::protobuf::Oneof for Param {
    }

    impl ::protobuf::OneofFull for Param {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::LockedPersonallineData as ::protobuf::MessageFull>::descriptor().oneof_by_name("param").unwrap()).clone()
        }
    }

    impl Param {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Param>("param")
        }
    }
    // @@protoc_insertion_point(message:LockedPersonallineData.QuestParam)
    #[derive(PartialEq,Clone,Default,Debug)]
    pub struct QuestParam {
        // message fields
        // @@protoc_insertion_point(field:LockedPersonallineData.QuestParam.chapter_id)
        pub chapter_id: u32,
        // @@protoc_insertion_point(field:LockedPersonallineData.QuestParam.quest_id)
        pub quest_id: u32,
        // special fields
        // @@protoc_insertion_point(special_field:LockedPersonallineData.QuestParam.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a QuestParam {
        fn default() -> &'a QuestParam {
            <QuestParam as ::protobuf::Message>::default_instance()
        }
    }

    impl QuestParam {
        pub fn new() -> QuestParam {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(2);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "chapter_id",
                |m: &QuestParam| { &m.chapter_id },
                |m: &mut QuestParam| { &mut m.chapter_id },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "quest_id",
                |m: &QuestParam| { &m.quest_id },
                |m: &mut QuestParam| { &mut m.quest_id },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<QuestParam>(
                "LockedPersonallineData.QuestParam",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for QuestParam {
        const NAME: &'static str = "QuestParam";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    8 => {
                        self.chapter_id = is.read_uint32()?;
                    },
                    40 => {
                        self.quest_id = is.read_uint32()?;
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
            if self.chapter_id != 0 {
                my_size += ::protobuf::rt::uint32_size(1, self.chapter_id);
            }
            if self.quest_id != 0 {
                my_size += ::protobuf::rt::uint32_size(5, self.quest_id);
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if self.chapter_id != 0 {
                os.write_uint32(1, self.chapter_id)?;
            }
            if self.quest_id != 0 {
                os.write_uint32(5, self.quest_id)?;
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

        fn new() -> QuestParam {
            QuestParam::new()
        }

        fn clear(&mut self) {
            self.chapter_id = 0;
            self.quest_id = 0;
            self.special_fields.clear();
        }

        fn default_instance() -> &'static QuestParam {
            static instance: QuestParam = QuestParam {
                chapter_id: 0,
                quest_id: 0,
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for QuestParam {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("LockedPersonallineData.QuestParam").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for QuestParam {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for QuestParam {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }

    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:LockedPersonallineData.LockReason)
    pub enum LockReason {
        // @@protoc_insertion_point(enum_value:LockedPersonallineData.LockReason.LockReason_LEVEL)
        LockReason_LEVEL = 0,
        // @@protoc_insertion_point(enum_value:LockedPersonallineData.LockReason.LockReason_QUEST)
        LockReason_QUEST = 1,
    }

    impl ::protobuf::Enum for LockReason {
        const NAME: &'static str = "LockReason";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<LockReason> {
            match value {
                0 => ::std::option::Option::Some(LockReason::LockReason_LEVEL),
                1 => ::std::option::Option::Some(LockReason::LockReason_QUEST),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<LockReason> {
            match str {
                "LockReason_LEVEL" => ::std::option::Option::Some(LockReason::LockReason_LEVEL),
                "LockReason_QUEST" => ::std::option::Option::Some(LockReason::LockReason_QUEST),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [LockReason] = &[
            LockReason::LockReason_LEVEL,
            LockReason::LockReason_QUEST,
        ];
    }

    impl ::protobuf::EnumFull for LockReason {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("LockedPersonallineData.LockReason").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for LockReason {
        fn default() -> Self {
            LockReason::LockReason_LEVEL
        }
    }

    impl LockReason {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<LockReason>("LockedPersonallineData.LockReason")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cLockedPersonallineData.proto\"\x92\x03\n\x16LockedPersonallineData\
    \x12C\n\x0block_reason\x18\x03\x20\x01(\x0e2\".LockedPersonallineData.Lo\
    ckReasonR\nlockReason\x12(\n\x10personal_line_id\x18\x05\x20\x01(\rR\x0e\
    personalLineId\x12\x1f\n\nchapter_id\x18\x0b\x20\x01(\rH\0R\tchapterId\
    \x12\x16\n\x05level\x18\x02\x20\x01(\rH\0R\x05level\x12E\n\x0bquest_para\
    m\x18\x01\x20\x01(\x0b2\".LockedPersonallineData.QuestParamH\0R\nquestPa\
    ram\x1aF\n\nQuestParam\x12\x1d\n\nchapter_id\x18\x01\x20\x01(\rR\tchapte\
    rId\x12\x19\n\x08quest_id\x18\x05\x20\x01(\rR\x07questId\"8\n\nLockReaso\
    n\x12\x14\n\x10LockReason_LEVEL\x10\0\x12\x14\n\x10LockReason_QUEST\x10\
    \x01B\x07\n\x05paramB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(LockedPersonallineData::generated_message_descriptor_data());
            messages.push(locked_personalline_data::QuestParam::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(locked_personalline_data::LockReason::generated_enum_descriptor_data());
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

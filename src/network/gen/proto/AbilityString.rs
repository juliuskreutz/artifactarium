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

//! Generated file from `AbilityString.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_6_0;

// @@protoc_insertion_point(message:AbilityString)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AbilityString {
    // message oneof groups
    pub type_: ::std::option::Option<ability_string::Type>,
    // special fields
    // @@protoc_insertion_point(special_field:AbilityString.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AbilityString {
    fn default() -> &'a AbilityString {
        <AbilityString as ::protobuf::Message>::default_instance()
    }
}

impl AbilityString {
    pub fn new() -> AbilityString {
        ::std::default::Default::default()
    }

    // string str = 1;

    pub fn str(&self) -> &str {
        match self.type_ {
            ::std::option::Option::Some(ability_string::Type::Str(ref v)) => v,
            _ => "",
        }
    }

    pub fn clear_str(&mut self) {
        self.type_ = ::std::option::Option::None;
    }

    pub fn has_str(&self) -> bool {
        match self.type_ {
            ::std::option::Option::Some(ability_string::Type::Str(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_str(&mut self, v: ::std::string::String) {
        self.type_ = ::std::option::Option::Some(ability_string::Type::Str(v))
    }

    // Mutable pointer to the field.
    pub fn mut_str(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(ability_string::Type::Str(_)) = self.type_ {
        } else {
            self.type_ = ::std::option::Option::Some(ability_string::Type::Str(::std::string::String::new()));
        }
        match self.type_ {
            ::std::option::Option::Some(ability_string::Type::Str(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_str(&mut self) -> ::std::string::String {
        if self.has_str() {
            match self.type_.take() {
                ::std::option::Option::Some(ability_string::Type::Str(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // uint32 hash = 2;

    pub fn hash(&self) -> u32 {
        match self.type_ {
            ::std::option::Option::Some(ability_string::Type::Hash(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_hash(&mut self) {
        self.type_ = ::std::option::Option::None;
    }

    pub fn has_hash(&self) -> bool {
        match self.type_ {
            ::std::option::Option::Some(ability_string::Type::Hash(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: u32) {
        self.type_ = ::std::option::Option::Some(ability_string::Type::Hash(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_deref_has_get_set_simpler_accessor::<_, _>(
            "str",
            AbilityString::has_str,
            AbilityString::str,
            AbilityString::set_str,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "hash",
            AbilityString::has_hash,
            AbilityString::hash,
            AbilityString::set_hash,
        ));
        oneofs.push(ability_string::Type::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AbilityString>(
            "AbilityString",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AbilityString {
    const NAME: &'static str = "AbilityString";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.type_ = ::std::option::Option::Some(ability_string::Type::Str(is.read_string()?));
                },
                16 => {
                    self.type_ = ::std::option::Option::Some(ability_string::Type::Hash(is.read_uint32()?));
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
        if let ::std::option::Option::Some(ref v) = self.type_ {
            match v {
                &ability_string::Type::Str(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &ability_string::Type::Hash(v) => {
                    my_size += ::protobuf::rt::uint32_size(2, v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.type_ {
            match v {
                &ability_string::Type::Str(ref v) => {
                    os.write_string(1, v)?;
                },
                &ability_string::Type::Hash(v) => {
                    os.write_uint32(2, v)?;
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

    fn new() -> AbilityString {
        AbilityString::new()
    }

    fn clear(&mut self) {
        self.type_ = ::std::option::Option::None;
        self.type_ = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AbilityString {
        static instance: AbilityString = AbilityString {
            type_: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AbilityString {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AbilityString").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AbilityString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AbilityString {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `AbilityString`
pub mod ability_string {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:AbilityString.type)
    pub enum Type {
        // @@protoc_insertion_point(oneof_field:AbilityString.str)
        Str(::std::string::String),
        // @@protoc_insertion_point(oneof_field:AbilityString.hash)
        Hash(u32),
    }

    impl ::protobuf::Oneof for Type {
    }

    impl ::protobuf::OneofFull for Type {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::AbilityString as ::protobuf::MessageFull>::descriptor().oneof_by_name("type").unwrap()).clone()
        }
    }

    impl Type {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Type>("type")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13AbilityString.proto\"A\n\rAbilityString\x12\x12\n\x03str\x18\x01\
    \x20\x01(\tH\0R\x03str\x12\x14\n\x04hash\x18\x02\x20\x01(\rH\0R\x04hashB\
    \x06\n\x04typeB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(AbilityString::generated_message_descriptor_data());
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

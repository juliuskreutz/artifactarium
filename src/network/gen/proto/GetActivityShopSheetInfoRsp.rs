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

//! Generated file from `GetActivityShopSheetInfoRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_6_0;

// @@protoc_insertion_point(message:GetActivityShopSheetInfoRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetActivityShopSheetInfoRsp {
    // message fields
    // @@protoc_insertion_point(field:GetActivityShopSheetInfoRsp.sheet_info_list)
    pub sheet_info_list: ::std::vec::Vec<super::ActivityShopSheetInfo::ActivityShopSheetInfo>,
    // @@protoc_insertion_point(field:GetActivityShopSheetInfoRsp.shop_type)
    pub shop_type: u32,
    // @@protoc_insertion_point(field:GetActivityShopSheetInfoRsp.retcode)
    pub retcode: i32,
    // special fields
    // @@protoc_insertion_point(special_field:GetActivityShopSheetInfoRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetActivityShopSheetInfoRsp {
    fn default() -> &'a GetActivityShopSheetInfoRsp {
        <GetActivityShopSheetInfoRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetActivityShopSheetInfoRsp {
    pub fn new() -> GetActivityShopSheetInfoRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "sheet_info_list",
            |m: &GetActivityShopSheetInfoRsp| { &m.sheet_info_list },
            |m: &mut GetActivityShopSheetInfoRsp| { &mut m.sheet_info_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "shop_type",
            |m: &GetActivityShopSheetInfoRsp| { &m.shop_type },
            |m: &mut GetActivityShopSheetInfoRsp| { &mut m.shop_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetActivityShopSheetInfoRsp| { &m.retcode },
            |m: &mut GetActivityShopSheetInfoRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetActivityShopSheetInfoRsp>(
            "GetActivityShopSheetInfoRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetActivityShopSheetInfoRsp {
    const NAME: &'static str = "GetActivityShopSheetInfoRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    self.sheet_info_list.push(is.read_message()?);
                },
                8 => {
                    self.shop_type = is.read_uint32()?;
                },
                48 => {
                    self.retcode = is.read_int32()?;
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
        for value in &self.sheet_info_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.shop_type != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.shop_type);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.sheet_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.shop_type != 0 {
            os.write_uint32(1, self.shop_type)?;
        }
        if self.retcode != 0 {
            os.write_int32(6, self.retcode)?;
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

    fn new() -> GetActivityShopSheetInfoRsp {
        GetActivityShopSheetInfoRsp::new()
    }

    fn clear(&mut self) {
        self.sheet_info_list.clear();
        self.shop_type = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetActivityShopSheetInfoRsp {
        static instance: GetActivityShopSheetInfoRsp = GetActivityShopSheetInfoRsp {
            sheet_info_list: ::std::vec::Vec::new(),
            shop_type: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetActivityShopSheetInfoRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetActivityShopSheetInfoRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetActivityShopSheetInfoRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetActivityShopSheetInfoRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!GetActivityShopSheetInfoRsp.proto\x1a\x1bActivityShopSheetInfo.proto\
    \"\x94\x01\n\x1bGetActivityShopSheetInfoRsp\x12>\n\x0fsheet_info_list\
    \x18\x05\x20\x03(\x0b2\x16.ActivityShopSheetInfoR\rsheetInfoList\x12\x1b\
    \n\tshop_type\x18\x01\x20\x01(\rR\x08shopType\x12\x18\n\x07retcode\x18\
    \x06\x20\x01(\x05R\x07retcodeB\x1b\n\x19emu.grasscutter.net.protob\x06pr\
    oto3\
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
            deps.push(super::ActivityShopSheetInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetActivityShopSheetInfoRsp::generated_message_descriptor_data());
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

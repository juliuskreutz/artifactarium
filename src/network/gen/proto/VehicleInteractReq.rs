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

//! Generated file from `VehicleInteractReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_6_0;

// @@protoc_insertion_point(message:VehicleInteractReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct VehicleInteractReq {
    // message fields
    // @@protoc_insertion_point(field:VehicleInteractReq.pos)
    pub pos: u32,
    // @@protoc_insertion_point(field:VehicleInteractReq.entity_id)
    pub entity_id: u32,
    // @@protoc_insertion_point(field:VehicleInteractReq.EILHBJJEPOK)
    pub EILHBJJEPOK: bool,
    // @@protoc_insertion_point(field:VehicleInteractReq.interact_type)
    pub interact_type: ::protobuf::EnumOrUnknown<super::VehicleInteractType::VehicleInteractType>,
    // special fields
    // @@protoc_insertion_point(special_field:VehicleInteractReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a VehicleInteractReq {
    fn default() -> &'a VehicleInteractReq {
        <VehicleInteractReq as ::protobuf::Message>::default_instance()
    }
}

impl VehicleInteractReq {
    pub fn new() -> VehicleInteractReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "pos",
            |m: &VehicleInteractReq| { &m.pos },
            |m: &mut VehicleInteractReq| { &mut m.pos },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "entity_id",
            |m: &VehicleInteractReq| { &m.entity_id },
            |m: &mut VehicleInteractReq| { &mut m.entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EILHBJJEPOK",
            |m: &VehicleInteractReq| { &m.EILHBJJEPOK },
            |m: &mut VehicleInteractReq| { &mut m.EILHBJJEPOK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "interact_type",
            |m: &VehicleInteractReq| { &m.interact_type },
            |m: &mut VehicleInteractReq| { &mut m.interact_type },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<VehicleInteractReq>(
            "VehicleInteractReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for VehicleInteractReq {
    const NAME: &'static str = "VehicleInteractReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.pos = is.read_uint32()?;
                },
                40 => {
                    self.entity_id = is.read_uint32()?;
                },
                8 => {
                    self.EILHBJJEPOK = is.read_bool()?;
                },
                64 => {
                    self.interact_type = is.read_enum_or_unknown()?;
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
        if self.pos != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.pos);
        }
        if self.entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.entity_id);
        }
        if self.EILHBJJEPOK != false {
            my_size += 1 + 1;
        }
        if self.interact_type != ::protobuf::EnumOrUnknown::new(super::VehicleInteractType::VehicleInteractType::VEHICLE_INTERACT_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(8, self.interact_type.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.pos != 0 {
            os.write_uint32(13, self.pos)?;
        }
        if self.entity_id != 0 {
            os.write_uint32(5, self.entity_id)?;
        }
        if self.EILHBJJEPOK != false {
            os.write_bool(1, self.EILHBJJEPOK)?;
        }
        if self.interact_type != ::protobuf::EnumOrUnknown::new(super::VehicleInteractType::VehicleInteractType::VEHICLE_INTERACT_TYPE_NONE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.interact_type))?;
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

    fn new() -> VehicleInteractReq {
        VehicleInteractReq::new()
    }

    fn clear(&mut self) {
        self.pos = 0;
        self.entity_id = 0;
        self.EILHBJJEPOK = false;
        self.interact_type = ::protobuf::EnumOrUnknown::new(super::VehicleInteractType::VehicleInteractType::VEHICLE_INTERACT_TYPE_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static VehicleInteractReq {
        static instance: VehicleInteractReq = VehicleInteractReq {
            pos: 0,
            entity_id: 0,
            EILHBJJEPOK: false,
            interact_type: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for VehicleInteractReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("VehicleInteractReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for VehicleInteractReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VehicleInteractReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18VehicleInteractReq.proto\x1a\x19VehicleInteractType.proto\"\xa0\
    \x01\n\x12VehicleInteractReq\x12\x10\n\x03pos\x18\r\x20\x01(\rR\x03pos\
    \x12\x1b\n\tentity_id\x18\x05\x20\x01(\rR\x08entityId\x12\x20\n\x0bEILHB\
    JJEPOK\x18\x01\x20\x01(\x08R\x0bEILHBJJEPOK\x129\n\rinteract_type\x18\
    \x08\x20\x01(\x0e2\x14.VehicleInteractTypeR\x0cinteractTypeB\x1b\n\x19em\
    u.grasscutter.net.protob\x06proto3\
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
            deps.push(super::VehicleInteractType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(VehicleInteractReq::generated_message_descriptor_data());
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

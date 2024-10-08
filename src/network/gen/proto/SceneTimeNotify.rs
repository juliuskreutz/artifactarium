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

//! Generated file from `SceneTimeNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_6_0;

// @@protoc_insertion_point(message:SceneTimeNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneTimeNotify {
    // message fields
    // @@protoc_insertion_point(field:SceneTimeNotify.scene_id)
    pub scene_id: u32,
    // @@protoc_insertion_point(field:SceneTimeNotify.scene_time)
    pub scene_time: u64,
    // @@protoc_insertion_point(field:SceneTimeNotify.is_paused)
    pub is_paused: bool,
    // special fields
    // @@protoc_insertion_point(special_field:SceneTimeNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneTimeNotify {
    fn default() -> &'a SceneTimeNotify {
        <SceneTimeNotify as ::protobuf::Message>::default_instance()
    }
}

impl SceneTimeNotify {
    pub fn new() -> SceneTimeNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "scene_id",
            |m: &SceneTimeNotify| { &m.scene_id },
            |m: &mut SceneTimeNotify| { &mut m.scene_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "scene_time",
            |m: &SceneTimeNotify| { &m.scene_time },
            |m: &mut SceneTimeNotify| { &mut m.scene_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_paused",
            |m: &SceneTimeNotify| { &m.is_paused },
            |m: &mut SceneTimeNotify| { &mut m.is_paused },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneTimeNotify>(
            "SceneTimeNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneTimeNotify {
    const NAME: &'static str = "SceneTimeNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.scene_id = is.read_uint32()?;
                },
                88 => {
                    self.scene_time = is.read_uint64()?;
                },
                104 => {
                    self.is_paused = is.read_bool()?;
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
        if self.scene_id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.scene_id);
        }
        if self.scene_time != 0 {
            my_size += ::protobuf::rt::uint64_size(11, self.scene_time);
        }
        if self.is_paused != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.scene_id != 0 {
            os.write_uint32(7, self.scene_id)?;
        }
        if self.scene_time != 0 {
            os.write_uint64(11, self.scene_time)?;
        }
        if self.is_paused != false {
            os.write_bool(13, self.is_paused)?;
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

    fn new() -> SceneTimeNotify {
        SceneTimeNotify::new()
    }

    fn clear(&mut self) {
        self.scene_id = 0;
        self.scene_time = 0;
        self.is_paused = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneTimeNotify {
        static instance: SceneTimeNotify = SceneTimeNotify {
            scene_id: 0,
            scene_time: 0,
            is_paused: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SceneTimeNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneTimeNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneTimeNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneTimeNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15SceneTimeNotify.proto\"h\n\x0fSceneTimeNotify\x12\x19\n\x08scene_i\
    d\x18\x07\x20\x01(\rR\x07sceneId\x12\x1d\n\nscene_time\x18\x0b\x20\x01(\
    \x04R\tsceneTime\x12\x1b\n\tis_paused\x18\r\x20\x01(\x08R\x08isPausedB\
    \x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(SceneTimeNotify::generated_message_descriptor_data());
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

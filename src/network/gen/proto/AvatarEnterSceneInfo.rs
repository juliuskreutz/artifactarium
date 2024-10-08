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

//! Generated file from `AvatarEnterSceneInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_6_0;

// @@protoc_insertion_point(message:AvatarEnterSceneInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AvatarEnterSceneInfo {
    // message fields
    // @@protoc_insertion_point(field:AvatarEnterSceneInfo.weapon_entity_id)
    pub weapon_entity_id: u32,
    // @@protoc_insertion_point(field:AvatarEnterSceneInfo.weapon_ability_info)
    pub weapon_ability_info: ::protobuf::MessageField<super::AbilitySyncStateInfo::AbilitySyncStateInfo>,
    // @@protoc_insertion_point(field:AvatarEnterSceneInfo.avatar_entity_id)
    pub avatar_entity_id: u32,
    // @@protoc_insertion_point(field:AvatarEnterSceneInfo.weapon_guid)
    pub weapon_guid: u64,
    // @@protoc_insertion_point(field:AvatarEnterSceneInfo.server_buff_list)
    pub server_buff_list: ::std::vec::Vec<super::ServerBuff::ServerBuff>,
    // @@protoc_insertion_point(field:AvatarEnterSceneInfo.avatar_ability_info)
    pub avatar_ability_info: ::protobuf::MessageField<super::AbilitySyncStateInfo::AbilitySyncStateInfo>,
    // @@protoc_insertion_point(field:AvatarEnterSceneInfo.buff_id_list)
    pub buff_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:AvatarEnterSceneInfo.avatar_guid)
    pub avatar_guid: u64,
    // special fields
    // @@protoc_insertion_point(special_field:AvatarEnterSceneInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AvatarEnterSceneInfo {
    fn default() -> &'a AvatarEnterSceneInfo {
        <AvatarEnterSceneInfo as ::protobuf::Message>::default_instance()
    }
}

impl AvatarEnterSceneInfo {
    pub fn new() -> AvatarEnterSceneInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "weapon_entity_id",
            |m: &AvatarEnterSceneInfo| { &m.weapon_entity_id },
            |m: &mut AvatarEnterSceneInfo| { &mut m.weapon_entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AbilitySyncStateInfo::AbilitySyncStateInfo>(
            "weapon_ability_info",
            |m: &AvatarEnterSceneInfo| { &m.weapon_ability_info },
            |m: &mut AvatarEnterSceneInfo| { &mut m.weapon_ability_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_entity_id",
            |m: &AvatarEnterSceneInfo| { &m.avatar_entity_id },
            |m: &mut AvatarEnterSceneInfo| { &mut m.avatar_entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "weapon_guid",
            |m: &AvatarEnterSceneInfo| { &m.weapon_guid },
            |m: &mut AvatarEnterSceneInfo| { &mut m.weapon_guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "server_buff_list",
            |m: &AvatarEnterSceneInfo| { &m.server_buff_list },
            |m: &mut AvatarEnterSceneInfo| { &mut m.server_buff_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AbilitySyncStateInfo::AbilitySyncStateInfo>(
            "avatar_ability_info",
            |m: &AvatarEnterSceneInfo| { &m.avatar_ability_info },
            |m: &mut AvatarEnterSceneInfo| { &mut m.avatar_ability_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "buff_id_list",
            |m: &AvatarEnterSceneInfo| { &m.buff_id_list },
            |m: &mut AvatarEnterSceneInfo| { &mut m.buff_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_guid",
            |m: &AvatarEnterSceneInfo| { &m.avatar_guid },
            |m: &mut AvatarEnterSceneInfo| { &mut m.avatar_guid },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AvatarEnterSceneInfo>(
            "AvatarEnterSceneInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AvatarEnterSceneInfo {
    const NAME: &'static str = "AvatarEnterSceneInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.weapon_entity_id = is.read_uint32()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.weapon_ability_info)?;
                },
                48 => {
                    self.avatar_entity_id = is.read_uint32()?;
                },
                56 => {
                    self.weapon_guid = is.read_uint64()?;
                },
                66 => {
                    self.server_buff_list.push(is.read_message()?);
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.avatar_ability_info)?;
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.buff_id_list)?;
                },
                80 => {
                    self.buff_id_list.push(is.read_uint32()?);
                },
                96 => {
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
        if self.weapon_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.weapon_entity_id);
        }
        if let Some(v) = self.weapon_ability_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.avatar_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.avatar_entity_id);
        }
        if self.weapon_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(7, self.weapon_guid);
        }
        for value in &self.server_buff_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.avatar_ability_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(10, &self.buff_id_list);
        if self.avatar_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(12, self.avatar_guid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.weapon_entity_id != 0 {
            os.write_uint32(3, self.weapon_entity_id)?;
        }
        if let Some(v) = self.weapon_ability_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.avatar_entity_id != 0 {
            os.write_uint32(6, self.avatar_entity_id)?;
        }
        if self.weapon_guid != 0 {
            os.write_uint64(7, self.weapon_guid)?;
        }
        for v in &self.server_buff_list {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if let Some(v) = self.avatar_ability_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        os.write_repeated_packed_uint32(10, &self.buff_id_list)?;
        if self.avatar_guid != 0 {
            os.write_uint64(12, self.avatar_guid)?;
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

    fn new() -> AvatarEnterSceneInfo {
        AvatarEnterSceneInfo::new()
    }

    fn clear(&mut self) {
        self.weapon_entity_id = 0;
        self.weapon_ability_info.clear();
        self.avatar_entity_id = 0;
        self.weapon_guid = 0;
        self.server_buff_list.clear();
        self.avatar_ability_info.clear();
        self.buff_id_list.clear();
        self.avatar_guid = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AvatarEnterSceneInfo {
        static instance: AvatarEnterSceneInfo = AvatarEnterSceneInfo {
            weapon_entity_id: 0,
            weapon_ability_info: ::protobuf::MessageField::none(),
            avatar_entity_id: 0,
            weapon_guid: 0,
            server_buff_list: ::std::vec::Vec::new(),
            avatar_ability_info: ::protobuf::MessageField::none(),
            buff_id_list: ::std::vec::Vec::new(),
            avatar_guid: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AvatarEnterSceneInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AvatarEnterSceneInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AvatarEnterSceneInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AvatarEnterSceneInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aAvatarEnterSceneInfo.proto\x1a\x1aAbilitySyncStateInfo.proto\x1a\
    \x10ServerBuff.proto\"\x93\x03\n\x14AvatarEnterSceneInfo\x12(\n\x10weapo\
    n_entity_id\x18\x03\x20\x01(\rR\x0eweaponEntityId\x12E\n\x13weapon_abili\
    ty_info\x18\x04\x20\x01(\x0b2\x15.AbilitySyncStateInfoR\x11weaponAbility\
    Info\x12(\n\x10avatar_entity_id\x18\x06\x20\x01(\rR\x0eavatarEntityId\
    \x12\x1f\n\x0bweapon_guid\x18\x07\x20\x01(\x04R\nweaponGuid\x125\n\x10se\
    rver_buff_list\x18\x08\x20\x03(\x0b2\x0b.ServerBuffR\x0eserverBuffList\
    \x12E\n\x13avatar_ability_info\x18\t\x20\x01(\x0b2\x15.AbilitySyncStateI\
    nfoR\x11avatarAbilityInfo\x12\x20\n\x0cbuff_id_list\x18\n\x20\x03(\rR\nb\
    uffIdList\x12\x1f\n\x0bavatar_guid\x18\x0c\x20\x01(\x04R\navatarGuidB\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::AbilitySyncStateInfo::file_descriptor().clone());
            deps.push(super::ServerBuff::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AvatarEnterSceneInfo::generated_message_descriptor_data());
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

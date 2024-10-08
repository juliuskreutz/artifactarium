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

//! Generated file from `RegionInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_6_0;

// @@protoc_insertion_point(message:RegionInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RegionInfo {
    // message fields
    // @@protoc_insertion_point(field:RegionInfo.gateserver_ip)
    pub gateserver_ip: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.gateserver_port)
    pub gateserver_port: u32,
    // @@protoc_insertion_point(field:RegionInfo.pay_callback_url)
    pub pay_callback_url: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.area_type)
    pub area_type: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.resource_url)
    pub resource_url: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.data_url)
    pub data_url: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.feedback_url)
    pub feedback_url: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.bulletin_url)
    pub bulletin_url: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.resource_url_bak)
    pub resource_url_bak: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.data_url_bak)
    pub data_url_bak: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.client_data_version)
    pub client_data_version: u32,
    // @@protoc_insertion_point(field:RegionInfo.handbook_url)
    pub handbook_url: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.client_silence_data_version)
    pub client_silence_data_version: u32,
    // @@protoc_insertion_point(field:RegionInfo.client_data_md5)
    pub client_data_md5: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.client_silence_data_md5)
    pub client_silence_data_md5: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.res_version_config)
    pub res_version_config: ::protobuf::MessageField<super::ResVersionConfig::ResVersionConfig>,
    // @@protoc_insertion_point(field:RegionInfo.secret_key)
    pub secret_key: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:RegionInfo.official_community_url)
    pub official_community_url: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.client_version_suffix)
    pub client_version_suffix: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.client_silence_version_suffix)
    pub client_silence_version_suffix: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.use_gateserver_domain_name)
    pub use_gateserver_domain_name: bool,
    // @@protoc_insertion_point(field:RegionInfo.gateserver_domain_name)
    pub gateserver_domain_name: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.user_center_url)
    pub user_center_url: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.account_bind_url)
    pub account_bind_url: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.cdkey_url)
    pub cdkey_url: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.privacy_policy_url)
    pub privacy_policy_url: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.next_resource_url)
    pub next_resource_url: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.next_res_version_config)
    pub next_res_version_config: ::protobuf::MessageField<super::ResVersionConfig::ResVersionConfig>,
    // @@protoc_insertion_point(field:RegionInfo.game_biz)
    pub game_biz: ::std::string::String,
    // @@protoc_insertion_point(field:RegionInfo.gateserver_ipv6_ip)
    pub gateserver_ipv6_ip: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:RegionInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RegionInfo {
    fn default() -> &'a RegionInfo {
        <RegionInfo as ::protobuf::Message>::default_instance()
    }
}

impl RegionInfo {
    pub fn new() -> RegionInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(30);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gateserver_ip",
            |m: &RegionInfo| { &m.gateserver_ip },
            |m: &mut RegionInfo| { &mut m.gateserver_ip },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gateserver_port",
            |m: &RegionInfo| { &m.gateserver_port },
            |m: &mut RegionInfo| { &mut m.gateserver_port },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "pay_callback_url",
            |m: &RegionInfo| { &m.pay_callback_url },
            |m: &mut RegionInfo| { &mut m.pay_callback_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "area_type",
            |m: &RegionInfo| { &m.area_type },
            |m: &mut RegionInfo| { &mut m.area_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "resource_url",
            |m: &RegionInfo| { &m.resource_url },
            |m: &mut RegionInfo| { &mut m.resource_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data_url",
            |m: &RegionInfo| { &m.data_url },
            |m: &mut RegionInfo| { &mut m.data_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "feedback_url",
            |m: &RegionInfo| { &m.feedback_url },
            |m: &mut RegionInfo| { &mut m.feedback_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "bulletin_url",
            |m: &RegionInfo| { &m.bulletin_url },
            |m: &mut RegionInfo| { &mut m.bulletin_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "resource_url_bak",
            |m: &RegionInfo| { &m.resource_url_bak },
            |m: &mut RegionInfo| { &mut m.resource_url_bak },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data_url_bak",
            |m: &RegionInfo| { &m.data_url_bak },
            |m: &mut RegionInfo| { &mut m.data_url_bak },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "client_data_version",
            |m: &RegionInfo| { &m.client_data_version },
            |m: &mut RegionInfo| { &mut m.client_data_version },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "handbook_url",
            |m: &RegionInfo| { &m.handbook_url },
            |m: &mut RegionInfo| { &mut m.handbook_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "client_silence_data_version",
            |m: &RegionInfo| { &m.client_silence_data_version },
            |m: &mut RegionInfo| { &mut m.client_silence_data_version },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "client_data_md5",
            |m: &RegionInfo| { &m.client_data_md5 },
            |m: &mut RegionInfo| { &mut m.client_data_md5 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "client_silence_data_md5",
            |m: &RegionInfo| { &m.client_silence_data_md5 },
            |m: &mut RegionInfo| { &mut m.client_silence_data_md5 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ResVersionConfig::ResVersionConfig>(
            "res_version_config",
            |m: &RegionInfo| { &m.res_version_config },
            |m: &mut RegionInfo| { &mut m.res_version_config },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "secret_key",
            |m: &RegionInfo| { &m.secret_key },
            |m: &mut RegionInfo| { &mut m.secret_key },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "official_community_url",
            |m: &RegionInfo| { &m.official_community_url },
            |m: &mut RegionInfo| { &mut m.official_community_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "client_version_suffix",
            |m: &RegionInfo| { &m.client_version_suffix },
            |m: &mut RegionInfo| { &mut m.client_version_suffix },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "client_silence_version_suffix",
            |m: &RegionInfo| { &m.client_silence_version_suffix },
            |m: &mut RegionInfo| { &mut m.client_silence_version_suffix },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "use_gateserver_domain_name",
            |m: &RegionInfo| { &m.use_gateserver_domain_name },
            |m: &mut RegionInfo| { &mut m.use_gateserver_domain_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gateserver_domain_name",
            |m: &RegionInfo| { &m.gateserver_domain_name },
            |m: &mut RegionInfo| { &mut m.gateserver_domain_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "user_center_url",
            |m: &RegionInfo| { &m.user_center_url },
            |m: &mut RegionInfo| { &mut m.user_center_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "account_bind_url",
            |m: &RegionInfo| { &m.account_bind_url },
            |m: &mut RegionInfo| { &mut m.account_bind_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cdkey_url",
            |m: &RegionInfo| { &m.cdkey_url },
            |m: &mut RegionInfo| { &mut m.cdkey_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "privacy_policy_url",
            |m: &RegionInfo| { &m.privacy_policy_url },
            |m: &mut RegionInfo| { &mut m.privacy_policy_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "next_resource_url",
            |m: &RegionInfo| { &m.next_resource_url },
            |m: &mut RegionInfo| { &mut m.next_resource_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ResVersionConfig::ResVersionConfig>(
            "next_res_version_config",
            |m: &RegionInfo| { &m.next_res_version_config },
            |m: &mut RegionInfo| { &mut m.next_res_version_config },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "game_biz",
            |m: &RegionInfo| { &m.game_biz },
            |m: &mut RegionInfo| { &mut m.game_biz },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gateserver_ipv6_ip",
            |m: &RegionInfo| { &m.gateserver_ipv6_ip },
            |m: &mut RegionInfo| { &mut m.gateserver_ipv6_ip },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RegionInfo>(
            "RegionInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RegionInfo {
    const NAME: &'static str = "RegionInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.gateserver_ip = is.read_string()?;
                },
                16 => {
                    self.gateserver_port = is.read_uint32()?;
                },
                26 => {
                    self.pay_callback_url = is.read_string()?;
                },
                58 => {
                    self.area_type = is.read_string()?;
                },
                66 => {
                    self.resource_url = is.read_string()?;
                },
                74 => {
                    self.data_url = is.read_string()?;
                },
                82 => {
                    self.feedback_url = is.read_string()?;
                },
                90 => {
                    self.bulletin_url = is.read_string()?;
                },
                98 => {
                    self.resource_url_bak = is.read_string()?;
                },
                106 => {
                    self.data_url_bak = is.read_string()?;
                },
                112 => {
                    self.client_data_version = is.read_uint32()?;
                },
                130 => {
                    self.handbook_url = is.read_string()?;
                },
                144 => {
                    self.client_silence_data_version = is.read_uint32()?;
                },
                154 => {
                    self.client_data_md5 = is.read_string()?;
                },
                162 => {
                    self.client_silence_data_md5 = is.read_string()?;
                },
                178 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.res_version_config)?;
                },
                186 => {
                    self.secret_key = is.read_bytes()?;
                },
                194 => {
                    self.official_community_url = is.read_string()?;
                },
                210 => {
                    self.client_version_suffix = is.read_string()?;
                },
                218 => {
                    self.client_silence_version_suffix = is.read_string()?;
                },
                224 => {
                    self.use_gateserver_domain_name = is.read_bool()?;
                },
                234 => {
                    self.gateserver_domain_name = is.read_string()?;
                },
                242 => {
                    self.user_center_url = is.read_string()?;
                },
                250 => {
                    self.account_bind_url = is.read_string()?;
                },
                258 => {
                    self.cdkey_url = is.read_string()?;
                },
                266 => {
                    self.privacy_policy_url = is.read_string()?;
                },
                274 => {
                    self.next_resource_url = is.read_string()?;
                },
                282 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.next_res_version_config)?;
                },
                290 => {
                    self.game_biz = is.read_string()?;
                },
                298 => {
                    self.gateserver_ipv6_ip = is.read_string()?;
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
        if !self.gateserver_ip.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.gateserver_ip);
        }
        if self.gateserver_port != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.gateserver_port);
        }
        if !self.pay_callback_url.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.pay_callback_url);
        }
        if !self.area_type.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.area_type);
        }
        if !self.resource_url.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.resource_url);
        }
        if !self.data_url.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.data_url);
        }
        if !self.feedback_url.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.feedback_url);
        }
        if !self.bulletin_url.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.bulletin_url);
        }
        if !self.resource_url_bak.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.resource_url_bak);
        }
        if !self.data_url_bak.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.data_url_bak);
        }
        if self.client_data_version != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.client_data_version);
        }
        if !self.handbook_url.is_empty() {
            my_size += ::protobuf::rt::string_size(16, &self.handbook_url);
        }
        if self.client_silence_data_version != 0 {
            my_size += ::protobuf::rt::uint32_size(18, self.client_silence_data_version);
        }
        if !self.client_data_md5.is_empty() {
            my_size += ::protobuf::rt::string_size(19, &self.client_data_md5);
        }
        if !self.client_silence_data_md5.is_empty() {
            my_size += ::protobuf::rt::string_size(20, &self.client_silence_data_md5);
        }
        if let Some(v) = self.res_version_config.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.secret_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(23, &self.secret_key);
        }
        if !self.official_community_url.is_empty() {
            my_size += ::protobuf::rt::string_size(24, &self.official_community_url);
        }
        if !self.client_version_suffix.is_empty() {
            my_size += ::protobuf::rt::string_size(26, &self.client_version_suffix);
        }
        if !self.client_silence_version_suffix.is_empty() {
            my_size += ::protobuf::rt::string_size(27, &self.client_silence_version_suffix);
        }
        if self.use_gateserver_domain_name != false {
            my_size += 2 + 1;
        }
        if !self.gateserver_domain_name.is_empty() {
            my_size += ::protobuf::rt::string_size(29, &self.gateserver_domain_name);
        }
        if !self.user_center_url.is_empty() {
            my_size += ::protobuf::rt::string_size(30, &self.user_center_url);
        }
        if !self.account_bind_url.is_empty() {
            my_size += ::protobuf::rt::string_size(31, &self.account_bind_url);
        }
        if !self.cdkey_url.is_empty() {
            my_size += ::protobuf::rt::string_size(32, &self.cdkey_url);
        }
        if !self.privacy_policy_url.is_empty() {
            my_size += ::protobuf::rt::string_size(33, &self.privacy_policy_url);
        }
        if !self.next_resource_url.is_empty() {
            my_size += ::protobuf::rt::string_size(34, &self.next_resource_url);
        }
        if let Some(v) = self.next_res_version_config.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.game_biz.is_empty() {
            my_size += ::protobuf::rt::string_size(36, &self.game_biz);
        }
        if !self.gateserver_ipv6_ip.is_empty() {
            my_size += ::protobuf::rt::string_size(37, &self.gateserver_ipv6_ip);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.gateserver_ip.is_empty() {
            os.write_string(1, &self.gateserver_ip)?;
        }
        if self.gateserver_port != 0 {
            os.write_uint32(2, self.gateserver_port)?;
        }
        if !self.pay_callback_url.is_empty() {
            os.write_string(3, &self.pay_callback_url)?;
        }
        if !self.area_type.is_empty() {
            os.write_string(7, &self.area_type)?;
        }
        if !self.resource_url.is_empty() {
            os.write_string(8, &self.resource_url)?;
        }
        if !self.data_url.is_empty() {
            os.write_string(9, &self.data_url)?;
        }
        if !self.feedback_url.is_empty() {
            os.write_string(10, &self.feedback_url)?;
        }
        if !self.bulletin_url.is_empty() {
            os.write_string(11, &self.bulletin_url)?;
        }
        if !self.resource_url_bak.is_empty() {
            os.write_string(12, &self.resource_url_bak)?;
        }
        if !self.data_url_bak.is_empty() {
            os.write_string(13, &self.data_url_bak)?;
        }
        if self.client_data_version != 0 {
            os.write_uint32(14, self.client_data_version)?;
        }
        if !self.handbook_url.is_empty() {
            os.write_string(16, &self.handbook_url)?;
        }
        if self.client_silence_data_version != 0 {
            os.write_uint32(18, self.client_silence_data_version)?;
        }
        if !self.client_data_md5.is_empty() {
            os.write_string(19, &self.client_data_md5)?;
        }
        if !self.client_silence_data_md5.is_empty() {
            os.write_string(20, &self.client_silence_data_md5)?;
        }
        if let Some(v) = self.res_version_config.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(22, v, os)?;
        }
        if !self.secret_key.is_empty() {
            os.write_bytes(23, &self.secret_key)?;
        }
        if !self.official_community_url.is_empty() {
            os.write_string(24, &self.official_community_url)?;
        }
        if !self.client_version_suffix.is_empty() {
            os.write_string(26, &self.client_version_suffix)?;
        }
        if !self.client_silence_version_suffix.is_empty() {
            os.write_string(27, &self.client_silence_version_suffix)?;
        }
        if self.use_gateserver_domain_name != false {
            os.write_bool(28, self.use_gateserver_domain_name)?;
        }
        if !self.gateserver_domain_name.is_empty() {
            os.write_string(29, &self.gateserver_domain_name)?;
        }
        if !self.user_center_url.is_empty() {
            os.write_string(30, &self.user_center_url)?;
        }
        if !self.account_bind_url.is_empty() {
            os.write_string(31, &self.account_bind_url)?;
        }
        if !self.cdkey_url.is_empty() {
            os.write_string(32, &self.cdkey_url)?;
        }
        if !self.privacy_policy_url.is_empty() {
            os.write_string(33, &self.privacy_policy_url)?;
        }
        if !self.next_resource_url.is_empty() {
            os.write_string(34, &self.next_resource_url)?;
        }
        if let Some(v) = self.next_res_version_config.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(35, v, os)?;
        }
        if !self.game_biz.is_empty() {
            os.write_string(36, &self.game_biz)?;
        }
        if !self.gateserver_ipv6_ip.is_empty() {
            os.write_string(37, &self.gateserver_ipv6_ip)?;
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

    fn new() -> RegionInfo {
        RegionInfo::new()
    }

    fn clear(&mut self) {
        self.gateserver_ip.clear();
        self.gateserver_port = 0;
        self.pay_callback_url.clear();
        self.area_type.clear();
        self.resource_url.clear();
        self.data_url.clear();
        self.feedback_url.clear();
        self.bulletin_url.clear();
        self.resource_url_bak.clear();
        self.data_url_bak.clear();
        self.client_data_version = 0;
        self.handbook_url.clear();
        self.client_silence_data_version = 0;
        self.client_data_md5.clear();
        self.client_silence_data_md5.clear();
        self.res_version_config.clear();
        self.secret_key.clear();
        self.official_community_url.clear();
        self.client_version_suffix.clear();
        self.client_silence_version_suffix.clear();
        self.use_gateserver_domain_name = false;
        self.gateserver_domain_name.clear();
        self.user_center_url.clear();
        self.account_bind_url.clear();
        self.cdkey_url.clear();
        self.privacy_policy_url.clear();
        self.next_resource_url.clear();
        self.next_res_version_config.clear();
        self.game_biz.clear();
        self.gateserver_ipv6_ip.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RegionInfo {
        static instance: RegionInfo = RegionInfo {
            gateserver_ip: ::std::string::String::new(),
            gateserver_port: 0,
            pay_callback_url: ::std::string::String::new(),
            area_type: ::std::string::String::new(),
            resource_url: ::std::string::String::new(),
            data_url: ::std::string::String::new(),
            feedback_url: ::std::string::String::new(),
            bulletin_url: ::std::string::String::new(),
            resource_url_bak: ::std::string::String::new(),
            data_url_bak: ::std::string::String::new(),
            client_data_version: 0,
            handbook_url: ::std::string::String::new(),
            client_silence_data_version: 0,
            client_data_md5: ::std::string::String::new(),
            client_silence_data_md5: ::std::string::String::new(),
            res_version_config: ::protobuf::MessageField::none(),
            secret_key: ::std::vec::Vec::new(),
            official_community_url: ::std::string::String::new(),
            client_version_suffix: ::std::string::String::new(),
            client_silence_version_suffix: ::std::string::String::new(),
            use_gateserver_domain_name: false,
            gateserver_domain_name: ::std::string::String::new(),
            user_center_url: ::std::string::String::new(),
            account_bind_url: ::std::string::String::new(),
            cdkey_url: ::std::string::String::new(),
            privacy_policy_url: ::std::string::String::new(),
            next_resource_url: ::std::string::String::new(),
            next_res_version_config: ::protobuf::MessageField::none(),
            game_biz: ::std::string::String::new(),
            gateserver_ipv6_ip: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RegionInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RegionInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RegionInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegionInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10RegionInfo.proto\x1a\x16ResVersionConfig.proto\"\xbe\n\n\nRegionIn\
    fo\x12#\n\rgateserver_ip\x18\x01\x20\x01(\tR\x0cgateserverIp\x12'\n\x0fg\
    ateserver_port\x18\x02\x20\x01(\rR\x0egateserverPort\x12(\n\x10pay_callb\
    ack_url\x18\x03\x20\x01(\tR\x0epayCallbackUrl\x12\x1b\n\tarea_type\x18\
    \x07\x20\x01(\tR\x08areaType\x12!\n\x0cresource_url\x18\x08\x20\x01(\tR\
    \x0bresourceUrl\x12\x19\n\x08data_url\x18\t\x20\x01(\tR\x07dataUrl\x12!\
    \n\x0cfeedback_url\x18\n\x20\x01(\tR\x0bfeedbackUrl\x12!\n\x0cbulletin_u\
    rl\x18\x0b\x20\x01(\tR\x0bbulletinUrl\x12(\n\x10resource_url_bak\x18\x0c\
    \x20\x01(\tR\x0eresourceUrlBak\x12\x20\n\x0cdata_url_bak\x18\r\x20\x01(\
    \tR\ndataUrlBak\x12.\n\x13client_data_version\x18\x0e\x20\x01(\rR\x11cli\
    entDataVersion\x12!\n\x0chandbook_url\x18\x10\x20\x01(\tR\x0bhandbookUrl\
    \x12=\n\x1bclient_silence_data_version\x18\x12\x20\x01(\rR\x18clientSile\
    nceDataVersion\x12&\n\x0fclient_data_md5\x18\x13\x20\x01(\tR\rclientData\
    Md5\x125\n\x17client_silence_data_md5\x18\x14\x20\x01(\tR\x14clientSilen\
    ceDataMd5\x12?\n\x12res_version_config\x18\x16\x20\x01(\x0b2\x11.ResVers\
    ionConfigR\x10resVersionConfig\x12\x1d\n\nsecret_key\x18\x17\x20\x01(\
    \x0cR\tsecretKey\x124\n\x16official_community_url\x18\x18\x20\x01(\tR\
    \x14officialCommunityUrl\x122\n\x15client_version_suffix\x18\x1a\x20\x01\
    (\tR\x13clientVersionSuffix\x12A\n\x1dclient_silence_version_suffix\x18\
    \x1b\x20\x01(\tR\x1aclientSilenceVersionSuffix\x12;\n\x1ause_gateserver_\
    domain_name\x18\x1c\x20\x01(\x08R\x17useGateserverDomainName\x124\n\x16g\
    ateserver_domain_name\x18\x1d\x20\x01(\tR\x14gateserverDomainName\x12&\n\
    \x0fuser_center_url\x18\x1e\x20\x01(\tR\ruserCenterUrl\x12(\n\x10account\
    _bind_url\x18\x1f\x20\x01(\tR\x0eaccountBindUrl\x12\x1b\n\tcdkey_url\x18\
    \x20\x20\x01(\tR\x08cdkeyUrl\x12,\n\x12privacy_policy_url\x18!\x20\x01(\
    \tR\x10privacyPolicyUrl\x12*\n\x11next_resource_url\x18\"\x20\x01(\tR\
    \x0fnextResourceUrl\x12H\n\x17next_res_version_config\x18#\x20\x01(\x0b2\
    \x11.ResVersionConfigR\x14nextResVersionConfig\x12\x19\n\x08game_biz\x18\
    $\x20\x01(\tR\x07gameBiz\x12,\n\x12gateserver_ipv6_ip\x18%\x20\x01(\tR\
    \x10gateserverIpv6IpB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::ResVersionConfig::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RegionInfo::generated_message_descriptor_data());
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

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

//! Parse config files from SRData ExcelOutput
//!
//! All `...Config.json` files have a `...Config` struct and a `...ConfigMap` struct counterpart.
//! The `...Config` struct represents a single map entry, while the `...ConfigMap` struct represents the
//! (nested) map all config entries are stored in, in the .json file.
//! Both types can be deserialized into.
//!
//! The `...Config` structs are written manually right now.
//! It might be possible to automate this process.
//!
//! The `...ConfigMap` structs are automatically generated using a macro.
//! They contain a `get` method to get a specific entry from the (nested) map
//! with the specified keys.
//!
//! Floats and text-map entries are stored in a special way.
//! Please refer to [`Float`] and [`TextMapEntry`].
//!
//! ## Example
//!
//! Using RelicMainAffixConfig as an example
//!
//! ```no_run
//! use artifactarium::resource::excel::AvatarExcelConfigData;
//! let content = "";
//! let config_map: AvatarExcelConfigData = serde_json::from_str(&content).unwrap();
//! let config = config_map.get(&0, &1).unwrap(); // GroupID, AffixID
//! ```

use serde::Deserialize;

use artifactarium_proc_macro::Resource;

use crate::resource::{Float, TextMapEntry};

#[derive(Deserialize, Debug)]
pub struct propGrowCurves {
    #[serde(rename = "type")]
    pub r#type: String,
    pub growCurve: String,
}

#[derive(Resource, Deserialize, Debug)]
pub struct AvatarExcelConfigData {
    pub bodyType: String,
    pub scriptDataPathHash: u64,
    pub iconName: String,
    pub sideIconName: String,
    pub qualityType: String,
    pub chargeEfficiency: u32,
    pub combatConfigHash: u64,
    pub initialWeapon: u32,
    pub weaponType: String,
    pub manekinPathHash: u64,
    pub imageName: String,
    pub gachaCardNameHash: u64,
    pub gachaImageNameHash: u64,
    pub coopPicNameHash: u64,
    pub cutsceneShow: String,
    pub skillDepotId: u32,
    pub staminaRecoverSpeed: u32,
    pub candSkillDepotIds: Vec<u32>,
    pub manekinJsonConfigHash: u64,
    pub manekinMotionConfig: u32,
    pub descTextMapHash: TextMapEntry,
    pub avatarIdentityType: String,
    pub avatarPromoteId: u32,
    pub avatarPromoteRewardLevelList: Vec<u32>,
    pub avatarPromoteRewardIdList: Vec<u32>,
    pub featureTagGroupID: u32,
    pub infoDescTextMapHash: TextMapEntry,
    pub LFIKJDGADNH: u64,
    pub INLHDHDDIHJ: u64,
    pub hpBase: u32,
    pub attackBase: u32,
    pub defenseBase: u32,
    pub critical: f32,
    pub criticalHurt: f32,
    pub propGrowCurves: Vec<propGrowCurves>,
    pub prefabPathRagdollHash: u64,
    pub deformationMeshPathHash: u64,
    #[resource_key]
    pub id: u32,
    pub nameTextMapHash: TextMapEntry,
    pub prefabPathHash: u64,
    pub prefabPathRemoteHash: u64,
    pub controllerPathHash: u64,
    pub controllerPathRemoteHash: u64,
    pub lodPatternName: String,
}

#[derive(Deserialize, Debug)]
pub struct weaponProp {
    pub propType: String,
    pub initValue: f32,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Resource, Deserialize, Debug)]
pub struct WeaponExcelConfigData {
    pub weaponType: String,
    pub rankLevel: u32,
    pub weaponBaseExp: u32,
    pub skillAffix: Vec<u32>,
    pub weaponProp: Vec<weaponProp>,
    pub awakenTexture: String,
    pub awakenLightMapTexture: String,
    pub awakenIcon: String,
    pub weaponPromoteId: u32,
    pub storyId: u32,
    pub awakenCosts: Vec<u32>,
    pub gachaCardNameHash: u64,
    pub destroyRule: String,
    pub destroyReturnMaterial: Vec<u32>,
    pub destroyReturnMaterialCount: Vec<u32>,
    #[resource_key]
    pub id: u32,
    pub nameTextMapHash: TextMapEntry,
    pub descTextMapHash: TextMapEntry,
    pub icon: String,
    pub itemType: String,
    pub weight: u32,
    pub rank: u32,
    pub gadgetId: u32,
}

#[derive(Resource, Deserialize, Debug)]
pub struct EquipAffixExcelConfigData {
    #[resource_key]
    pub GroupID: u32,
    #[resource_key]
    pub AffixID: u32,
    pub Property: String,
    pub BaseValue: Float,
    pub LevelAdd: Float,
    pub IsAvailable: bool,
}

#[derive(Deserialize, Debug)]
#[derive(Default)]
#[serde(default)]
pub struct addProps {
    pub propType: Option<String>,
    pub value: Option<f32>,
}

#[derive(Resource, Deserialize, Debug)]
pub struct ReliquarySetExcelConfigData {
    #[resource_key]
    pub affixId: u32,
    #[resource_key]
    pub id: u32,
    pub nameTextMapHash: TextMapEntry,
    pub descTextMapHash: TextMapEntry,
    pub openConfig: String,
    pub addProps: Vec<addProps>,
    pub paramList: Vec<u32>,
}

#[derive(Resource, Deserialize, Debug)]
pub struct ReliquaryExcelConfigData {
    pub equipType: String,
    pub showPic: String,
    pub rankLevel: u32,
    pub mainPropDepotId: u32,
    pub appendPropDepotId: u32,
    pub setId: u32,
    pub addPropLevels: Vec<u32>,
    pub baseConvExp: u32,
    pub maxLevel: u32,
    pub storyId: u32,
    pub destroyRule: Option<String>,
    pub destroyReturnMaterial: Vec<u32>,
    pub destroyReturnMaterialCount: Vec<u32>,
    #[resource_key]
    pub id: u32,
    pub nameTextMapHash: TextMapEntry,
    pub descTextMapHash: TextMapEntry,
    pub icon: String,
    pub itemType: String,
    pub weight: u32,
    pub rank: u32,
    pub gadgetId: u32,
}


#[derive(Resource, Deserialize, Debug)]
pub struct AvatarSkillExcelConfigData {
    #[resource_key]
    pub id: u32,
    pub nameTextMapHash: TextMapEntry,
    pub abilityName: String,
    pub descTextMapHash: TextMapEntry,
    pub skillIcon: String,
    pub maxChargeNum: u32,
    pub lockShape: String,
    pub lockWeightParams: Vec<u32>,
    pub isAttackCameraLock: bool,
    pub buffIcon: String,
    pub proudSkillGroupId: u32,
    pub globalValueKey: String,
}

#[derive(Deserialize, Debug)]
pub struct itemUse {
    pub useOp: Option<String>,
    pub useParam: Vec<String>,
}

#[derive(Resource, Deserialize, Debug)]
pub struct MaterialExcelConfigData {
    pub interactionTitleTextMapHash: TextMapEntry,
    pub materialType: String,
    pub stackLimit: u32,
    pub itemUse: Vec<itemUse>,
    pub effectDescTextMapHash: TextMapEntry,
    pub specialDescTextMapHash: TextMapEntry,
    pub typeDescTextMapHash: TextMapEntry,
    pub effectIcon: String,
    pub effectName: String,
    pub picPath: Vec<String>,
    #[serde(default)]
    pub isSplitDrop: bool,
    pub satiationParams: Vec<u32>,
    pub destroyRule: String,
    pub destroyReturnMaterial: Vec<u32>,
    pub destroyReturnMaterialCount: Vec<u32>,
    pub setID: Option<u32>,
    #[resource_key]
    pub id: u32,
    pub nameTextMapHash: TextMapEntry,
    pub descTextMapHash: TextMapEntry,
    pub icon: String,
    pub itemType: String,
    pub weight: u32,
    pub rank: u32,
}

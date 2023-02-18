use nanoserde::DeJson;
#[derive(Debug, DeJson, Clone)]
pub struct Vrm {
    #[nserde(rename = "exporterVersion")]
    pub exporter_version: String,
    #[nserde(rename = "specVersion")]
    pub spec_version: String,
    #[nserde(rename = "firstPerson")]
    pub first_person: FirstPerson,
    pub humanoid: Humanoid,
}

#[derive(Debug, DeJson, Clone)]
pub struct FirstPerson {
    #[nserde(rename = "firstPersonBone")]
    pub first_person_bone: u32,
    #[nserde(rename = "firstPersonBoneOffset")]
    pub first_person_bone_offset: Vec3
}

#[derive(Debug, DeJson, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Debug, DeJson, Clone)]
pub struct Humanoid {
    #[nserde(rename = "humanBones")]
    pub human_bones: Vec<Bone>,
}
#[derive(Debug, DeJson, Clone)]
pub struct Bone {
    pub bone: String,
    pub node: u32,
    #[nserde(rename = "useDefaultValues")]
    pub use_default_values: bool
}
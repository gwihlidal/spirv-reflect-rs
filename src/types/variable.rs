use crate::ffi;
use crate::types::descriptor::ReflectDescriptorSet;
use crate::types::image::ReflectFormat;
use crate::types::op::{ReflectBuiltIn, ReflectOp};
use crate::types::traits::*;

bitflags! {
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    pub struct ReflectDecorationFlags: u32 {
        const NONE = 0;
        const BLOCK = 1;
        const BUFFER_BLOCK = 2;
        const ROW_MAJOR = 4;
        const COLUMN_MAJOR = 8;
        const BUILT_IN = 16;
        const NO_PERSPECTIVE = 32;
        const FLAT = 64;
        const NON_WRITABLE = 128;
    }
}

impl Default for ReflectDecorationFlags {
    fn default() -> Self {
        ReflectDecorationFlags::NONE
    }
}

bitflags! {
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    pub struct ReflectTypeFlags: u32 {
        const UNDEFINED = 0;
        const VOID = 1;
        const BOOL = 2;
        const INT = 4;
        const FLOAT = 8;
        const VECTOR = 256;
        const MATRIX = 512;
        const EXTERNAL_IMAGE = 65536;
        const EXTERNAL_SAMPLER = 131_072;
        const EXTERNAL_SAMPLED_IMAGE = 262_144;
        const EXTERNAL_BLOCK = 524_288;
        const EXTERNAL_ACCELERATION_STRUCTURE_NV = 1_048_576;
        const EXTERNAL_MASK = 2_031_616;
        const STRUCT = 268_435_456;
        const ARRAY = 536_870_912;
    }
}

impl Default for ReflectTypeFlags {
    fn default() -> Self {
        ReflectTypeFlags::UNDEFINED
    }
}

bitflags! {
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    pub struct ReflectShaderStageFlags: u32 {
        const UNDEFINED = 0x0000_0000;
        const VERTEX = 0x0000_0001;
        const TESSELLATION_CONTROL = 0x0000_0002;
        const TESSELLATION_EVALUATION = 0x0000_0004;
        const GEOMETRY = 0x0000_0008;
        const FRAGMENT = 0x0000_0010;
        const COMPUTE = 0x0000_0020;
        const RAYGEN_BIT_NV = 256;
        const ANY_HIT_BIT_NV = 512;
        const CLOSEST_HIT_BIT_NV = 1024;
        const MISS_BIT_NV = 2048;
        const INTERSECTION_BIT_NV = 4096;
        const CALLABLE_BIT_NV = 8192;
    }
}

impl Default for ReflectShaderStageFlags {
    fn default() -> Self {
        ReflectShaderStageFlags::UNDEFINED
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ReflectDimension {
    Undefined,
    Type1d,
    Type2d,
    Type3d,
    Cube,
    Rect,
    Buffer,
    SubPassData,
}

impl Default for ReflectDimension {
    fn default() -> Self {
        ReflectDimension::Undefined
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ReflectTypeDescription {
    pub id: u32,
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub op: ReflectOp, // TODO: Serialization support
    pub type_name: String,
    pub struct_member_name: String,
    pub storage_class: ReflectStorageClass,
    pub type_flags: ReflectTypeFlags,
    pub decoration_flags: ReflectDecorationFlags,
    pub traits: ReflectTypeDescriptionTraits,
    pub members: Vec<ReflectTypeDescription>,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ReflectBlockVariable {
    pub spirv_id: u32,
    pub name: String,
    pub offset: u32,
    pub absolute_offset: u32,
    pub size: u32,
    pub padded_size: u32,
    pub decoration_flags: ReflectDecorationFlags,
    pub numeric: ReflectNumericTraits,
    pub array: ReflectArrayTraits,
    pub members: Vec<ReflectBlockVariable>,
    pub type_description: Option<ReflectTypeDescription>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ReflectStorageClass {
    Undefined,
    UniformConstant,
    Input,
    Uniform,
    Output,
    WorkGroup,
    CrossWorkGroup,
    Private,
    Function,
    Generic,
    PushConstant,
    AtomicCounter,
    Image,
    StorageBuffer,
}

impl Default for ReflectStorageClass {
    fn default() -> Self {
        ReflectStorageClass::Undefined
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ReflectInterfaceVariable {
    pub spirv_id: u32,
    pub name: String,
    pub location: u32,
    pub storage_class: ReflectStorageClass,
    pub semantic: String,
    pub decoration_flags: ReflectDecorationFlags,
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub built_in: ReflectBuiltIn, // TODO: Serialization support
    pub numeric: ReflectNumericTraits,
    pub array: ReflectArrayTraits,
    pub members: Vec<ReflectInterfaceVariable>,
    pub format: ReflectFormat,
    pub type_description: Option<ReflectTypeDescription>,
    pub word_offset: u32,
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub(crate) internal_data: *const ffi::SpvReflectInterfaceVariable,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ReflectEntryPoint {
    pub name: String,
    pub id: u32,
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub spirv_execution_model: spirv_headers::ExecutionModel, // TODO: Serialization support
    pub shader_stage: ReflectShaderStageFlags,
    pub input_variables: Vec<ReflectInterfaceVariable>,
    pub output_variables: Vec<ReflectInterfaceVariable>,
    pub descriptor_sets: Vec<ReflectDescriptorSet>,
    pub used_uniforms: Vec<u32>,
    pub used_push_constants: Vec<u32>,
}

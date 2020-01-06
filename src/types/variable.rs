use crate::types::descriptor::ReflectDescriptorSet;
use crate::types::image::ReflectFormat;
use crate::types::op::{ReflectBuiltIn, ReflectOp};
use crate::types::traits::*;

bitflags! {
    #[derive(Serialize)]
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
    #[derive(Serialize)]
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
        const SAMPLED_MASK = 65536 | 262_144;
        const STRUCT = 268_435_456;
        const ARRAY = 536_870_912;
    }
}

impl Default for ReflectTypeFlags {
    fn default() -> Self {
        ReflectTypeFlags::UNDEFINED
    }
}

#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
pub enum ReflectShaderStage {
    Undefined,
    Vertex,
    TessellationControl,
    TessellationEvaluation,
    Geometry,
    Fragment,
    Compute,
    Kernel,
    TaskNV,
    MeshNV,
    RayGenerationNV,
    IntersectionNV,
    AnyHitNV,
    ClosestHitNV,
    MissNV,
    CallableNV,
}

impl Default for ReflectShaderStage {
    fn default() -> Self {
        ReflectShaderStage::Undefined
    }
}

#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
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

impl From<Option<spirv_headers::Dim>> for ReflectDimension {
    fn from(raw: Option<spirv_headers::Dim>) -> ReflectDimension {
        match raw {
            Some(spirv_headers::Dim::Dim1D) => ReflectDimension::Type1d,
            Some(spirv_headers::Dim::Dim2D) => ReflectDimension::Type2d,
            Some(spirv_headers::Dim::Dim3D) => ReflectDimension::Type3d,
            Some(spirv_headers::Dim::DimCube) => ReflectDimension::Cube,
            Some(spirv_headers::Dim::DimRect) => ReflectDimension::Rect,
            Some(spirv_headers::Dim::DimBuffer) => ReflectDimension::Buffer,
            Some(spirv_headers::Dim::DimSubpassData) => ReflectDimension::SubPassData,
            _ => ReflectDimension::Undefined,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ReflectTypeDescription {
    pub id: u32,
    #[serde(skip_serializing)]
    pub op: ReflectOp, // TODO: Serialization support
    pub type_name: String,
    pub struct_member_name: String,
    pub storage_class: ReflectStorageClass,
    pub type_flags: ReflectTypeFlags,
    pub decoration_flags: ReflectDecorationFlags,
    pub traits: ReflectTypeDescriptionTraits,
    pub members: Vec<ReflectTypeDescription>,
}

impl Default for ReflectTypeDescription {
    fn default() -> Self {
        ReflectTypeDescription {
            id: std::u32::MAX,
            op: ReflectOp::default(),
            type_name: String::new(),
            struct_member_name: String::new(),
            storage_class: ReflectStorageClass::default(),
            type_flags: ReflectTypeFlags::default(),
            decoration_flags: ReflectDecorationFlags::default(),
            traits: ReflectTypeDescriptionTraits::default(),
            members: Vec::new(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, PartialEq)]
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
    pub type_description: ReflectTypeDescription,
}

#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
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

impl From<Option<spirv_headers::StorageClass>> for ReflectStorageClass {
    fn from(raw: Option<spirv_headers::StorageClass>) -> ReflectStorageClass {
        match raw {
            Some(spirv_headers::StorageClass::UniformConstant) => {
                ReflectStorageClass::UniformConstant
            }
            Some(spirv_headers::StorageClass::Input) => ReflectStorageClass::Input,
            Some(spirv_headers::StorageClass::Uniform) => ReflectStorageClass::Uniform,
            Some(spirv_headers::StorageClass::Output) => ReflectStorageClass::Output,
            Some(spirv_headers::StorageClass::Workgroup) => ReflectStorageClass::WorkGroup,
            Some(spirv_headers::StorageClass::CrossWorkgroup) => {
                ReflectStorageClass::CrossWorkGroup
            }
            Some(spirv_headers::StorageClass::Private) => ReflectStorageClass::Private,
            Some(spirv_headers::StorageClass::Function) => ReflectStorageClass::Function,
            Some(spirv_headers::StorageClass::Generic) => ReflectStorageClass::Generic,
            Some(spirv_headers::StorageClass::PushConstant) => ReflectStorageClass::PushConstant,
            Some(spirv_headers::StorageClass::AtomicCounter) => ReflectStorageClass::AtomicCounter,
            Some(spirv_headers::StorageClass::Image) => ReflectStorageClass::Image,
            Some(spirv_headers::StorageClass::StorageBuffer) => ReflectStorageClass::StorageBuffer,
            _ => ReflectStorageClass::Undefined,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct ReflectInterfaceVariable {
    pub spirv_id: u32,
    pub name: String,
    pub location: u32,
    pub storage_class: ReflectStorageClass,
    pub semantic: String,
    pub decoration_flags: ReflectDecorationFlags,
    #[serde(skip_serializing)]
    pub built_in: Option<ReflectBuiltIn>, // TODO: Serialization support
    pub numeric: ReflectNumericTraits,
    pub array: ReflectArrayTraits,
    pub members: Vec<ReflectInterfaceVariable>,
    pub format: ReflectFormat,
    pub type_description: ReflectTypeDescription,
    pub word_offset: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReflectEntryPoint {
    pub name: String,
    pub id: u32,
    #[serde(skip_serializing)]
    pub spirv_execution_model: Option<spirv_headers::ExecutionModel>, // TODO: Serialization support
    pub shader_stage: ReflectShaderStage,
    pub input_variables: Vec<ReflectInterfaceVariable>,
    pub output_variables: Vec<ReflectInterfaceVariable>,
    pub descriptor_sets: Vec<ReflectDescriptorSet>,
    pub used_uniforms: Vec<u32>,
    pub used_push_constants: Vec<u32>,
}

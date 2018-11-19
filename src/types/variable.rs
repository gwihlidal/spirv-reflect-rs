use types::op::ReflectOp;
use types::traits::*;

bitflags! {
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
    pub struct ReflectTypeFlags: u32 {
        const UNDEFINED = 0;
        const VOID = 1;
        const BOOL = 2;
        const INT = 4;
        const FLOAT = 8;
        const VECTOR = 256;
        const MATRIX = 512;
        const EXTERNAL_IMAGE = 65536;
        const EXTERNAL_SAMPLER = 131072;
        const EXTERNAL_SAMPLED_IMAGE = 262144;
        const EXTERNAL_BLOCK = 524288;
        const EXTERNAL_MASK = 983040;
        const STRUCT = 268435456;
        const ARRAY = 536870912;
    }
}

impl Default for ReflectTypeFlags {
    fn default() -> Self {
        ReflectTypeFlags::UNDEFINED
    }
}

bitflags! {
    pub struct ReflectShaderStageFlags: u32 {
        const UNDEFINED = 0x00000000;
        const VERTEX = 0x00000001;
        const TESSELLATION_CONTROL = 0x00000002;
        const TESSELLATION_EVALUATION = 0x00000004;
        const GEOMETRY = 0x00000008;
        const FRAGMENT = 0x00000010;
        const COMPUTE = 0x00000020;
    }
}

impl Default for ReflectShaderStageFlags {
    fn default() -> Self {
        ReflectShaderStageFlags::UNDEFINED
    }
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Default, Debug, Clone)]
pub struct ReflectTypeDescription {
    pub id: u32,
    pub op: ReflectOp,
    pub type_name: String,
    pub struct_member_name: String,
    pub storage_class: ReflectStorageClass,
    pub type_flags: ReflectTypeFlags,
    pub decoration_flags: ReflectDecorationFlags,
    pub traits: ReflectTypeDescriptionTraits,
    pub members: Vec<Box<ReflectTypeDescription>>,
}

#[derive(Default, Debug, Clone)]
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
    pub members: Vec<Box<ReflectBlockVariable>>,
    pub type_description: Option<Box<ReflectTypeDescription>>,
}

#[derive(Debug, Copy, Clone)]
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

pub struct ReflectInterfaceVariable {
    //pub word_offset: u32,
}

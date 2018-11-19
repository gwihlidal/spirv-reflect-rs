use types::op::ReflectOp;

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

#[derive(Debug, Copy, Clone)]
pub struct ReflectBindingArrayTraits {
    pub dims_count: u32,
    pub dims: [u32; 32usize],
}

impl Default for ReflectBindingArrayTraits {
    fn default() -> Self {
        ReflectBindingArrayTraits {
            dims_count: 0,
            dims: [0; 32],
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct ReflectNumericTraitsScalar {
    pub width: u32,
    pub signedness: u32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct ReflectNumericTraitsVector {
    pub component_count: u32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct ReflectNumericTraitsMatrix {
    pub column_count: u32,
    pub row_count: u32,
    pub stride: u32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct ReflectNumericTraits {
    pub scalar: ReflectNumericTraitsScalar,
    pub vector: ReflectNumericTraitsVector,
    pub matrix: ReflectNumericTraitsMatrix,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct ReflectArrayTraits {
    pub dims_count: u32,
    pub dims: [u32; 32usize],
    pub stride: u32,
}

#[derive(Default, Debug, Clone)]
pub struct ReflectTypeDescription {
    pub id: u32,
    pub op: ReflectOp,
    pub type_name: String,
    pub struct_member_name: String,
    pub storage_class: ReflectStorageClass,
    //pub type_flags: ReflectTypeFlags,
    pub decoration_flags: ReflectDecorationFlags,
    //pub traits: ReflectTypeDescription_Traits,
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

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
pub enum ReflectBindingArrayTraits {
    Undefined,
}

impl Default for ReflectBindingArrayTraits {
    fn default() -> Self {
        ReflectBindingArrayTraits::Undefined
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct ReflectNumericTraitsScalar {

}

#[derive(Default, Debug, Copy, Clone)]
pub struct ReflectNumericTraitsVector {
    
}

#[derive(Default, Debug, Copy, Clone)]
pub struct ReflectNumericTraitsMatrix {
    
}

#[derive(Default, Debug, Copy, Clone)]
pub struct ReflectNumericTraits {

}

#[derive(Default, Debug, Copy, Clone)]
pub struct ReflectArrayTraits {

}

#[derive(Default, Debug, Copy, Clone)]
pub struct ReflectTypeDescription {}

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


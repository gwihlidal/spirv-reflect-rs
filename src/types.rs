#[derive(Debug, Copy, Clone)]
pub enum ReflectDescriptorType {
    Undefined,
    Sampler,
    CombinedImageSampler,
    SampledImage,
    StorageImage,
    UniformTexelBuffer,
    StorageTexelBuffer,
    UniformBuffer,
    StorageBuffer,
    UniformBufferDynamic,
    StorageBufferDynamic,
    InputAttachment,
}

impl Default for ReflectDescriptorType {
    fn default() -> Self {
        ReflectDescriptorType::Undefined
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ReflectResourceType {
    Undefined,
    Sampler,
    ConstantBufferView,
    ShaderResourceView,
    UnorderedAccessView,
}

impl Default for ReflectResourceType {
    fn default() -> Self {
        ReflectResourceType::Undefined
    }
}

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
pub enum ReflectImageFormat {
    Undefined,
}

impl Default for ReflectImageFormat {
    fn default() -> Self {
        ReflectImageFormat::Undefined
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct SpvReflectImageTraits {
    pub dim: ReflectDimension,
    pub depth: u32,
    pub arrayed: u32,
    pub ms: u32,
    pub sampled: u32,
    pub image_format: ReflectImageFormat,
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

pub type ReflectOrdinalBinding = u32;
pub type ReflectOrdinalSet = u32;
pub type ReflectDescriptorBindingSet = (ReflectOrdinalBinding, ReflectOrdinalSet);

#[derive(Default, Debug, Copy, Clone)]
pub struct ReflectTypeDescription {}

#[derive(Default, Debug, Clone)]
pub struct ReflectDescriptorBinding {
    pub spirv_id: u32,
    pub name: String,
    pub binding: u32,
    pub input_attachment_index: u32,
    pub set: u32,
    pub descriptor_type: ReflectDescriptorType,
    pub resource_type: ReflectResourceType,
    //pub image: ReflectImageTraits,
    //pub block: ReflectBlockVariable,
    //pub array: ReflectBindingArrayTraits,
    pub count: u32,
    //pub uav_counter_id: u32,
    //pub uav_counter_binding: *mut SpvReflectDescriptorBinding,
    //pub type_description: *mut SpvReflectTypeDescription,
    pub word_offset: ReflectDescriptorBindingSet,
}

#[derive(Default, Debug, Clone)]
pub struct ReflectDescriptorSet {
    pub set: u32,
    pub bindings: Vec<ReflectDescriptorBinding>,
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
    //pub numeric: SpvReflectNumericTraits,
    //pub array: SpvReflectArrayTraits,
    //pub member_count: u32,
    //pub members: *mut SpvReflectBlockVariable,
    //pub type_description: *mut SpvReflectTypeDescription,
}
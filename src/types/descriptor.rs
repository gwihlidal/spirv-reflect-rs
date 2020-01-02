use crate::types::{
    ReflectBindingArrayTraits,
    ReflectBlockVariable,
    ReflectImageTraits,
    ReflectResourceTypeFlags,
    //ReflectTypeDescription,
};

#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
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
    AccelerationStructureNV,
}

impl Default for ReflectDescriptorType {
    fn default() -> Self {
        ReflectDescriptorType::Undefined
    }
}

pub type ReflectOrdinalBinding = u32;
pub type ReflectOrdinalSet = u32;
pub type ReflectDescriptorBindingSet = (ReflectOrdinalBinding, ReflectOrdinalSet);

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ReflectDescriptorBinding {
    pub spirv_id: u32,
    pub name: String,
    pub binding: u32,
    pub input_attachment_index: u32,
    pub set: u32,
    pub descriptor_type: ReflectDescriptorType,
    pub resource_type: ReflectResourceTypeFlags,
    pub image: ReflectImageTraits,
    pub block: ReflectBlockVariable,
    pub array: ReflectBindingArrayTraits,
    pub count: u32,
    pub uav_counter_id: u32,
    pub uav_counter_index: usize,
    pub type_index: Option<usize>,
    pub word_offset: ReflectDescriptorBindingSet,
    pub accessed: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ReflectDescriptorSet {
    pub set: u32,
    pub bindings: Vec<usize>, //ReflectDescriptorBinding>,
}

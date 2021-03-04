use crate::ffi;
use crate::types::{
    ReflectBindingArrayTraits, ReflectBlockVariable, ReflectImageTraits, ReflectResourceType,
    ReflectTypeDescription,
};

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ReflectDescriptorBinding {
    pub spirv_id: u32,
    pub name: String,
    pub binding: u32,
    pub input_attachment_index: u32,
    pub set: u32,
    pub descriptor_type: ReflectDescriptorType,
    pub resource_type: ReflectResourceType,
    pub image: ReflectImageTraits,
    pub block: ReflectBlockVariable,
    pub array: ReflectBindingArrayTraits,
    pub count: u32,
    pub uav_counter_id: u32,
    pub uav_counter_binding: Option<Box<ReflectDescriptorBinding>>,
    pub type_description: Option<ReflectTypeDescription>,
    pub word_offset: ReflectDescriptorBindingSet,
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub(crate) internal_data: *const ffi::SpvReflectDescriptorBinding,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ReflectDescriptorSet {
    pub set: u32,
    pub bindings: Vec<ReflectDescriptorBinding>,
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub(crate) internal_data: *const ffi::SpvReflectDescriptorSet,
}

use ffi;
use types::*;

pub(crate) fn ffi_to_descriptor_type(
    ffi_type: ffi::SpvReflectDescriptorType,
) -> ReflectDescriptorType {
    match ffi_type {
        ffi::SpvReflectDescriptorType_SPV_REFLECT_DESCRIPTOR_TYPE_SAMPLER => {
            ReflectDescriptorType::Sampler
        }
        ffi::SpvReflectDescriptorType_SPV_REFLECT_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER => {
            ReflectDescriptorType::CombinedImageSampler
        }
        ffi::SpvReflectDescriptorType_SPV_REFLECT_DESCRIPTOR_TYPE_SAMPLED_IMAGE => {
            ReflectDescriptorType::SampledImage
        }
        ffi::SpvReflectDescriptorType_SPV_REFLECT_DESCRIPTOR_TYPE_STORAGE_IMAGE => {
            ReflectDescriptorType::StorageImage
        }
        ffi::SpvReflectDescriptorType_SPV_REFLECT_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER => {
            ReflectDescriptorType::UniformTexelBuffer
        }
        ffi::SpvReflectDescriptorType_SPV_REFLECT_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER => {
            ReflectDescriptorType::StorageTexelBuffer
        }
        ffi::SpvReflectDescriptorType_SPV_REFLECT_DESCRIPTOR_TYPE_UNIFORM_BUFFER => {
            ReflectDescriptorType::UniformBuffer
        }
        ffi::SpvReflectDescriptorType_SPV_REFLECT_DESCRIPTOR_TYPE_STORAGE_BUFFER => {
            ReflectDescriptorType::StorageBuffer
        }
        ffi::SpvReflectDescriptorType_SPV_REFLECT_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC => {
            ReflectDescriptorType::UniformBufferDynamic
        }
        ffi::SpvReflectDescriptorType_SPV_REFLECT_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC => {
            ReflectDescriptorType::StorageBufferDynamic
        }
        ffi::SpvReflectDescriptorType_SPV_REFLECT_DESCRIPTOR_TYPE_INPUT_ATTACHMENT => {
            ReflectDescriptorType::InputAttachment
        }
        _ => unimplemented!(),
    }
}

pub(crate) fn ffi_to_resource_type(
    ffi_type: ffi::SpvReflectResourceType,
) -> ReflectResourceType {
    match ffi_type {
        ffi::SpvReflectResourceType_SPV_REFLECT_RESOURCE_FLAG_UNDEFINED => {
            ReflectResourceType::Undefined
        },
        ffi::SpvReflectResourceType_SPV_REFLECT_RESOURCE_FLAG_SAMPLER => {
            ReflectResourceType::Sampler
        },
        ffi::SpvReflectResourceType_SPV_REFLECT_RESOURCE_FLAG_CBV => {
            ReflectResourceType::ConstantBufferView
        },
        ffi::SpvReflectResourceType_SPV_REFLECT_RESOURCE_FLAG_SRV => {
            ReflectResourceType::ShaderResourceView
        },
        ffi::SpvReflectResourceType_SPV_REFLECT_RESOURCE_FLAG_UAV => {
            ReflectResourceType::UnorderedAccessView
        },
        _ => unimplemented!(),
    }
}

pub(crate) fn ffi_to_dimension(
    ffi_type: ffi::SpvDim,
) -> ReflectDimension {
    match ffi_type {
        ffi::SpvDim__SpvDim1D => {
            ReflectDimension::Type1d
        },
        ffi::SpvDim__SpvDim2D => {
            ReflectDimension::Type2d
        },
        ffi::SpvDim__SpvDim3D => {
            ReflectDimension::Type3d
        },
        ffi::SpvDim__SpvDimCube => {
            ReflectDimension::Cube
        },
        ffi::SpvDim__SpvDimRect => {
            ReflectDimension::Rect
        },
        ffi::SpvDim__SpvDimBuffer => {
            ReflectDimension::Buffer
        },
        ffi::SpvDim__SpvDimSubpassData => {
            ReflectDimension::SubPassData
        },
        _ => unimplemented!(),
    }
}

pub(crate) fn ffi_to_image_traits(
    ffi_type: ffi::SpvReflectImageTraits,
) -> ReflectImageTraits {
    ReflectImageTraits {
        dim: ffi_to_dimension(ffi_type.dim),
        depth: ffi_type.depth,
        arrayed: ffi_type.arrayed,
        ms: ffi_type.ms,
        sampled: ffi_type.sampled,
        image_format: ffi_to_format(ffi_type.image_format),
    }
}

pub(crate) fn ffi_to_format(
    ffi_type: ffi::SpvReflectFormat,
) -> ReflectFormat {
    ReflectFormat::Undefined
}

pub(crate) fn ffi_to_decoration_flags(
    ffi_type: ffi::SpvReflectDecorationFlags,
) -> ReflectDecorationFlags {
    ReflectDecorationFlags::NONE
}

pub(crate) fn ffi_to_numeric_traits(
    ffi_type: ffi::SpvReflectNumericTraits,
) -> ReflectNumericTraits {
    ReflectNumericTraits::default()
}

pub(crate) fn ffi_to_array_traits(
    ffi_type: ffi::SpvReflectArrayTraits,
) -> ReflectArrayTraits {
    ReflectArrayTraits::default()
}

pub(crate) fn ffi_to_binding_array_traits(
    ffi_type: ffi::SpvReflectBindingArrayTraits,
) -> ReflectBindingArrayTraits {
    ReflectBindingArrayTraits::default()
}

pub(crate) fn ffi_to_block_variable(
    ffi_type: ffi::SpvReflectBlockVariable,
) -> ReflectBlockVariable {
    let c_str: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(ffi_type.name) };
    let str_slice: &str = c_str.to_str().unwrap();
    //member_count: ffi_type.member_count,
    let members: Vec<Box<ReflectBlockVariable>> = Vec::new();
    let type_description = Some(Box::new(ReflectTypeDescription::default()));
    ReflectBlockVariable {
        spirv_id: ffi_type.spirv_id,
        name: str_slice.to_owned(),
        offset: ffi_type.offset,
        absolute_offset: ffi_type.absolute_offset,
        size: ffi_type.size,
        padded_size: ffi_type.padded_size,
        decoration_flags: ffi_to_decoration_flags(ffi_type.decoration_flags),
        numeric: ffi_to_numeric_traits(ffi_type.numeric),
        array: ffi_to_array_traits(ffi_type.array),
        members,
        type_description,
    }
}

pub fn result_to_string(result: ffi::SpvReflectResult) -> &'static str {
    match result {
        ffi::SpvReflectResult_SPV_REFLECT_RESULT_SUCCESS => "Success",
        ffi::SpvReflectResult_SPV_REFLECT_RESULT_NOT_READY => "Result Not Ready",
        ffi::SpvReflectResult_SPV_REFLECT_RESULT_ERROR_PARSE_FAILED => "Parse Failed",
        ffi::SpvReflectResult_SPV_REFLECT_RESULT_ERROR_ALLOC_FAILED => "Allocation Failed",
        ffi::SpvReflectResult_SPV_REFLECT_RESULT_ERROR_RANGE_EXCEEDED => "Range Exceeded",
        ffi::SpvReflectResult_SPV_REFLECT_RESULT_ERROR_NULL_POINTER => "Null Pointer",
        ffi::SpvReflectResult_SPV_REFLECT_RESULT_ERROR_INTERNAL_ERROR => "Internal Error",
        ffi::SpvReflectResult_SPV_REFLECT_RESULT_ERROR_COUNT_MISMATCH => "Count Mismatch",
        ffi::SpvReflectResult_SPV_REFLECT_RESULT_ERROR_ELEMENT_NOT_FOUND => "Element Not Found",
        ffi::SpvReflectResult_SPV_REFLECT_RESULT_ERROR_SPIRV_INVALID_CODE_SIZE => {
            "Invalid Code Size"
        }
        ffi::SpvReflectResult_SPV_REFLECT_RESULT_ERROR_SPIRV_INVALID_MAGIC_NUMBER => {
            "Invalid Magic Number"
        }
        ffi::SpvReflectResult_SPV_REFLECT_RESULT_ERROR_SPIRV_UNEXPECTED_EOF => "Unexpected EoF",
        ffi::SpvReflectResult_SPV_REFLECT_RESULT_ERROR_SPIRV_INVALID_ID_REFERENCE => {
            "Invalid ID Reference"
        }
        ffi::SpvReflectResult_SPV_REFLECT_RESULT_ERROR_SPIRV_SET_NUMBER_OVERFLOW => {
            "Set Number Overflow"
        }
        ffi::SpvReflectResult_SPV_REFLECT_RESULT_ERROR_SPIRV_INVALID_STORAGE_CLASS => {
            "Invalid Storage Class"
        }
        ffi::SpvReflectResult_SPV_REFLECT_RESULT_ERROR_SPIRV_RECURSION => "Spirv Recursion",
        _ => unimplemented!(),
    }
}

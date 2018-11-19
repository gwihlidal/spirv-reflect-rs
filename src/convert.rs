use ffi;
use types::*;

pub(crate) fn ffi_to_generator(ffi_type: ffi::SpvReflectGenerator) -> ReflectGenerator {
    match ffi_type {
        SpvReflectGenerator_SPV_REFLECT_GENERATOR_KHRONOS_LLVM_SPIRV_TRANSLATOR => ReflectGenerator::KHRONOS_LLVM_SPIRV_TRANSLATOR,
        SpvReflectGenerator_SPV_REFLECT_GENERATOR_KHRONOS_KHRONOS_SPIRV_TOOLS_ASSEMBLER => ReflectGenerator::KHRONOS_SPIRV_TOOLS_ASSEMBLER,
        SpvReflectGenerator_SPV_REFLECT_GENERATOR_KHRONOS_KHRONOS_GLSLANG_REFERENCE_FRONT_END => ReflectGenerator::KHRONOS_GLSLANG_REFERENCE_FRONT_END,
        SpvReflectGenerator_SPV_REFLECT_GENERATOR_KHRONOS_GOOGLE_SHADERC_OVER_GLSLANG => ReflectGenerator::GOOGLE_SHADERC_OVER_GLSLANG,
        SpvReflectGenerator_SPV_REFLECT_GENERATOR_KHRONOS_GOOGLE_SPIREGG => ReflectGenerator::GOOGLE_SPIREGG,
        SpvReflectGenerator_SPV_REFLECT_GENERATOR_KHRONOS_GOOGLE_RSPIRV => ReflectGenerator::GOOGLE_RSPIRV,
        SpvReflectGenerator_SPV_REFLECT_GENERATOR_KHRONOS_X_LEGEND_MESA_MESAIR_SPIRV_TRANSLATOR => ReflectGenerator::X_LEGEND_MESA_MESAIR_SPIRV_TRANSLATOR,
        SpvReflectGenerator_SPV_REFLECT_GENERATOR_KHRONOS_KHRONOS_SPIRV_TOOLS_LINKER => ReflectGenerator::KHRONOS_SPIRV_TOOLS_LINKER,
        SpvReflectGenerator_SPV_REFLECT_GENERATOR_KHRONOS_WINE_VKD3D_SHADER_COMPILER => ReflectGenerator::WINE_VKD3D_SHADER_COMPILER,
        SpvReflectGenerator_SPV_REFLECT_GENERATOR_KHRONOS_CLAY_CLAY_SHADER_COMPILER => ReflectGenerator::CLAY_CLAY_SHADER_COMPILER,
        _ => ReflectGenerator::UNKNOWN,
    }
}

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
        image_format: ffi_to_image_format(ffi_type.image_format),
    }
}

pub(crate) fn ffi_to_image_format(
    ffi_type: ffi::SpvImageFormat,
) -> ReflectImageFormat {
    match ffi_type {
        ffi::SpvImageFormat__SpvImageFormatUnknown => ReflectImageFormat::Undefined,
        ffi::SpvImageFormat__SpvImageFormatRgba32f => ReflectImageFormat::RGBA32_FLOAT,
        ffi::SpvImageFormat__SpvImageFormatRgba16f => ReflectImageFormat::RGBA16_FLOAT,
        ffi::SpvImageFormat__SpvImageFormatR32f => ReflectImageFormat::R32_FLOAT,
        ffi::SpvImageFormat__SpvImageFormatRgba8 => ReflectImageFormat::RGBA8,
        ffi::SpvImageFormat__SpvImageFormatRgba8Snorm => ReflectImageFormat::RGBA8_SNORM,
        ffi::SpvImageFormat__SpvImageFormatRg32f => ReflectImageFormat::RG32_FLOAT,
        ffi::SpvImageFormat__SpvImageFormatRg16f => ReflectImageFormat::RG16_FLOAT,
        ffi::SpvImageFormat__SpvImageFormatR11fG11fB10f => ReflectImageFormat::R11G11B10_FLOAT,
        ffi::SpvImageFormat__SpvImageFormatR16f => ReflectImageFormat::R16_FLOAT,
        ffi::SpvImageFormat__SpvImageFormatRgba16 => ReflectImageFormat::RGBA16,
        ffi::SpvImageFormat__SpvImageFormatRgb10A2 => ReflectImageFormat::RGB10A2,
        ffi::SpvImageFormat__SpvImageFormatRg16 => ReflectImageFormat::RG16,
        ffi::SpvImageFormat__SpvImageFormatRg8 => ReflectImageFormat::RG8,
        ffi::SpvImageFormat__SpvImageFormatR16 => ReflectImageFormat::R16,
        ffi::SpvImageFormat__SpvImageFormatR8 => ReflectImageFormat::R8,
        ffi::SpvImageFormat__SpvImageFormatRgba16Snorm => ReflectImageFormat::RGBA16_SNORM,
        ffi::SpvImageFormat__SpvImageFormatRg16Snorm => ReflectImageFormat::RG16_SNORM,
        ffi::SpvImageFormat__SpvImageFormatRg8Snorm => ReflectImageFormat::RG8_SNORM,
        ffi::SpvImageFormat__SpvImageFormatR16Snorm => ReflectImageFormat::R16_SNORM,
        ffi::SpvImageFormat__SpvImageFormatR8Snorm => ReflectImageFormat::R8_SNORM,
        ffi::SpvImageFormat__SpvImageFormatRgba32i => ReflectImageFormat::RGBA32_INT,
        ffi::SpvImageFormat__SpvImageFormatRgba16i => ReflectImageFormat::RGBA16_INT,
        ffi::SpvImageFormat__SpvImageFormatRgba8i => ReflectImageFormat::RGBA8_INT,
        ffi::SpvImageFormat__SpvImageFormatR32i => ReflectImageFormat::R32_INT,
        ffi::SpvImageFormat__SpvImageFormatRg32i => ReflectImageFormat::RG32_INT,
        ffi::SpvImageFormat__SpvImageFormatRg16i => ReflectImageFormat::RG16_INT,
        ffi::SpvImageFormat__SpvImageFormatRg8i => ReflectImageFormat::RG8_INT,
        ffi::SpvImageFormat__SpvImageFormatR16i => ReflectImageFormat::R16_INT,
        ffi::SpvImageFormat__SpvImageFormatR8i => ReflectImageFormat::R8_INT,
        ffi::SpvImageFormat__SpvImageFormatRgba32ui => ReflectImageFormat::RGBA32_UINT,
        ffi::SpvImageFormat__SpvImageFormatRgba16ui => ReflectImageFormat::RGBA16_UINT,
        ffi::SpvImageFormat__SpvImageFormatRgba8ui => ReflectImageFormat::RGBA8_UINT,
        ffi::SpvImageFormat__SpvImageFormatR32ui => ReflectImageFormat::R32_UINT,
        ffi::SpvImageFormat__SpvImageFormatRgb10a2ui => ReflectImageFormat::RGB10A2_UINT,
        ffi::SpvImageFormat__SpvImageFormatRg32ui => ReflectImageFormat::RG32_UINT,
        ffi::SpvImageFormat__SpvImageFormatRg16ui => ReflectImageFormat::RG16_UINT,
        ffi::SpvImageFormat__SpvImageFormatRg8ui => ReflectImageFormat::RG8_UINT,
        ffi::SpvImageFormat__SpvImageFormatR16ui => ReflectImageFormat::R16_UINT,
        ffi::SpvImageFormat__SpvImageFormatR8ui => ReflectImageFormat::R8_UINT,
        _ => unimplemented!()
    }
}

pub(crate) fn ffi_to_format(
    ffi_type: ffi::SpvReflectFormat,
) -> ReflectFormat {
    match ffi_type {
        ffi::SpvReflectFormat_SPV_REFLECT_FORMAT_UNDEFINED => ReflectFormat::Undefined,
        ffi::SpvReflectFormat_SPV_REFLECT_FORMAT_R32_UINT => ReflectFormat::R32_UINT,
        ffi::SpvReflectFormat_SPV_REFLECT_FORMAT_R32_SINT => ReflectFormat::R32_SINT,
        ffi::SpvReflectFormat_SPV_REFLECT_FORMAT_R32_SFLOAT => ReflectFormat::R32_SFLOAT,
        ffi::SpvReflectFormat_SPV_REFLECT_FORMAT_R32G32_UINT => ReflectFormat::R32G32_UINT,
        ffi::SpvReflectFormat_SPV_REFLECT_FORMAT_R32G32_SINT => ReflectFormat::R32G32_SINT,
        ffi::SpvReflectFormat_SPV_REFLECT_FORMAT_R32G32_SFLOAT => ReflectFormat::R32G32_SFLOAT,
        ffi::SpvReflectFormat_SPV_REFLECT_FORMAT_R32G32B32_UINT => ReflectFormat::R32G32B32_UINT,
        ffi::SpvReflectFormat_SPV_REFLECT_FORMAT_R32G32B32_SINT => ReflectFormat::R32G32B32_SINT,
        ffi::SpvReflectFormat_SPV_REFLECT_FORMAT_R32G32B32_SFLOAT => ReflectFormat::R32G32B32_SFLOAT,
        ffi::SpvReflectFormat_SPV_REFLECT_FORMAT_R32G32B32A32_UINT => ReflectFormat::R32G32B32A32_UINT,
        ffi::SpvReflectFormat_SPV_REFLECT_FORMAT_R32G32B32A32_SINT => ReflectFormat::R32G32B32A32_SINT,
        ffi::SpvReflectFormat_SPV_REFLECT_FORMAT_R32G32B32A32_SFLOAT => ReflectFormat::R32G32B32A32_SFLOAT,
        _ => unimplemented!()
    }
}

pub(crate) fn ffi_to_storage_class(
    ffi_type: ffi::SpvStorageClass,
) -> ReflectStorageClass {
    match ffi_type {
        ffi::SpvStorageClass__SpvStorageClassUniformConstant => ReflectStorageClass::Undefined,
        ffi::SpvStorageClass__SpvStorageClassInput => ReflectStorageClass::Input,
        ffi::SpvStorageClass__SpvStorageClassUniform => ReflectStorageClass::Uniform,
        ffi::SpvStorageClass__SpvStorageClassOutput => ReflectStorageClass::Output,
        ffi::SpvStorageClass__SpvStorageClassWorkgroup => ReflectStorageClass::WorkGroup,
        ffi::SpvStorageClass__SpvStorageClassCrossWorkgroup => ReflectStorageClass::CrossWorkGroup,
        ffi::SpvStorageClass__SpvStorageClassPrivate => ReflectStorageClass::Private,
        ffi::SpvStorageClass__SpvStorageClassFunction => ReflectStorageClass::Function,
        ffi::SpvStorageClass__SpvStorageClassGeneric => ReflectStorageClass::Generic,
        ffi::SpvStorageClass__SpvStorageClassPushConstant => ReflectStorageClass::PushConstant,
        ffi::SpvStorageClass__SpvStorageClassAtomicCounter => ReflectStorageClass::AtomicCounter,
        ffi::SpvStorageClass__SpvStorageClassImage => ReflectStorageClass::Image,
        ffi::SpvStorageClass__SpvStorageClassStorageBuffer => ReflectStorageClass::StorageBuffer,
        _ => unimplemented!()
    }
}

pub(crate) fn ffi_to_shader_stage_flags(
    ffi_type: ffi::SpvReflectShaderStageFlagBits,
) -> ReflectShaderStageFlags {
    ReflectShaderStageFlags::from_bits(ffi_type as u32).unwrap()
}

pub(crate) fn ffi_to_decoration_flags(
    ffi_type: ffi::SpvReflectDecorationFlags,
) -> ReflectDecorationFlags {
    ReflectDecorationFlags::from_bits(ffi_type).unwrap()
}

pub(crate) fn ffi_to_numeric_traits(
    ffi_type: ffi::SpvReflectNumericTraits,
) -> ReflectNumericTraits {
    ReflectNumericTraits {
        scalar: ReflectNumericTraitsScalar {
            width: ffi_type.scalar.width,
            signedness: ffi_type.scalar.signedness,
        },
        vector: ReflectNumericTraitsVector {
            component_count: ffi_type.vector.component_count,
        },
        matrix: ReflectNumericTraitsMatrix {
            column_count: ffi_type.matrix.column_count,
            row_count: ffi_type.matrix.row_count,
            stride: ffi_type.matrix.stride,
        }
    }
}

pub(crate) fn ffi_to_array_traits(
    ffi_type: ffi::SpvReflectArrayTraits,
) -> ReflectArrayTraits {
    ReflectArrayTraits {
        dims_count: ffi_type.dims_count,
        dims: ffi_type.dims,
        stride: ffi_type.stride,
    }
}

pub(crate) fn ffi_to_binding_array_traits(
    ffi_type: ffi::SpvReflectBindingArrayTraits,
) -> ReflectBindingArrayTraits {
    ReflectBindingArrayTraits {
        dims_count: ffi_type.dims_count,
        dims: ffi_type.dims,
    }
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

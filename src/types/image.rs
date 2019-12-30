#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
pub enum ReflectImageFormat {
    Undefined,
    RGBA32_FLOAT,
    RGBA16_FLOAT,
    R32_FLOAT,
    RGBA8,
    RGBA8_SNORM,
    RG32_FLOAT,
    RG16_FLOAT,
    R11G11B10_FLOAT,
    R16_FLOAT,
    RGBA16,
    RGB10A2,
    RG16,
    RG8,
    R16,
    R8,
    RGBA16_SNORM,
    RG16_SNORM,
    RG8_SNORM,
    R16_SNORM,
    R8_SNORM,
    RGBA32_INT,
    RGBA16_INT,
    RGBA8_INT,
    R32_INT,
    RG32_INT,
    RG16_INT,
    RG8_INT,
    R16_INT,
    R8_INT,
    RGBA32_UINT,
    RGBA16_UINT,
    RGBA8_UINT,
    R32_UINT,
    RGB10A2_UINT,
    RG32_UINT,
    RG16_UINT,
    RG8_UINT,
    R16_UINT,
    R8_UINT,
}

impl Default for ReflectImageFormat {
    fn default() -> Self {
        ReflectImageFormat::Undefined
    }
}

impl From<Option<spirv_headers::ImageFormat>> for ReflectImageFormat {
    fn from(raw: Option<spirv_headers::ImageFormat>) -> ReflectImageFormat {
        match raw {
            Some(spirv_headers::ImageFormat::Rgba32f) => ReflectImageFormat::RGBA32_FLOAT,
            Some(spirv_headers::ImageFormat::Rgba16f) => ReflectImageFormat::RGBA16_FLOAT,
            Some(spirv_headers::ImageFormat::R32f) => ReflectImageFormat::R32_FLOAT,
            Some(spirv_headers::ImageFormat::Rgba8) => ReflectImageFormat::RGBA8,
            Some(spirv_headers::ImageFormat::Rgba8Snorm) => ReflectImageFormat::RGBA8_SNORM,
            Some(spirv_headers::ImageFormat::Rg32f) => ReflectImageFormat::RG32_FLOAT,
            Some(spirv_headers::ImageFormat::Rg16f) => ReflectImageFormat::RG16_FLOAT,
            Some(spirv_headers::ImageFormat::R11fG11fB10f) => ReflectImageFormat::R11G11B10_FLOAT,
            Some(spirv_headers::ImageFormat::R16f) => ReflectImageFormat::R16_FLOAT,
            Some(spirv_headers::ImageFormat::Rgba16) => ReflectImageFormat::RGBA16,
            Some(spirv_headers::ImageFormat::Rgb10A2) => ReflectImageFormat::RGB10A2,
            Some(spirv_headers::ImageFormat::Rg16) => ReflectImageFormat::RG16,
            Some(spirv_headers::ImageFormat::Rg8) => ReflectImageFormat::RG8,
            Some(spirv_headers::ImageFormat::R16) => ReflectImageFormat::R16,
            Some(spirv_headers::ImageFormat::R8) => ReflectImageFormat::R8,
            Some(spirv_headers::ImageFormat::Rgba16Snorm) => ReflectImageFormat::RGBA16_SNORM,
            Some(spirv_headers::ImageFormat::Rg16Snorm) => ReflectImageFormat::RG16_SNORM,
            Some(spirv_headers::ImageFormat::Rg8Snorm) => ReflectImageFormat::RG8_SNORM,
            Some(spirv_headers::ImageFormat::R16Snorm) => ReflectImageFormat::R16_SNORM,
            Some(spirv_headers::ImageFormat::R8Snorm) => ReflectImageFormat::R8_SNORM,
            Some(spirv_headers::ImageFormat::Rgba32i) => ReflectImageFormat::RGBA32_INT,
            Some(spirv_headers::ImageFormat::Rgba16i) => ReflectImageFormat::RGBA16_INT,
            Some(spirv_headers::ImageFormat::Rgba8i) => ReflectImageFormat::RGBA8_INT,
            Some(spirv_headers::ImageFormat::R32i) => ReflectImageFormat::R32_INT,
            Some(spirv_headers::ImageFormat::Rg32i) => ReflectImageFormat::RG32_INT,
            Some(spirv_headers::ImageFormat::Rg16i) => ReflectImageFormat::RG16_INT,
            Some(spirv_headers::ImageFormat::Rg8i) => ReflectImageFormat::RG8_INT,
            Some(spirv_headers::ImageFormat::R16i) => ReflectImageFormat::R16_INT,
            Some(spirv_headers::ImageFormat::R8i) => ReflectImageFormat::R8_INT,
            Some(spirv_headers::ImageFormat::Rgba32ui) => ReflectImageFormat::RGBA32_UINT,
            Some(spirv_headers::ImageFormat::Rgba16ui) => ReflectImageFormat::RGBA16_UINT,
            Some(spirv_headers::ImageFormat::Rgba8ui) => ReflectImageFormat::RGBA8_UINT,
            Some(spirv_headers::ImageFormat::R32ui) => ReflectImageFormat::R32_UINT,
            Some(spirv_headers::ImageFormat::Rgb10a2ui) => ReflectImageFormat::RGB10A2_UINT,
            Some(spirv_headers::ImageFormat::Rg32ui) => ReflectImageFormat::RG32_UINT,
            Some(spirv_headers::ImageFormat::Rg16ui) => ReflectImageFormat::RG16_UINT,
            Some(spirv_headers::ImageFormat::Rg8ui) => ReflectImageFormat::RG8_UINT,
            Some(spirv_headers::ImageFormat::R16ui) => ReflectImageFormat::R16_UINT,
            Some(spirv_headers::ImageFormat::R8ui) => ReflectImageFormat::R8_UINT,
            _ => ReflectImageFormat::Undefined,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
pub enum ReflectFormat {
    Undefined,
    R32_UINT,
    R32_SINT,
    R32_SFLOAT,
    R32G32_UINT,
    R32G32_SINT,
    R32G32_SFLOAT,
    R32G32B32_UINT,
    R32G32B32_SINT,
    R32G32B32_SFLOAT,
    R32G32B32A32_UINT,
    R32G32B32A32_SINT,
    R32G32B32A32_SFLOAT,
}

impl Default for ReflectFormat {
    fn default() -> Self {
        ReflectFormat::Undefined
    }
}

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

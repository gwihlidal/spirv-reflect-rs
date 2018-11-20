use types::image::ReflectImageFormat;
use types::ReflectDimension;

#[derive(Debug, Default, Clone, Serialize)]
pub struct ReflectBindingArrayTraits {
    pub dims: Vec<u32>,
}

#[derive(Default, Debug, Copy, Clone, Serialize)]
pub struct ReflectNumericTraitsScalar {
    pub width: u32,
    pub signedness: u32,
}

#[derive(Default, Debug, Copy, Clone, Serialize)]
pub struct ReflectNumericTraitsVector {
    pub component_count: u32,
}

#[derive(Default, Debug, Copy, Clone, Serialize)]
pub struct ReflectNumericTraitsMatrix {
    pub column_count: u32,
    pub row_count: u32,
    pub stride: u32,
}

#[derive(Default, Debug, Copy, Clone, Serialize)]
pub struct ReflectNumericTraits {
    pub scalar: ReflectNumericTraitsScalar,
    pub vector: ReflectNumericTraitsVector,
    pub matrix: ReflectNumericTraitsMatrix,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ReflectArrayTraits {
    pub dims: Vec<u32>,
    pub stride: u32,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ReflectTypeDescriptionTraits {
    pub numeric: ReflectNumericTraits,
    pub image: ReflectImageTraits,
    pub array: ReflectArrayTraits,
}

#[derive(Default, Debug, Copy, Clone, Serialize)]
pub struct ReflectImageTraits {
    pub dim: ReflectDimension,
    pub depth: u32,
    pub arrayed: u32,
    pub ms: u32,
    pub sampled: u32,
    pub image_format: ReflectImageFormat,
}

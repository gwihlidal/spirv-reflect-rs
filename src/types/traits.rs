use types::image::ReflectImageFormat;
use types::ReflectDimension;

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

#[derive(Default, Debug, Copy, Clone)]
pub struct ReflectTypeDescriptionTraits {
    pub numeric: ReflectNumericTraits,
    pub image: ReflectImageTraits,
    pub array: ReflectArrayTraits,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct ReflectImageTraits {
    pub dim: ReflectDimension,
    pub depth: u32,
    pub arrayed: u32,
    pub ms: u32,
    pub sampled: u32,
    pub image_format: ReflectImageFormat,
}

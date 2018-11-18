use types::ReflectDimension;

#[derive(Debug, Copy, Clone)]
pub enum ReflectFormat {
    Undefined,
}

impl Default for ReflectFormat {
    fn default() -> Self {
        ReflectFormat::Undefined
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct ReflectImageTraits {
    pub dim: ReflectDimension,
    pub depth: u32,
    pub arrayed: u32,
    pub ms: u32,
    pub sampled: u32,
    pub image_format: ReflectFormat,
}
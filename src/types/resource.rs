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

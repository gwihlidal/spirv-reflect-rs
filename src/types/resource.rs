bitflags! {
    #[derive(Serialize)]
    pub struct ReflectResourceTypeFlags: u32 {
        const UNDEFINED = 0;
        const SAMPLER = 1;
        const COMBINED_IMAGE_SAMPLER = 2;
        const CONSTANT_BUFFER_VIEW = 4;
        const SHADER_RESOURCE_VIEW = 8;
        const UNORDERED_ACCESS_VIEW = 256;
    }
}

impl Default for ReflectResourceTypeFlags {
    fn default() -> Self {
        ReflectResourceTypeFlags::UNDEFINED
    }
}

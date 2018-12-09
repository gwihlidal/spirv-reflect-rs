extern crate spirv_reflect;

#[cfg(test)]
mod tests {
    use spirv_reflect::*;

    #[test]
    fn load_module() {
        let ps_data = include_bytes!("./ImGuiPs.spirv");
        ShaderModule::load_u8_data(ps_data).unwrap();
    }

    #[test]
    fn enumerate_sets() {
        let ps_data = include_bytes!("./ImGuiPs.spirv");
        let module = ShaderModule::load_u8_data(ps_data).unwrap();
        let descriptor_sets = module.enumerate_descriptor_sets(None).unwrap();
        
        assert_eq!(descriptor_sets.len(), 1);
        let descriptor_set = &descriptor_sets[0];
        assert_eq!(descriptor_set.bindings.len(), 2);
    }

    #[test]
    fn enumerate_bindings() {
        let ps_data = include_bytes!("./ImGuiPs.spirv");
        let module = ShaderModule::load_u8_data(ps_data).unwrap();
        let descriptor_sets = module.enumerate_descriptor_sets(None).unwrap();
        
        assert_eq!(descriptor_sets.len(), 1);
        let descriptor_set = &descriptor_sets[0];
        
        assert_eq!(descriptor_set.bindings.len(), 2);
        let tex_descriptor = &descriptor_set.bindings[0];
        assert_eq!(tex_descriptor.name, "tex");
        assert_eq!(tex_descriptor.descriptor_type, types::ReflectDescriptorType::SampledImage);
        
        let smp_descriptor = &descriptor_set.bindings[1];
        assert_eq!(smp_descriptor.name, "smp");
        assert_eq!(smp_descriptor.descriptor_type, types::ReflectDescriptorType::Sampler);
    }

    #[test]
    fn change_binding_numbers() {
        let ps_data = include_bytes!("./ImGuiPs.spirv");
        let mut module = ShaderModule::load_u8_data(ps_data).unwrap();
        
        let descriptor_sets = module.enumerate_descriptor_sets(None).unwrap();

        assert_eq!(descriptor_sets.len(), 1);
        let descriptor_set = &descriptor_sets[0];
        assert_eq!(descriptor_set.bindings.len(), 2);

        let tex_descriptor = &descriptor_set.bindings[0];
        module.change_descriptor_binding_numbers(&tex_descriptor, 30, Some(1)).unwrap();

        let smp_descriptor = &descriptor_set.bindings[1];
        module.change_descriptor_binding_numbers(&smp_descriptor, 4, Some(2)).unwrap();
    }
}
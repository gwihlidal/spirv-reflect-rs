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

        let tex_descriptor_index = descriptor_set.bindings[0];
        let tex_descriptor = &module
            .get_descriptor_binding(tex_descriptor_index)
            .expect("invalid tex descriptor binding");
        assert_eq!(tex_descriptor.name, "tex");
        assert_eq!(
            tex_descriptor.descriptor_type,
            types::ReflectDescriptorType::SampledImage
        );

        let smp_descriptor_index = descriptor_set.bindings[1];
        let smp_descriptor = &module
            .get_descriptor_binding(smp_descriptor_index)
            .expect("invalid smp descriptor binding");
        assert_eq!(smp_descriptor.name, "smp");
        assert_eq!(
            smp_descriptor.descriptor_type,
            types::ReflectDescriptorType::Sampler
        );
    }

    #[test]
    fn change_binding_numbers() {
        let ps_data = include_bytes!("./ImGuiPs.spirv");
        let mut module = ShaderModule::load_u8_data(ps_data).unwrap();

        let descriptor_sets = module.enumerate_descriptor_sets(None).unwrap();

        assert_eq!(descriptor_sets.len(), 1);
        let descriptor_set = &descriptor_sets[0];
        assert_eq!(descriptor_set.bindings.len(), 2);

        let tex_descriptor_index = descriptor_set.bindings[0];
        let tex_descriptor = &module
            .get_descriptor_binding(tex_descriptor_index)
            .expect("invalid tex descriptor binding");
        assert_ne!(tex_descriptor.binding, 30);
        assert_ne!(tex_descriptor.set, 1);

        module
            .change_descriptor_binding_numbers(tex_descriptor_index, Some(30), Some(1))
            .unwrap();

        let smp_descriptor_index = descriptor_set.bindings[1];
        let smp_descriptor = &module
            .get_descriptor_binding(smp_descriptor_index)
            .expect("invalid smp descriptor binding");
        assert_ne!(smp_descriptor.binding, 4);
        assert_ne!(smp_descriptor.set, 2);

        module
            .change_descriptor_binding_numbers(smp_descriptor_index, Some(4), Some(2))
            .unwrap();

        let descriptor_sets2 = module.enumerate_descriptor_sets(None).unwrap();

        assert_eq!(descriptor_sets2.len(), 2);

        let descriptor_set_1 = &descriptor_sets2[0];
        assert_eq!(descriptor_set_1.bindings.len(), 1);

        let descriptor_set_2 = &descriptor_sets2[1];
        assert_eq!(descriptor_set_2.bindings.len(), 1);

        let tex_descriptor2_index = descriptor_set_1.bindings[0];
        let tex_descriptor2 = &module
            .get_descriptor_binding(tex_descriptor2_index)
            .expect("invalid tex descriptor binding");
        assert_eq!(tex_descriptor2.binding, 30);
        assert_eq!(tex_descriptor2.set, 1);

        let smp_descriptor2_index = descriptor_set_2.bindings[0];
        let smp_descriptor2 = &module
            .get_descriptor_binding(smp_descriptor2_index)
            .expect("invalid smp descriptor binding");
        assert_eq!(smp_descriptor2.binding, 4);
        assert_eq!(smp_descriptor2.set, 2);
    }
}

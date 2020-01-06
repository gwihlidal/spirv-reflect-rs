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
        assert_eq!(descriptor_set.value.binding_refs.len(), 2);
    }

    #[test]
    fn enumerate_bindings() {
        let ps_data = include_bytes!("./ImGuiPs.spirv");
        let module = ShaderModule::load_u8_data(ps_data).unwrap();

        let descriptor_sets = module.enumerate_descriptor_sets(None).unwrap();

        assert_eq!(descriptor_sets.len(), 1);
        let descriptor_set = &descriptor_sets[0];
        assert_eq!(descriptor_set.value.binding_refs.len(), 2);

        let tex_descriptor_ref = &descriptor_set.value.binding_refs[0];
        let tex_descriptor_val = &tex_descriptor_ref.value;
        assert_eq!(tex_descriptor_val.name, "tex");
        assert_eq!(
            tex_descriptor_val.descriptor_type,
            types::ReflectDescriptorType::SampledImage
        );

        let smp_descriptor_ref = &descriptor_set.value.binding_refs[1];
        let smp_descriptor_val = &smp_descriptor_ref.value;
        assert_eq!(smp_descriptor_val.name, "smp");
        assert_eq!(
            smp_descriptor_val.descriptor_type,
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
        assert_eq!(descriptor_set.value.binding_refs.len(), 2);

        let tex_descriptor_ref = &descriptor_set.value.binding_refs[0];
        let tex_descriptor_val = &tex_descriptor_ref.value;
        assert_ne!(tex_descriptor_val.binding, 30);
        assert_ne!(tex_descriptor_val.set, 1);

        module
            .change_descriptor_binding_numbers(&tex_descriptor_ref, Some(30), Some(1))
            .unwrap();

        let smp_descriptor_ref = &descriptor_set.value.binding_refs[1];
        let smp_descriptor_val = &smp_descriptor_ref.value;
        assert_ne!(smp_descriptor_val.binding, 4);
        assert_ne!(smp_descriptor_val.set, 2);

        module
            .change_descriptor_binding_numbers(&smp_descriptor_ref, Some(4), Some(2))
            .unwrap();

        let descriptor_sets2 = module.enumerate_descriptor_sets(None).unwrap();

        assert_eq!(descriptor_sets2.len(), 2);

        let descriptor_set_1 = &descriptor_sets2[0];
        assert_eq!(descriptor_set_1.value.binding_refs.len(), 1);

        let descriptor_set_2 = &descriptor_sets2[1];
        assert_eq!(descriptor_set_2.value.binding_refs.len(), 1);

        let tex_descriptor2_ref = &descriptor_set_1.value.binding_refs[0];
        let tex_descriptor2_val = &tex_descriptor2_ref.value;
        assert_eq!(tex_descriptor2_val.binding, 30);
        assert_eq!(tex_descriptor2_val.set, 1);

        let smp_descriptor2_ref = &descriptor_set_2.value.binding_refs[0];
        let smp_descriptor2_val = &smp_descriptor2_ref.value;
        assert_eq!(smp_descriptor2_val.binding, 4);
        assert_eq!(smp_descriptor2_val.set, 2);
    }
}

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

    /*
    #[test]
    fn enumerate_bindings() {
        let ps_data = include_bytes!("./ImGuiPs.spirv");
        let _module = ShaderModule::load_u8_data(ps_data).unwrap();

        //
    }

    #[test]
    fn change_binding_numbers() {
        let ps_data = include_bytes!("./ImGuiPs.spirv");
        let _module = ShaderModule::load_u8_data(ps_data).unwrap();

        //
    }*/
}

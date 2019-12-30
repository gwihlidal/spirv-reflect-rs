extern crate spirv_reflect;

#[cfg(test)]
mod tests {
    use spirv_reflect::*;

    #[test]
    fn load_module() {
        let ps_data = include_bytes!("./ImGuiPs.spirv");
        ShaderModule::load_u8_data(ps_data).unwrap();
    }
    /*
    #[test]
    fn enumerate_sets() {
        let ps_data = include_bytes!("./ImGuiPs.spirv");
        let _module = ShaderModule::load_u8_data(ps_data).unwrap();

        //
    }

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

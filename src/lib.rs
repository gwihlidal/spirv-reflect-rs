#[macro_use]
extern crate bitflags;
extern crate num_traits;
extern crate spirv_headers;
#[macro_use]
extern crate serde_derive;

//use num_traits::cast::FromPrimitive;

pub mod types;

#[derive(Default, Clone)]
pub struct ShaderModule {}

impl ShaderModule {
    pub fn load_u8_data(spv_data: &[u8]) -> Result<ShaderModule, &str> {
        Ok(create_shader_module(spv_data)?)
    }

    pub fn load_u32_data(spv_data: &[u32]) -> Result<ShaderModule, &str> {
        let u8_data: &[u8] = unsafe {
            std::slice::from_raw_parts(
                spv_data.as_ptr() as *const u8,
                spv_data.len() * std::mem::size_of::<u32>(),
            )
        };
        Ok(create_shader_module(u8_data)?)
    }

    pub fn get_code(&self) -> Vec<u32> {
        Vec::new()
    }

    pub fn get_generator(&self) -> types::ReflectGenerator {
        types::ReflectGenerator::default()
    }

    pub fn get_shader_stage(&self) -> types::ReflectShaderStageFlags {
        types::ReflectShaderStageFlags::UNDEFINED
    }

    pub fn get_source_language(&self) -> spirv_headers::SourceLanguage {
        spirv_headers::SourceLanguage::Unknown
    }

    pub fn get_source_language_version(&self) -> u32 {
        0
    }

    pub fn get_source_file(&self) -> String {
        String::new()
    }

    pub fn get_source_text(&self) -> String {
        String::new()
    }

    pub fn get_spirv_execution_model(&self) -> spirv_headers::ExecutionModel {
        spirv_headers::ExecutionModel::Vertex
    }

    pub fn enumerate_input_variables(
        &self,
        entry_point: Option<&str>,
    ) -> Result<Vec<types::ReflectInterfaceVariable>, &str> {
        Ok(Vec::new())
    }

    pub fn enumerate_output_variables(
        &self,
        entry_point: Option<&str>,
    ) -> Result<Vec<types::ReflectInterfaceVariable>, &str> {
        Ok(Vec::new())
    }

    pub fn enumerate_descriptor_bindings(
        &self,
        entry_point: Option<&str>,
    ) -> Result<Vec<types::ReflectDescriptorBinding>, &str> {
        Ok(Vec::new())
    }

    pub fn enumerate_descriptor_sets(
        &self,
        entry_point: Option<&str>,
    ) -> Result<Vec<types::ReflectDescriptorSet>, &str> {
        Ok(Vec::new())
    }

    pub fn enumerate_push_constant_blocks(
        &self,
        entry_point: Option<&str>,
    ) -> Result<Vec<types::ReflectBlockVariable>, &str> {
        Ok(Vec::new())
    }

    pub fn enumerate_entry_points(&self) -> Result<Vec<types::ReflectEntryPoint>, &str> {
        Ok(Vec::new())
    }

    pub fn get_entry_point_name(&self) -> String {
        String::new()
    }

    pub fn change_descriptor_binding_numbers(
        &mut self,
        binding: &types::descriptor::ReflectDescriptorBinding,
        new_binding: u32,
        new_set: Option<u32>,
    ) -> Result<(), &str> {
        Ok(())
    }

    pub fn change_descriptor_set_number(
        &mut self,
        set: &types::descriptor::ReflectDescriptorSet,
        new_set: u32,
    ) -> Result<(), &str> {
        Ok(())
    }

    pub fn change_input_variable_location(
        &mut self,
        variable: &types::variable::ReflectInterfaceVariable,
        new_location: u32,
    ) -> Result<(), &str> {
        Ok(())
    }

    pub fn change_output_variable_location(
        &mut self,
        variable: &types::variable::ReflectInterfaceVariable,
        new_location: u32,
    ) -> Result<(), &str> {
        Ok(())
    }
}

pub fn create_shader_module(spv_data: &[u8]) -> Result<ShaderModule, &str> {
    Ok(ShaderModule::default())
}

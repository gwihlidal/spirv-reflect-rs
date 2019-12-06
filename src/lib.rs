#[macro_use]
extern crate bitflags;
extern crate num_traits;
extern crate spirv_headers;
#[macro_use]
extern crate serde_derive;

//use num_traits::cast::FromPrimitive;

pub(crate) mod parser;
pub mod types;

#[derive(Default, Clone)]
pub struct ShaderModule {
    internal: types::ReflectShaderModule,
}

impl ShaderModule {
    pub fn load_u8_data(spv_data: &[u8]) -> Result<ShaderModule, String> {
        assert_eq!(spv_data.len() % std::mem::size_of::<u32>(), 0); // TODO: Nice error
        let u32_data: &[u32] = unsafe {
            std::slice::from_raw_parts(
                spv_data.as_ptr() as *const u32,
                spv_data.len() / std::mem::size_of::<u32>(),
            )
        };
        Ok(create_shader_module(u32_data)?)
    }

    pub fn load_u32_data(spv_words: &[u32]) -> Result<ShaderModule, String> {
        Ok(create_shader_module(spv_words)?)
    }

    pub fn get_code(&self) -> &[u32] {
        &self.internal.spirv_code
    }

    pub fn get_generator(&self) -> types::ReflectGenerator {
        self.internal.generator
    }

    pub fn get_shader_stage(&self) -> types::ReflectShaderStageFlags {
        self.internal.shader_stage
    }

    pub fn get_source_language(&self) -> Option<spirv_headers::SourceLanguage> {
        self.internal.source_language
    }

    pub fn get_source_language_version(&self) -> u32 {
        self.internal.source_language_version
    }

    pub fn get_source_file(&self) -> &str {
        &self.internal.source_file
    }

    pub fn get_source_text(&self) -> &str {
        &self.internal.source_text
    }

    pub fn get_spirv_execution_model(&self) -> spirv_headers::ExecutionModel {
        self.internal.spirv_execution_model
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

    pub fn get_entry_point_name(&self) -> &str {
        &self.internal.entry_point_name
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

pub fn create_shader_module(spv_words: &[u32]) -> Result<ShaderModule, String> {
    let mut module = ShaderModule::default();
    let mut parser = parser::Parser::default();
    parser.parse(spv_words, &mut module)?;
    Ok(module)
}

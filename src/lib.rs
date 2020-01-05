#[macro_use]
extern crate bitflags;
extern crate num_traits;
extern crate spirv_headers;
#[macro_use]
extern crate serde_derive;

pub(crate) mod parser;
pub mod types;

#[derive(Default, Clone)]
pub struct ShaderModule {
    internal: types::ReflectShaderModule,
}

impl ShaderModule {
    pub fn load_u8_data(spv_data: &[u8]) -> Result<ShaderModule, String> {
        if spv_data.len() % std::mem::size_of::<u32>() != 0 {
            return Err(
                "Invalid SPIR-V data - length must be evenly divisible by WORD size (4)".into(),
            );
        }
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

    pub fn get_shader_stage(&self) -> types::ReflectShaderStage {
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

    pub fn get_spirv_execution_model(&self) -> Option<spirv_headers::ExecutionModel> {
        self.internal.spirv_execution_model
    }

    pub fn enumerate_input_variables(
        &self,
        entry_point_name: Option<&str>,
    ) -> Result<Vec<types::ReflectInterfaceVariable>, &str> {
        match entry_point_name {
            Some(entry_point_name) => {
                if let Some(ref entry_point) = self
                    .internal
                    .entry_points
                    .iter()
                    .find(|entry_point| entry_point.name == entry_point_name)
                {
                    Ok(entry_point.input_variables.to_owned())
                } else {
                    return Err("Error enumerating input variables - entry point not found".into());
                }
            }
            None => Ok(self.internal.input_variables.to_owned()),
        }
    }

    pub fn enumerate_output_variables(
        &self,
        entry_point_name: Option<&str>,
    ) -> Result<Vec<types::ReflectInterfaceVariable>, &str> {
        match entry_point_name {
            Some(entry_point_name) => {
                if let Some(ref entry_point) = self
                    .internal
                    .entry_points
                    .iter()
                    .find(|entry_point| entry_point.name == entry_point_name)
                {
                    Ok(entry_point.output_variables.to_owned())
                } else {
                    return Err("Error enumerating output variables - entry point not found".into());
                }
            }
            None => Ok(self.internal.output_variables.to_owned()),
        }
    }

    pub fn enumerate_descriptor_bindings(
        &self,
        entry_point_name: Option<&str>,
    ) -> Result<Vec<types::ReflectDescriptorBinding>, &str> {
        match entry_point_name {
            Some(entry_point_name) => {
                if let Some(ref entry_point) = self
                    .internal
                    .entry_points
                    .iter()
                    .find(|entry_point| entry_point.name == entry_point_name)
                {
                    let mut descriptor_bindings = Vec::new();
                    for descriptor_binding in &self.internal.descriptor_bindings {
                        if entry_point
                            .used_uniforms
                            .iter()
                            .position(|x| x == &descriptor_binding.spirv_id)
                            .is_some()
                        {
                            descriptor_bindings.push(descriptor_binding.to_owned());
                        }
                    }
                    Ok(descriptor_bindings)
                } else {
                    return Err(
                        "Error enumerating descriptor bindings - entry point not found".into(),
                    );
                }
            }
            None => Ok(self.internal.descriptor_bindings.to_owned()),
        }
    }

    pub fn enumerate_descriptor_sets(
        &self,
        entry_point_name: Option<&str>,
    ) -> Result<Vec<types::ReflectDescriptorSet>, &str> {
        let mut descriptor_sets = match entry_point_name {
            Some(entry_point_name) => {
                if let Some(ref entry_point) = self
                    .internal
                    .entry_points
                    .iter()
                    .find(|entry_point| entry_point.name == entry_point_name)
                {
                    entry_point.descriptor_sets.to_owned()
                } else {
                    return Err("Error enumerating descriptor sets - entry point not found".into());
                }
            }
            None => self.internal.descriptor_sets.to_owned(),
        };

        descriptor_sets.retain(|x| x.set != std::u32::MAX);
        Ok(descriptor_sets)
    }

    pub fn enumerate_push_constant_blocks(
        &self,
        entry_point_name: Option<&str>,
    ) -> Result<Vec<types::ReflectBlockVariable>, &str> {
        match entry_point_name {
            Some(entry_point_name) => {
                if let Some(ref entry_point) = self
                    .internal
                    .entry_points
                    .iter()
                    .find(|entry_point| entry_point.name == entry_point_name)
                {
                    let mut push_constant_blocks = Vec::new();
                    for push_constant_block in &self.internal.push_constant_blocks {
                        if entry_point
                            .used_push_constants
                            .iter()
                            .position(|x| x == &push_constant_block.spirv_id)
                            .is_some()
                        {
                            push_constant_blocks.push(push_constant_block.to_owned());
                        }
                    }
                    Ok(push_constant_blocks)
                } else {
                    return Err(
                        "Error enumerating push constant blocks - entry point not found".into(),
                    );
                }
            }
            None => Ok(self.internal.push_constant_blocks.to_owned()),
        }
    }

    pub fn enumerate_entry_points(&self) -> Result<Vec<types::ReflectEntryPoint>, &str> {
        Ok(self.internal.entry_points.to_owned())
    }

    pub fn get_entry_point_name(&self) -> &str {
        &self.internal.entry_point_name
    }

    pub fn get_descriptor_binding(
        &self,
        binding_index: usize,
    ) -> Option<&crate::types::ReflectDescriptorBinding> {
        if binding_index < self.internal.descriptor_bindings.len() {
            Some(&self.internal.descriptor_bindings[binding_index])
        } else {
            None
        }
    }

    pub fn change_descriptor_binding_numbers(
        &mut self,
        binding_index: usize,
        new_binding: Option<u32>,
        new_set: Option<u32>,
    ) -> Result<(), String> {
        if binding_index < self.internal.descriptor_bindings.len() {
            let mut descriptor_binding = &mut self.internal.descriptor_bindings[binding_index];
            let (word_offset_binding, word_offset_set) = descriptor_binding.word_offset;

            if word_offset_binding as usize > self.internal.spirv_code.len() - 1 {
                return Err(
                    "Error attempting to change descriptor binding numbers - binding word offset range exceeded"
                        .into(),
                );
            }

            if word_offset_set as usize > self.internal.spirv_code.len() - 1 {
                return Err(
                    "Error attempting to change descriptor binding numbers - set word offset range exceeded"
                        .into(),
                );
            }

            if let Some(new_binding) = new_binding {
                descriptor_binding.binding = new_binding;
                self.internal.spirv_code[word_offset_binding as usize] = descriptor_binding.binding;
            }

            if let Some(new_set) = new_set {
                descriptor_binding.set = new_set;
                self.internal.spirv_code[word_offset_set as usize] = descriptor_binding.set;
                self.internal.build_descriptor_sets()?;
            }
            Ok(())
        } else {
            Err(
                "Error attempting to change descriptor binding numbers - index is out of range"
                    .into(),
            )
        }
    }

    pub fn change_descriptor_set_number(
        &mut self,
        _set: &types::descriptor::ReflectDescriptorSet,
        _new_set: u32,
    ) -> Result<(), String> {
        println!("UNIMPLEMENTED - change_descriptor_set_number");
        self.internal.build_descriptor_sets()?;
        Ok(())
    }

    pub fn change_input_variable_location(
        &mut self,
        _variable: &types::variable::ReflectInterfaceVariable,
        _new_location: u32,
    ) -> Result<(), String> {
        println!("UNIMPLEMENTED - change_input_variable_location");
        Ok(())
    }

    pub fn change_output_variable_location(
        &mut self,
        _variable: &types::variable::ReflectInterfaceVariable,
        _new_location: u32,
    ) -> Result<(), String> {
        println!("UNIMPLEMENTED - change_output_variable_location");
        Ok(())
    }
}

pub fn create_shader_module(spv_words: &[u32]) -> Result<ShaderModule, String> {
    let mut module = ShaderModule::default();
    let mut parser = parser::Parser::default();
    parser.parse(spv_words, &mut module)?;
    module.internal.spirv_code = spv_words.to_vec();
    Ok(module)
}

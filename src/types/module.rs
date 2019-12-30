use crate::types::*;

#[derive(Debug, Clone)]
pub struct ReflectShaderModule {
    pub generator: ReflectGenerator,
    pub entry_point_name: String,
    pub entry_point_id: u32,
    pub entry_points: Vec<ReflectEntryPoint>,
    pub source_language: Option<spirv_headers::SourceLanguage>,
    pub source_language_version: u32,
    pub source_file_id: u32,
    pub source_file: String,
    pub source_text: String,
    pub spirv_execution_model: spirv_headers::ExecutionModel,
    pub shader_stage: ReflectShaderStage,
    pub descriptor_bindings: Vec<ReflectDescriptorBinding>,
    pub descriptor_sets: Vec<ReflectDescriptorSet>,
    pub input_variables: Vec<ReflectInterfaceVariable>,
    pub output_variables: Vec<ReflectInterfaceVariable>,
    pub push_constant_blocks: Vec<ReflectBlockVariable>,
    pub spirv_code: Vec<u32>,
    pub type_descriptions: Vec<ReflectTypeDescription>,
}

impl Default for ReflectShaderModule {
    fn default() -> ReflectShaderModule {
        ReflectShaderModule {
            generator: ReflectGenerator::Unknown,
            entry_point_name: String::new(),
            entry_point_id: 0,
            entry_points: Vec::new(),
            source_language: None,
            source_language_version: 0,
            source_file_id: 0,
            source_file: String::new(),
            source_text: String::new(),
            spirv_execution_model: spirv_headers::ExecutionModel::Vertex,
            shader_stage: ReflectShaderStage::Undefined,
            descriptor_bindings: Vec::new(),
            descriptor_sets: Vec::new(),
            input_variables: Vec::new(),
            output_variables: Vec::new(),
            push_constant_blocks: Vec::new(),
            spirv_code: Vec::new(),
            type_descriptions: Vec::new(),
        }
    }
}

impl ReflectShaderModule {
    pub(crate) fn find_type(&self, type_id: u32) -> Option<usize> {
        for type_index in 0..self.type_descriptions.len() {
            let type_description = &self.type_descriptions[type_index];
            if type_description.id == type_id {
                return Some(type_index);
            }
        }

        None
    }
}

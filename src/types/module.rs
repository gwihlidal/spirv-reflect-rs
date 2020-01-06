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
    pub spirv_execution_model: Option<spirv_headers::ExecutionModel>,
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
            spirv_execution_model: None,
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

    pub(crate) fn build_descriptor_sets(&mut self) -> Result<(), String> {
        const MAX_DESCRIPTOR_SETS: usize = 64;

        // Clear out old descriptor set data
        self.descriptor_sets.clear();
        self.descriptor_sets.resize(
            MAX_DESCRIPTOR_SETS,
            crate::types::ReflectDescriptorSet {
                set: std::u32::MAX,
                binding_refs: Vec::new(),
            },
        );

        // TODO: Compiler ICE
        //let mut descriptor_count = [MAX_DESCRIPTOR_SETS; 0usize];

        for descriptor_binding in &self.descriptor_bindings {
            let _set_index = if let Some(set_index) = self
                .descriptor_sets
                .iter()
                .position(|set| set.set == descriptor_binding.set)
            {
                // Found existing set
                set_index
            } else {
                // Find an available set
                if let Some(set_index) = self
                    .descriptor_sets
                    .iter()
                    .position(|set| set.set == std::u32::MAX)
                {
                    self.descriptor_sets[set_index].set = descriptor_binding.set;
                    set_index
                } else {
                    // Ran out of sets!
                    return Err("Error building descriptor sets - no more slots available".into());
                }
            };

            // TODO: Compiler ICE
            //descriptor_count[set_index] += 1;
        }

        // TODO: Compiler ICE
        //for set_index in 0..self.descriptor_sets.len() {
        //    let count = descriptor_count[set_index];
        //    self.descriptor_sets[set_index]
        //        .bindings
        //        .reserve(count);
        //}

        let mut set_count = 0;
        for descriptor_set in &self.descriptor_sets {
            if descriptor_set.set != std::u32::MAX {
                set_count += 1;
            }
        }

        self.descriptor_sets.sort_by(|a, b| {
            let a_set = a.set;
            let b_set = b.set;
            a_set.cmp(&b_set)
        });

        for set_index in 0..set_count {
            let set = self.descriptor_sets[set_index].set;
            for binding_index in 0..self.descriptor_bindings.len() {
                let descriptor_binding = &self.descriptor_bindings[binding_index];
                if descriptor_binding.set == set {
                    self.descriptor_sets[set_index].binding_refs.push(
                        crate::DescriptorBindingRef {
                            ref_id: Some(binding_index),
                            entry_point_id: None,
                            value: descriptor_binding.to_owned(),
                        },
                    );
                }
            }
        }

        // Update entry points
        for entry_point_index in 0..self.entry_points.len() {
            let entry_point = &mut self.entry_points[entry_point_index];
            let mut descriptor_set_count = 0;
            for descriptor_set in &self.descriptor_sets {
                for binding_ref in &descriptor_set.binding_refs {
                    let binding_id =
                        &self.descriptor_bindings[binding_ref.ref_id.unwrap()].spirv_id;
                    if let Some(_) = entry_point
                        .used_uniforms
                        .iter()
                        .position(|id| id == binding_id)
                    {
                        descriptor_set_count += 1;
                        break;
                    }
                }
            }

            entry_point.descriptor_sets.clear();
            entry_point.descriptor_sets.reserve(descriptor_set_count);

            for descriptor_set in &self.descriptor_sets {
                let mut binding_count = 0;

                for binding_ref in &descriptor_set.binding_refs {
                    let binding_id =
                        &self.descriptor_bindings[binding_ref.ref_id.unwrap()].spirv_id;
                    if let Some(_) = entry_point
                        .used_uniforms
                        .iter()
                        .position(|id| id == binding_id)
                    {
                        binding_count += 1;
                    }
                }

                if binding_count == 0 {
                    continue;
                }

                let mut binding_refs = Vec::with_capacity(binding_count);
                for binding_ref in &descriptor_set.binding_refs {
                    let descriptor_binding_id = binding_ref.ref_id.unwrap();
                    let descriptor_binding = &self.descriptor_bindings[descriptor_binding_id];
                    if let Some(_) = entry_point
                        .used_uniforms
                        .iter()
                        .position(|id| id == &descriptor_binding.spirv_id)
                    {
                        binding_refs.push(crate::DescriptorBindingRef {
                            ref_id: Some(descriptor_binding_id),
                            entry_point_id: Some(entry_point_index),
                            value: descriptor_binding.to_owned(),
                        });
                    }
                }

                entry_point
                    .descriptor_sets
                    .push(crate::types::ReflectDescriptorSet {
                        set: descriptor_set.set,
                        binding_refs,
                    });
            }
        }

        Ok(())
    }
}

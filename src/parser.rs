use crate::num_traits::FromPrimitive;
use crate::types;
use std::ffi::CStr;
use std::os::raw::c_char;

pub const STARTING_WORD: usize = 5;

#[derive(Default, Debug, Clone, Serialize, PartialEq)]
pub(crate) struct NumberDecoration {
    pub word_offset: u32,
    pub value: u32,
}

#[derive(Default, Debug, Clone, Serialize, PartialEq)]
pub(crate) struct StringDecoration {
    pub word_offset: u32,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct Decorations {
    pub is_block: bool,
    pub is_buffer_block: bool,
    pub is_row_major: bool,
    pub is_column_major: bool,
    pub is_built_in: bool,
    pub is_noperspective: bool,
    pub is_flat: bool,
    pub is_non_writable: bool,
    pub set: NumberDecoration,
    pub binding: NumberDecoration,
    pub input_attachment_index: NumberDecoration,
    pub location: NumberDecoration,
    pub offset: NumberDecoration,
    pub uav_counter_buffer: NumberDecoration,
    pub semantic: StringDecoration,
    pub array_stride: u32,
    pub matrix_stride: u32,
    pub built_in: Option<spirv_headers::BuiltIn>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct ParserArrayTraits {
    pub element_type_id: u32,
    pub length_id: u32,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct ParserImageTraits {
    pub sampled_type_id: u32,
    pub dim: Option<spirv_headers::Dim>,
    pub depth: u32,
    pub arrayed: u32,
    pub ms: u32,
    pub sampled: u32,
    pub image_format: Option<spirv_headers::ImageFormat>,
}

/*
impl Default for ParserImageTraits {
    fn default() -> Self {
        ParserImageTraits {
            sampled_type_id: 0,
            dim: spirv_headers::Dim::Dim1D,
            depth: 0,
            arrayed: 0,
            ms: 0,
            sampled: 0,
            image_format: spirv_headers::ImageFormat::Unknown,
        }
    }
}
*/

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct ParserNode {
    pub result_id: u32,
    pub op: Option<spirv_headers::Op>,
    pub result_type_id: u32,
    pub type_id: u32,
    pub storage_class: Option<spirv_headers::StorageClass>,
    pub word_offset: u32,
    pub word_count: u32,
    pub is_type: bool,
    pub array_traits: ParserArrayTraits,
    pub image_traits: ParserImageTraits,
    pub image_type_id: u32,
    pub name: String,
    pub decorations: Decorations,
    pub member_count: u32,
    pub member_names: Vec<String>,
    pub member_decorations: Vec<Decorations>,
}

/*
impl Default for ParserNode {
    fn default() -> ParserNode {
        Self {
            result_id: 0,
            op: None,
            result_type_id: 0,
            type_id: 0,
            storage_class: None,
            word_offset: 0,
            word_count: 0,
            is_type: false,
            array_traits: types::ReflectArrayTraits::default(),
            image_traits: types::ReflectImageTraits::default(),
            image_type_id: 0,
            name: String::new(),
            decorations:
            member_count: 0,
            member_names: Vec::new(),
            member_decorations: Vec::new(),
        }
    }
}*/

#[derive(Default)]
pub(crate) struct ParserString {
    pub result_id: u32,
    pub string: String,
}

#[derive(Default)]
pub(crate) struct Parser {
    pub nodes: Vec<ParserNode>,
    pub strings: Vec<ParserString>,

    pub string_count: usize,
    pub type_count: usize,
    pub descriptor_count: usize,
    pub push_constant_count: usize,
    pub entry_point_count: usize,
    pub function_count: usize,
}

impl Parser {
    pub(crate) fn parse(
        &mut self,
        spv_words: &[u32],
        module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        if spv_words.len() == 0 {
            return Err("No SPIR-V specified for shader module".to_string());
        }

        if spv_words[0] != spirv_headers::MAGIC_NUMBER {
            return Err("Invalid SPIR-V - does not start with valid magic number.".to_string());
        }

        // Determine generator
        let generator: u32 = (spv_words[2] & 0xFFFF0000) >> 16u32;
        let generator = match generator {
            6 => types::ReflectGenerator::KhronosLlvmSpirvTranslator,
            7 => types::ReflectGenerator::KhronosSpirvToolsAssembler,
            8 => types::ReflectGenerator::KhronosGlslangReferenceFrontEnd,
            13 => types::ReflectGenerator::GoogleShadercOverGlslang,
            14 => types::ReflectGenerator::GoogleSpiregg,
            15 => types::ReflectGenerator::GoogleRspirv,
            16 => types::ReflectGenerator::XLegendMesaMesairSpirvTranslator,
            17 => types::ReflectGenerator::KhronosSpirvToolsLinker,
            18 => types::ReflectGenerator::WineVkd3dShaderCompiler,
            19 => types::ReflectGenerator::ClayClayShaderCompiler,
            _ => types::ReflectGenerator::Unknown,
        };

        self.parse_nodes(spv_words, module)?;
        self.parse_strings(spv_words, module)?;
        self.parse_functions(spv_words, module)?;
        self.parse_member_counts(spv_words, module)?;
        self.parse_names(spv_words, module)?;
        self.parse_decorations(spv_words, module)?;

        /*
          // Zero out descriptor set data
          p_module->descriptor_set_count = 0;
          memset(p_module->descriptor_sets, 0, SPV_REFLECT_MAX_DESCRIPTOR_SETS * sizeof(*p_module->descriptor_sets));
          // Initialize descriptor set numbers
          for (uint32_t set_number = 0; set_number < SPV_REFLECT_MAX_DESCRIPTOR_SETS; ++set_number) {
            p_module->descriptor_sets[set_number].set = (uint32_t)INVALID_VALUE;
          }
        }*/

        self.parse_types(spv_words, module)?;
        self.parse_descriptor_bindings(spv_words, module)?;
        self.parse_descriptor_type(spv_words, module)?;
        self.parse_counter_bindings(spv_words, module)?;
        self.parse_descriptor_blocks(spv_words, module)?;
        self.parse_push_constant_blocks(spv_words, module)?;
        self.parse_entry_points(spv_words, module)?;

        /*if module.entry_points.len() > 0 {
            SpvReflectEntryPoint* p_entry = &(p_module->entry_points[0]);
            p_module->entry_point_name = p_entry->name;
            p_module->entry_point_id = p_entry->id;
            p_module->spirv_execution_model = p_entry->spirv_execution_model;
            p_module->shader_stage = p_entry->shader_stage;
            p_module->input_variable_count = p_entry->input_variable_count;
            p_module->input_variables = p_entry->input_variables;
            p_module->output_variable_count = p_entry->output_variable_count;
            p_module->output_variables = p_entry->output_variables;
        }*/

        /*
        if (result == SPV_REFLECT_RESULT_SUCCESS) {
            result = DisambiguateStorageBufferSrvUav(p_module);
        }
        if (result == SPV_REFLECT_RESULT_SUCCESS) {
            result = SynchronizeDescriptorSets(p_module);
        }
        */

        Ok(())
    }

    fn parse_nodes(
        &mut self,
        spv_words: &[u32],
        module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        let mut word_index = STARTING_WORD;

        // Count the nodes
        let mut node_count = 0usize;
        while word_index < spv_words.len() {
            let word = spv_words[word_index];
            let node_word_count = (word >> 16u32) & 0xFFFF;
            word_index += node_word_count as usize;
            node_count += 1;
        }

        self.nodes.resize(node_count, ParserNode::default());

        if self.nodes.len() == 0 {
            return Err("No nodes found in SPIR-V binary, invalid!".to_string());
        }

        // Restart parser and process nodes
        word_index = STARTING_WORD;

        let mut function_node: usize = std::usize::MAX;
        let mut node_index = 0;

        while word_index < spv_words.len() {
            let word = spv_words[word_index];
            {
                let mut node = &mut self.nodes[node_index];
                node.word_count = (word >> 16u32) & 0x0000FFFF;
                node.word_offset = word_index as u32;
                node.op = spirv_headers::Op::from_u32(word & 0x0000FFFF);
            }

            if let Some(op) = self.nodes[node_index].op {
                match op {
                    spirv_headers::Op::String => self.string_count += 1,
                    spirv_headers::Op::Source => {
                        module.internal.source_language =
                            spirv_headers::SourceLanguage::from_u32(spv_words[word_index + 1]);
                        module.internal.source_language_version = spv_words[word_index + 2];
                        if self.nodes[node_index].word_count >= 4 {
                            module.internal.source_file_id = spv_words[word_index + 3];
                        }
                    }
                    spirv_headers::Op::EntryPoint => self.entry_point_count += 1,
                    spirv_headers::Op::Name | spirv_headers::Op::MemberName => {
                        let member_offset: usize = if op == spirv_headers::Op::MemberName {
                            1
                        } else {
                            0
                        };
                        let name_start = word_index + member_offset + 2;
                        let mut node = &mut self.nodes[node_index];
                        node.name = unsafe {
                            let name_ptr =
                                spv_words.as_ptr().offset(name_start as isize) as *const c_char;
                            let name_str = CStr::from_ptr(name_ptr);
                            name_str.to_str().unwrap().to_owned()
                        };
                        println!("Name: {}", &self.nodes[node_index].name);
                    }
                    spirv_headers::Op::TypeStruct => {
                        let mut node = &mut self.nodes[node_index];
                        node.member_count = node.word_count - 2;
                        node.result_id = spv_words[word_index + 1];
                        node.is_type = true;
                    }
                    spirv_headers::Op::TypeVoid
                    | spirv_headers::Op::TypeBool
                    | spirv_headers::Op::TypeInt
                    | spirv_headers::Op::TypeFloat
                    | spirv_headers::Op::TypeVector
                    | spirv_headers::Op::TypeMatrix
                    | spirv_headers::Op::TypeSampler
                    | spirv_headers::Op::TypeOpaque
                    | spirv_headers::Op::TypeFunction
                    | spirv_headers::Op::TypeEvent
                    | spirv_headers::Op::TypeDeviceEvent
                    | spirv_headers::Op::TypeReserveId
                    | spirv_headers::Op::TypeQueue
                    | spirv_headers::Op::TypePipe
                    | spirv_headers::Op::TypeAccelerationStructureNV => {
                        let mut node = &mut self.nodes[node_index];
                        node.result_id = spv_words[word_index + 1];
                        node.is_type = true;
                    }
                    spirv_headers::Op::TypeImage => {
                        let mut node = &mut self.nodes[node_index];
                        node.result_id = spv_words[word_index + 1];
                        node.image_traits.sampled_type_id = spv_words[word_index + 2];
                        node.image_traits.dim =
                            spirv_headers::Dim::from_u32(spv_words[word_index + 3]);
                        node.image_traits.depth = spv_words[word_index + 4];
                        node.image_traits.arrayed = spv_words[word_index + 5];
                        node.image_traits.ms = spv_words[word_index + 6];
                        node.image_traits.sampled = spv_words[word_index + 7];
                        node.image_traits.image_format =
                            spirv_headers::ImageFormat::from_u32(spv_words[word_index + 8]);
                        node.is_type = true;
                    }
                    spirv_headers::Op::TypeSampledImage => {
                        let mut node = &mut self.nodes[node_index];
                        node.result_id = spv_words[word_index + 1];
                        node.image_type_id = spv_words[word_index + 2];
                        node.is_type = true;
                    }
                    spirv_headers::Op::TypeArray => {
                        let mut node = &mut self.nodes[node_index];
                        node.result_id = spv_words[word_index + 1];
                        node.array_traits.element_type_id = spv_words[word_index + 2];
                        node.array_traits.length_id = spv_words[word_index + 3];
                        node.is_type = true;
                    }
                    spirv_headers::Op::TypeRuntimeArray => {
                        let mut node = &mut self.nodes[node_index];
                        node.result_id = spv_words[word_index + 1];
                        node.array_traits.element_type_id = spv_words[word_index + 2];
                        node.is_type = true;
                    }
                    spirv_headers::Op::TypePointer => {
                        let mut node = &mut self.nodes[node_index];
                        node.result_id = spv_words[word_index + 1];
                        node.storage_class =
                            spirv_headers::StorageClass::from_u32(spv_words[word_index + 2]);
                        node.type_id = spv_words[word_index + 3];
                        node.is_type = true;
                    }
                    spirv_headers::Op::TypeForwardPointer => {
                        let mut node = &mut self.nodes[node_index];
                        node.result_id = spv_words[word_index + 1];
                        node.storage_class =
                            spirv_headers::StorageClass::from_u32(spv_words[word_index + 2]);
                        node.is_type = true;
                    }
                    spirv_headers::Op::ConstantTrue
                    | spirv_headers::Op::ConstantFalse
                    | spirv_headers::Op::Constant
                    | spirv_headers::Op::ConstantComposite
                    | spirv_headers::Op::ConstantSampler
                    | spirv_headers::Op::ConstantNull => {
                        let mut node = &mut self.nodes[node_index];
                        node.result_type_id = spv_words[word_index + 1];
                        node.result_id = spv_words[word_index + 2];
                    }
                    spirv_headers::Op::Variable => {
                        let mut node = &mut self.nodes[node_index];
                        node.type_id = spv_words[word_index + 1];
                        node.result_id = spv_words[word_index + 2];
                        node.storage_class =
                            spirv_headers::StorageClass::from_u32(spv_words[word_index + 3]);
                    }
                    spirv_headers::Op::Load => {
                        let mut node = &mut self.nodes[node_index];
                        node.result_type_id = spv_words[word_index + 1];
                        node.result_id = spv_words[word_index + 2];
                    }
                    spirv_headers::Op::Function => {
                        let mut node = &mut self.nodes[node_index];
                        node.result_id = spv_words[word_index + 2];
                        function_node = node_index;
                    }
                    spirv_headers::Op::Label => {
                        if function_node != std::usize::MAX {
                            let mut node = &mut self.nodes[function_node];
                            node.result_id = spv_words[node.word_offset as usize + 2];
                            self.function_count += 1;
                        }
                    }
                    spirv_headers::Op::FunctionEnd => function_node = std::usize::MAX,
                    _ => {}
                }
            } else {
                return Err("Invalid SPIR-V op!".to_string());
            }

            let node = &self.nodes[node_index];
            if node.is_type {
                self.type_count += 1;
            }
            word_index += node.word_count as usize;
            node_index += 1;
        }

        Ok(())
    }

    fn parse_strings(
        &mut self,
        spv_words: &[u32],
        module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        if self.string_count > 0 && spv_words.len() > 0 && self.nodes.len() > 0 {
            self.strings.reserve(self.string_count);
            for node in &self.nodes {
                if let Some(op) = node.op {
                    if op != spirv_headers::Op::String {
                        continue;
                    }

                    if self.strings.len() >= self.string_count {
                        return Err("Count mismatch while parsing strings.".into());
                    }

                    let string_start = node.word_offset as usize + 2;
                    let string_value = unsafe {
                        let string_ptr =
                            spv_words.as_ptr().offset(string_start as isize) as *const c_char;
                        let string_str = CStr::from_ptr(string_ptr);
                        string_str.to_str().unwrap().to_owned()
                    };

                    self.strings.push(ParserString {
                        result_id: spv_words[node.word_offset as usize + 1],
                        string: string_value,
                    });
                }
            }

            for string in &self.strings {
                if string.result_id == module.internal.source_file_id {
                    module.internal.source_file = string.string.to_owned();
                    break;
                }
            }
        }

        Ok(())
    }

    fn parse_functions(
        &mut self,
        spv_words: &[u32],
        module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        Ok(())
    }

    fn parse_member_counts(
        &mut self,
        spv_words: &[u32],
        _: &mut super::ShaderModule,
    ) -> Result<(), String> {
        for node_index in 0..self.nodes.len() {
            let op = &self.nodes[node_index].op;
            if op != &Some(spirv_headers::Op::MemberName)
                && op != &Some(spirv_headers::Op::MemberDecorate)
            {
                continue;
            }

            let word_offset = self.nodes[node_index].word_offset as usize;
            let target_id = spv_words[word_offset + 1];
            let member_index = spv_words[word_offset + 2];

            // Not all nodes are parsed
            if let Some(target_node_index) = self.find_node(target_id) {
                let mut target_node = &mut self.nodes[target_node_index];
                target_node.member_count =
                    std::cmp::max(target_node.member_count, member_index + 1);
            }
        }

        for node in &mut self.nodes {
            if node.member_count == 0 {
                continue;
            }

            node.member_names
                .resize(node.member_count as usize, String::new());
            node.member_decorations
                .resize(node.member_count as usize, Decorations::default());
        }

        Ok(())
    }

    fn parse_names(
        &mut self,
        spv_words: &[u32],
        module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        Ok(())
    }

    fn parse_decorations(
        &mut self,
        spv_words: &[u32],
        module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        Ok(())
    }

    fn parse_types(
        &mut self,
        spv_words: &[u32],
        module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        Ok(())
    }

    fn parse_descriptor_bindings(
        &mut self,
        spv_words: &[u32],
        module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        Ok(())
    }

    fn parse_descriptor_type(
        &mut self,
        spv_words: &[u32],
        module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        Ok(())
    }

    fn parse_counter_bindings(
        &mut self,
        spv_words: &[u32],
        module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        Ok(())
    }

    fn parse_descriptor_blocks(
        &mut self,
        spv_words: &[u32],
        module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        Ok(())
    }

    fn parse_push_constant_blocks(
        &mut self,
        spv_words: &[u32],
        module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        Ok(())
    }

    fn parse_entry_points(
        &mut self,
        spv_words: &[u32],
        module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        module.internal.entry_points.reserve(self.entry_point_count);
        let uniforms = Self::enumerate_all_uniforms(module);
        //let push_constants = Self::enumerate_all_push_constants(module);

        for node in &self.nodes {
            let word_offset = node.word_offset as usize;
            if let Some(spirv_execution_model) = spirv_headers::ExecutionModel::from_u32(spv_words[word_offset + 1]) {
              let entry_point = crate::types::variable::ReflectEntryPoint {
                  spirv_execution_model,
                  id: spv_words[word_offset + 2],
                  shader_stage: match spirv_execution_model {
                      spirv_headers::ExecutionModel::Vertex => crate::types::ReflectShaderStageFlags::VERTEX,
                      spirv_headers::ExecutionModel::TessellationControl => crate::types::ReflectShaderStageFlags::TESSELLATION_CONTROL,
                      spirv_headers::ExecutionModel::TessellationEvaluation => crate::types::ReflectShaderStageFlags::TESSELLATION_EVALUATION,
                      spirv_headers::ExecutionModel::Geometry => crate::types::ReflectShaderStageFlags::GEOMETRY,
                      spirv_headers::ExecutionModel::Fragment => crate::types::ReflectShaderStageFlags::FRAGMENT,
                      spirv_headers::ExecutionModel::GLCompute => crate::types::ReflectShaderStageFlags::COMPUTE,
                      // TODO:
                      /*spirv_headers::ExecutionModel::RayGenerationNV => crate::types::ReflectShaderStageFlags::RAYGEN_BIT_NV,
                      spirv_headers::ExecutionModel::IntersectionNV => crate::types::ReflectShaderStageFlags::INTERSECTION_BIT_NV,
                      spirv_headers::ExecutionModel::AnyHitNV => crate::types::ReflectShaderStageFlags::ANY_HIT_BIT_NV,
                      spirv_headers::ExecutionModel::ClosestHitNV => crate::types::ReflectShaderStageFlags::CLOSEST_HIT_BIT_NV,
                      spirv_headers::ExecutionModel::MissNV => crate::types::ReflectShaderStageFlags::MISS_BIT_NV,
                      spirv_headers::ExecutionModel::CallableNV => crate::types::ReflectShaderStageFlags::CALLABLE_BIT_NV,*/

                  }
              };

              module.internal.entry_points.push(entry_point);
            }
        }

        Ok(())
    }

    fn find_node(&self, result_id: u32) -> Option<usize> {
        for node_index in 0..self.nodes.len() {
            let node = &self.nodes[node_index];
            if node.result_id == result_id {
                return Some(node_index);
            }
        }

        None
    }

    fn enumerate_all_uniforms(module: &super::ShaderModule) -> Vec<u32> {
        let mut uniforms: Vec<u32> = Vec::new();
        
        if module.internal.descriptor_bindings.len() > 0 {
            uniforms.reserve(module.internal.descriptor_bindings.len());
            for descriptor_binding in &module.internal.descriptor_bindings {
                uniforms.push(descriptor_binding.spirv_id);
            }

            uniforms.sort_by(|a, b| a.cmp(b));
        }

        uniforms
    }
}

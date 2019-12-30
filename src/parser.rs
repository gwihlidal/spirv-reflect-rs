use crate::num_traits::FromPrimitive;
use crate::types;
use std::ffi::CStr;
use std::os::raw::c_char;

pub const STARTING_WORD: usize = 5;
pub const SPIRV_WORD_SIZE: usize = std::mem::size_of::<u32>();

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
    //pub is_built_in: bool,
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

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ParserNode {
    pub result_id: u32,
    pub op: spirv_headers::Op,
    pub result_type_id: u32,
    pub type_id: u32,
    pub storage_class: spirv_headers::StorageClass,
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

impl Default for ParserNode {
    fn default() -> ParserNode {
        Self {
            result_id: 0,
            op: spirv_headers::Op::Undef,
            result_type_id: 0,
            type_id: 0,
            storage_class: spirv_headers::StorageClass::UniformConstant,
            word_offset: 0,
            word_count: 0,
            is_type: false,
            array_traits: ParserArrayTraits::default(),
            image_traits: ParserImageTraits::default(),
            image_type_id: 0,
            name: String::new(),
            decorations: Decorations::default(),
            member_count: 0,
            member_names: Vec::new(),
            member_decorations: Vec::new(),
        }
    }
}

#[derive(Default, Debug)]
pub(crate) struct ParserString {
    pub result_id: u32,
    pub string: String,
}

#[derive(Default, Debug)]
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
        let _generator = match generator {
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

        // TODO:

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

        // TODO:

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
                if let Some(op) = spirv_headers::Op::from_u32(word & 0x0000FFFF) {
                    node.op = op;
                } else {
                    return Err("Invalid SPIR-V op!".into());
                }
            }

            match self.nodes[node_index].op {
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
                    let member_offset: usize =
                        if self.nodes[node_index].op == spirv_headers::Op::MemberName {
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
                    node.image_traits.dim = spirv_headers::Dim::from_u32(spv_words[word_index + 3]);
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
                    if let Some(storage_class) =
                        spirv_headers::StorageClass::from_u32(spv_words[word_index + 2])
                    {
                        node.storage_class = storage_class;
                    } else {
                        return Err("Invalid SPIR-V storage class!".into());
                    }
                    node.type_id = spv_words[word_index + 3];
                    node.is_type = true;
                }
                spirv_headers::Op::TypeForwardPointer => {
                    let mut node = &mut self.nodes[node_index];
                    node.result_id = spv_words[word_index + 1];
                    if let Some(storage_class) =
                        spirv_headers::StorageClass::from_u32(spv_words[word_index + 2])
                    {
                        node.storage_class = storage_class;
                    } else {
                        return Err("Invalid SPIR-V storage class!".into());
                    }
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
                    if let Some(storage_class) =
                        spirv_headers::StorageClass::from_u32(spv_words[word_index + 3])
                    {
                        node.storage_class = storage_class;
                    } else {
                        return Err("Invalid SPIR-V storage class!".into());
                    }
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
                if node.op != spirv_headers::Op::String {
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
        _spv_words: &[u32],
        _module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        println!("UNIMPLEMENTED - parse_functions");
        Ok(())
    }

    fn parse_member_counts(
        &mut self,
        spv_words: &[u32],
        _: &mut super::ShaderModule,
    ) -> Result<(), String> {
        for node_index in 0..self.nodes.len() {
            let op = &self.nodes[node_index].op;
            if op != &spirv_headers::Op::MemberName && op != &spirv_headers::Op::MemberDecorate {
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
        _spv_words: &[u32],
        _module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        println!("UNIMPLEMENTED - parse_names");
        Ok(())
    }

    fn parse_decorations(
        &mut self,
        _spv_words: &[u32],
        _module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        println!("UNIMPLEMENTED - parse_decorations");
        Ok(())
    }

    fn apply_decorations(
        decorations: &Decorations,
    ) -> Result<crate::types::ReflectDecorationFlags, String> {
        let mut flags = crate::types::ReflectDecorationFlags::NONE;

        if decorations.is_block {
            flags |= crate::types::ReflectDecorationFlags::BLOCK;
        }

        if decorations.is_buffer_block {
            flags |= crate::types::ReflectDecorationFlags::BUFFER_BLOCK;
        }

        if decorations.is_row_major {
            flags |= crate::types::ReflectDecorationFlags::ROW_MAJOR;
        }

        if decorations.is_column_major {
            flags |= crate::types::ReflectDecorationFlags::COLUMN_MAJOR;
        }

        if decorations.built_in.is_some() {
            flags |= crate::types::ReflectDecorationFlags::BUILT_IN;
        }

        if decorations.is_noperspective {
            flags |= crate::types::ReflectDecorationFlags::NO_PERSPECTIVE;
        }

        if decorations.is_flat {
            flags |= crate::types::ReflectDecorationFlags::FLAT;
        }

        if decorations.is_non_writable {
            flags |= crate::types::ReflectDecorationFlags::NON_WRITABLE;
        }

        Ok(flags)
    }

    fn parse_type(
        &mut self,
        spv_words: &[u32],
        module: &mut super::ShaderModule,
        node_index: usize,
        struct_member_decorations: Option<(/* node */ usize, /* member */ usize)>,
        type_description: &mut crate::types::ReflectTypeDescription,
    ) -> Result<(), String> {
        let word_offset = self.nodes[node_index].word_offset as usize;
        type_description.members.resize(
            self.nodes[node_index].member_count as usize,
            crate::types::ReflectTypeDescription::default(),
        );

        if type_description.id == std::u32::MAX {
            type_description.id = self.nodes[node_index].result_id;
            type_description.op = crate::types::ReflectOp(self.nodes[node_index].op);
            type_description.decoration_flags = crate::types::ReflectDecorationFlags::NONE;
        }

        type_description.decoration_flags |=
            Self::apply_decorations(&self.nodes[node_index].decorations)?;

        match self.nodes[node_index].op {
            spirv_headers::Op::TypeOpaque => {}
            spirv_headers::Op::TypeVoid => {
                type_description.type_flags |= crate::types::ReflectTypeFlags::VOID
            }
            spirv_headers::Op::TypeBool => {
                type_description.type_flags |= crate::types::ReflectTypeFlags::BOOL
            }
            spirv_headers::Op::TypeSampler => {
                type_description.type_flags |= crate::types::ReflectTypeFlags::EXTERNAL_SAMPLER
            }
            spirv_headers::Op::TypeInt => {
                type_description.type_flags |= crate::types::ReflectTypeFlags::INT;
                type_description.traits.numeric.scalar.width = spv_words[word_offset + 2];
                type_description.traits.numeric.scalar.signedness = spv_words[word_offset + 3];
            }
            spirv_headers::Op::TypeFloat => {
                type_description.type_flags |= crate::types::ReflectTypeFlags::FLOAT;
                type_description.traits.numeric.scalar.width = spv_words[word_offset + 2];
            }
            spirv_headers::Op::TypeVector => {
                type_description.type_flags |= crate::types::ReflectTypeFlags::VECTOR;
                let component_type_id = spv_words[word_offset + 2];
                type_description.traits.numeric.vector.component_count = spv_words[word_offset + 3];
                if let Some(next_node_index) = self.find_node(component_type_id) {
                    self.parse_type(&spv_words, module, next_node_index, None, type_description)?;
                } else {
                    return Err("Invalid SPIR-V ID reference".into());
                }
            }
            spirv_headers::Op::TypeMatrix => {
                type_description.type_flags |= crate::types::ReflectTypeFlags::MATRIX;
                let column_type_id = spv_words[word_offset + 2];
                type_description.traits.numeric.matrix.column_count = spv_words[word_offset + 3];
                if let Some(next_node_index) = self.find_node(column_type_id) {
                    self.parse_type(&spv_words, module, next_node_index, None, type_description)?;
                } else {
                    return Err("Invalid SPIR-V ID reference".into());
                }
                type_description.traits.numeric.matrix.row_count =
                    type_description.traits.numeric.vector.component_count;
                if let Some(ref struct_member_index) = struct_member_decorations {
                    let member_node = &self.nodes[struct_member_index.0];
                    let member_decorations = &member_node.member_decorations[struct_member_index.1];
                    type_description.traits.numeric.matrix.stride =
                        member_decorations.matrix_stride;
                } else {
                    type_description.traits.numeric.matrix.stride =
                        self.nodes[node_index].decorations.matrix_stride;
                }
            }
            spirv_headers::Op::TypeImage => {
                type_description.type_flags |= crate::types::ReflectTypeFlags::EXTERNAL_IMAGE;
                type_description.traits.image.dim =
                    spirv_headers::Dim::from_u32(spv_words[word_offset + 3]).into();
                type_description.traits.image.depth = spv_words[word_offset + 4];
                type_description.traits.image.arrayed = spv_words[word_offset + 5];
                type_description.traits.image.ms = spv_words[word_offset + 6];
                type_description.traits.image.sampled = spv_words[word_offset + 7];
                type_description.traits.image.image_format =
                    spirv_headers::ImageFormat::from_u32(spv_words[word_offset + 8]).into();
            }
            spirv_headers::Op::TypeSampledImage => {
                type_description.type_flags |=
                    crate::types::ReflectTypeFlags::EXTERNAL_SAMPLED_IMAGE;
                let image_type_id = spv_words[word_offset + 2];
                if let Some(next_node_index) = self.find_node(image_type_id) {
                    self.parse_type(&spv_words, module, next_node_index, None, type_description)?;
                } else {
                    return Err("Invalid SPIR-V ID reference".into());
                }
            }
            spirv_headers::Op::TypeArray => {
                type_description.type_flags |= crate::types::ReflectTypeFlags::ARRAY;
                let element_type_id = spv_words[word_offset + 2];
                let length_id = spv_words[word_offset + 3];
                type_description.traits.array.stride =
                    self.nodes[node_index].decorations.array_stride;
                if let Some(length_node_index) = self.find_node(length_id) {
                    let length = spv_words[self.nodes[length_node_index].word_offset as usize + 3];
                    type_description.traits.array.dims.push(length);
                    if let Some(next_node_index) = self.find_node(element_type_id) {
                        self.parse_type(
                            &spv_words,
                            module,
                            next_node_index,
                            None,
                            type_description,
                        )?;
                    } else {
                        return Err("Invalid SPIR-V ID reference".into());
                    }
                } else {
                    return Err("Invalid SPIR-V ID reference".into());
                }
            }
            spirv_headers::Op::TypeRuntimeArray => {
                let element_type_id = spv_words[word_offset + 2];
                if let Some(next_node_index) = self.find_node(element_type_id) {
                    self.parse_type(&spv_words, module, next_node_index, None, type_description)?;
                } else {
                    return Err("Invalid SPIR-V ID reference".into());
                }
            }
            spirv_headers::Op::TypeStruct => {
                type_description.type_flags |= crate::types::ReflectTypeFlags::STRUCT
                    | crate::types::ReflectTypeFlags::EXTERNAL_BLOCK;
                let mut member_index = 0;
                for word_index in 2..self.nodes[node_index].word_count as usize {
                    let member_id = spv_words[word_offset + word_index];
                    if let Some(member_node_index) = self.find_node(member_id) {
                        dbg!(&type_description.members);
                        assert!(member_index < type_description.members.len());
                        let mut member_type_description =
                            &mut type_description.members[member_index];
                        member_type_description.id = member_id;
                        member_type_description.op =
                            crate::types::ReflectOp(self.nodes[member_node_index].op);
                        self.parse_type(
                            &spv_words,
                            module,
                            member_node_index,
                            Some((node_index, member_node_index)),
                            &mut member_type_description,
                        )?;
                        member_type_description.struct_member_name =
                            self.nodes[node_index].member_names[member_index].to_owned();
                    } else {
                        return Err("Invalid SPIR-V ID reference".into());
                    }

                    member_index += 1;
                }
            }
            spirv_headers::Op::TypePointer => {
                type_description.type_flags |= crate::types::ReflectTypeFlags::STRUCT
                    | crate::types::ReflectTypeFlags::EXTERNAL_BLOCK;
                type_description.storage_class =
                    spirv_headers::StorageClass::from_u32(spv_words[word_offset + 2]).into();
                if type_description.storage_class == crate::types::ReflectStorageClass::Undefined {
                    return Err("Invalid SPIR-V ID reference".into());
                }
                let type_id = spv_words[word_offset + 3];
                if let Some(next_node_index) = self.find_node(type_id) {
                    self.parse_type(&spv_words, module, next_node_index, None, type_description)?;
                } else {
                    return Err("Invalid SPIR-V ID reference".into());
                }
            }
            _ => {}
        }

        if type_description.type_name.is_empty() {
            type_description.type_name = self.nodes[node_index].name.to_owned();
        }

        Ok(())
    }

    fn parse_types(
        &mut self,
        spv_words: &[u32],
        module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        module.internal.type_descriptions.reserve(self.type_count);
        for node_index in 0..self.nodes.len() {
            if !self.nodes[node_index].is_type {
                continue;
            }
            let mut type_description = crate::types::ReflectTypeDescription::default();
            self.parse_type(&spv_words, module, node_index, None, &mut type_description)?;
            module.internal.type_descriptions.push(type_description);
        }
        Ok(())
    }

    fn parse_descriptor_bindings(
        &mut self,
        _spv_words: &[u32],
        _module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        println!("UNIMPLEMENTED - parse_descriptor_bindings");
        Ok(())
    }

    fn parse_descriptor_type(
        &mut self,
        _spv_words: &[u32],
        _module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        println!("UNIMPLEMENTED - parse_descriptor_type");
        Ok(())
    }

    fn parse_counter_bindings(
        &mut self,
        _spv_words: &[u32],
        _module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        println!("UNIMPLEMENTED - parse_counter_bindings");
        Ok(())
    }

    fn parse_descriptor_blocks(
        &mut self,
        _spv_words: &[u32],
        _module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        println!("UNIMPLEMENTED - parse_descriptor_blocks");
        Ok(())
    }

    fn parse_push_constant_blocks(
        &mut self,
        _spv_words: &[u32],
        _module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        println!("UNIMPLEMENTED - parse_push_constant_blocks");
        Ok(())
    }

    fn parse_interface_variable(
        &self,
        _module: &super::ShaderModule,
        _built_in: &mut bool,
        _type_decorations: &Decorations,
        _type_description: &crate::types::ReflectTypeDescription,
    ) -> Result<(), String> {
        println!("UNIMPLEMENTED - parse_interface_variable");
        Ok(())
    }

    fn parse_interface_variables(
        &self,
        _spv_words: &[u32],
        module: &mut super::ShaderModule,
        interface_vars: &[u32],
        entry_point: &mut crate::types::variable::ReflectEntryPoint,
    ) -> Result<(), String> {
        if interface_vars.len() > 0 {
            let mut input_count = 0;
            let mut output_count = 0;
            for var_id in interface_vars {
                if let Some(node_index) = self.find_node(*var_id) {
                    let node = &self.nodes[node_index];
                    match node.storage_class {
                        spirv_headers::StorageClass::Input => input_count += 1,
                        spirv_headers::StorageClass::Output => output_count += 1,
                        _ => {}
                    }
                } else {
                    return Err("Invalid SPIR-V ID reference".into());
                }
            }

            entry_point.input_variables.reserve(input_count);
            entry_point.output_variables.reserve(output_count);

            for var_id in interface_vars {
                if let Some(node_index) = self.find_node(*var_id) {
                    let node = &self.nodes[node_index];
                    if let Some(type_index) = module.internal.find_type(node.type_id) {
                        let mut type_description = &module.internal.type_descriptions[type_index];

                        // Resolve pointer types
                        if *type_description.op == spirv_headers::Op::TypePointer {
                            if let Some(type_node_index) = self.find_node(type_description.id) {
                                let type_node = &self.nodes[type_node_index];
                                if let Some(pointer_type_index) =
                                    module.internal.find_type(type_node.type_id)
                                {
                                    type_description =
                                        &module.internal.type_descriptions[pointer_type_index];
                                } else {
                                    return Err("Invalid SPIR-V ID reference".into());
                                }
                            } else {
                                return Err("Invalid SPIR-V ID reference".into());
                            }
                        }

                        if let Some(type_node_index) = self.find_node(type_description.id) {
                            let type_node = &self.nodes[type_node_index];
                            let type_decorations = &type_node.decorations;

                            let mut variable =
                                crate::types::variable::ReflectInterfaceVariable::default();
                            match node.storage_class {
                                spirv_headers::StorageClass::Input => {
                                    variable.storage_class =
                                        crate::types::ReflectStorageClass::Input
                                }
                                spirv_headers::StorageClass::Output => {
                                    variable.storage_class =
                                        crate::types::ReflectStorageClass::Output
                                }
                                _ => return Err("Invalid SPIR-V ID storage class".into()),
                            }

                            let mut built_in = node.decorations.built_in.is_some();
                            self.parse_interface_variable(
                                &module,
                                &mut built_in,
                                &type_decorations,
                                &type_description,
                            )?;

                            variable.spirv_id = node.result_id;
                            variable.name = node.name.to_owned();
                            variable.semantic = node.decorations.semantic.value.to_owned();
                            if built_in {
                                variable.decoration_flags |=
                                    crate::types::ReflectDecorationFlags::BUILT_IN;
                            }
                            variable.location = node.decorations.location.value;
                            variable.word_offset = node.decorations.location.word_offset;
                            if let Some(built_in) = node.decorations.built_in {
                                variable.built_in = crate::types::ReflectBuiltIn(built_in);
                            }

                            match variable.storage_class {
                                crate::types::ReflectStorageClass::Input => {
                                    entry_point.input_variables.push(variable)
                                }
                                crate::types::ReflectStorageClass::Output => {
                                    entry_point.output_variables.push(variable)
                                }
                                _ => {}
                            }
                        } else {
                            return Err("Invalid SPIR-V ID reference".into());
                        }
                    } else {
                        return Err("Invalid SPIR-V ID reference".into());
                    }
                } else {
                    return Err("Invalid SPIR-V ID reference".into());
                }
            }
        }
        Ok(())
    }

    fn parse_static_resources(
        &self,
        _spv_words: &[u32],
        _module: &mut super::ShaderModule,
        _uniforms: &[u32],
        _push_constants: &[u32],
        _entry_point: &mut crate::types::variable::ReflectEntryPoint,
    ) -> Result<(), String> {
        println!("UNIMPLEMENTED - parse_static_resources");
        Ok(())
    }

    fn parse_entry_points(
        &mut self,
        spv_words: &[u32],
        module: &mut super::ShaderModule,
    ) -> Result<(), String> {
        module.internal.entry_points.reserve(self.entry_point_count);
        let uniforms = Self::enumerate_all_uniforms(module);
        let push_constants = Self::enumerate_all_push_constants(module);

        for node in &self.nodes {
            if node.op != spirv_headers::Op::EntryPoint {
                continue;
            }

            let word_offset = node.word_offset as usize;
            let word_count = node.word_count as usize;

            let spirv_execution_model =
                spirv_headers::ExecutionModel::from_u32(spv_words[word_offset + 1]);
            let shader_stage = match spirv_execution_model {
                Some(spirv_headers::ExecutionModel::Vertex) => {
                    crate::types::ReflectShaderStage::Vertex
                }
                Some(spirv_headers::ExecutionModel::TessellationControl) => {
                    crate::types::ReflectShaderStage::TessellationControl
                }
                Some(spirv_headers::ExecutionModel::TessellationEvaluation) => {
                    crate::types::ReflectShaderStage::TessellationEvaluation
                }
                Some(spirv_headers::ExecutionModel::Geometry) => {
                    crate::types::ReflectShaderStage::Geometry
                }
                Some(spirv_headers::ExecutionModel::Fragment) => {
                    crate::types::ReflectShaderStage::Fragment
                }
                Some(spirv_headers::ExecutionModel::GLCompute) => {
                    crate::types::ReflectShaderStage::Compute
                }
                Some(spirv_headers::ExecutionModel::Kernel) => {
                    crate::types::ReflectShaderStage::Kernel
                }
                _ => {
                    // TODO: Get NV support in spirv_headers. For now, parse it directly from raw
                    // https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_execution_model_a_execution_model
                    match spv_words[word_offset + 1] {
                        5267 => crate::types::ReflectShaderStage::TaskNV,
                        5268 => crate::types::ReflectShaderStage::MeshNV,
                        5313 => crate::types::ReflectShaderStage::RayGenerationNV,
                        5314 => crate::types::ReflectShaderStage::IntersectionNV,
                        5315 => crate::types::ReflectShaderStage::AnyHitNV,
                        5316 => crate::types::ReflectShaderStage::ClosestHitNV,
                        5317 => crate::types::ReflectShaderStage::MissNV,
                        5318 => crate::types::ReflectShaderStage::CallableNV,
                        _ => crate::types::ReflectShaderStage::Undefined,
                    }
                }
            };

            // The name string length determines the next operand offset.
            let name_start_offset = 3;
            let name = unsafe {
                let name_offset = word_offset + name_start_offset;
                if name_offset + word_count >= spv_words.len() {
                    return Err("Count mismatch while parsing strings.".into());
                }

                // We want to take a byte slice of the valid name string range, since we can't assume
                // it is a valid null terminated string.
                let name_ptr = spv_words.as_ptr().offset(name_offset as isize) as *const _;
                let name_slice = std::slice::from_raw_parts(name_ptr, word_count * SPIRV_WORD_SIZE);
                let name_slice_end = name_slice.iter().position(|&b| b == 0).map_or(0, |i| i + 1);

                // Convert the slice to a string (if it's corectly null terminated).
                let name_str = CStr::from_bytes_with_nul(&name_slice[..name_slice_end]);
                if name_str.is_err() {
                    return Err("Entry point name is not a valid string.".into());
                }
                let name_str = name_str.unwrap();

                // Convert ffi to string
                name_str.to_str().unwrap().to_owned()
            };

            let name_length_with_null = name.len() + 1;
            let name_word_count =
                (name_length_with_null + SPIRV_WORD_SIZE - 1) & !(SPIRV_WORD_SIZE - 1);
            let name_word_count = name_word_count / SPIRV_WORD_SIZE;

            let mut entry_point = crate::types::variable::ReflectEntryPoint {
                name,
                spirv_execution_model,
                id: spv_words[word_offset + 2],
                shader_stage,
                input_variables: Vec::new(),
                output_variables: Vec::new(),
                descriptor_sets: Vec::new(),
                used_uniforms: Vec::new(),
                used_push_constants: Vec::new(),
            };

            let interface_var_count = word_count - (name_start_offset + name_word_count);
            let interface_var_offset = name_start_offset + name_word_count;
            let mut interface_vars = Vec::with_capacity(interface_var_count);
            for var_index in 0..interface_var_count {
                let var_offset = interface_var_offset + var_index;
                interface_vars.push(spv_words[word_offset + var_offset]);
            }

            self.parse_interface_variables(spv_words, module, &interface_vars, &mut entry_point)?;
            self.parse_static_resources(
                spv_words,
                module,
                &uniforms,
                &push_constants,
                &mut entry_point,
            )?;

            module.internal.entry_points.push(entry_point);
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

    fn enumerate_all_push_constants(_module: &super::ShaderModule) -> Vec<u32> {
        println!("UNIMPLEMENTED - enumerate_all_push_constants");
        Vec::new()
    }
}

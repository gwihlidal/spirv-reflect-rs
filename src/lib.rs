#[macro_use]
extern crate bitflags;
extern crate num_traits;
extern crate spirv_headers;

use num_traits::cast::FromPrimitive;

pub mod convert;
pub mod ffi;
pub mod types;

pub fn ffi_to_string(ffi: *const ::std::os::raw::c_char) -> String {
    if ffi.is_null() {
        String::new()
    } else {
        let c_str: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(ffi) };
        let str_slice: &str = match c_str.to_str() {
            Ok(c_str) => c_str,
            Err(_) => &"",
        };
        str_slice.to_owned()
    }
}

impl Default for ffi::SpvReflectShaderModule {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl Default for ffi::SpvReflectDescriptorSet {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[derive(Default, Clone)]
pub struct ShaderModule {
    module: Option<ffi::SpvReflectShaderModule>,
}

impl ShaderModule {
    pub fn get_code_size(&self) -> usize {
        match self.module {
            Some(module) => unsafe { ffi::spvReflectGetCodeSize(&module) as usize },
            None => 0,
        }
    }

    pub fn get_code_slice(&self) -> &[u32] {
        let code_size = self.get_code_size();
        let module = self.module.unwrap();
        unsafe { std::slice::from_raw_parts(ffi::spvReflectGetCode(&module), code_size / 4) }
    }

    pub fn get_generator(&self) -> types::ReflectGenerator {
        match self.module {
            Some(module) => convert::ffi_to_generator(module.generator),
            None => types::ReflectGenerator::Unknown,
        }
    }

    pub fn get_shader_stage(&self) -> types::ReflectShaderStageFlags {
        match self.module {
            Some(module) => convert::ffi_to_shader_stage_flags(module.shader_stage),
            None => types::ReflectShaderStageFlags::UNDEFINED,
        }
    }

    pub fn get_source_language(&self) -> spirv_headers::SourceLanguage {
        match self.module {
            Some(module) => match spirv_headers::SourceLanguage::from_i32(module.source_language) {
                Some(language) => language,
                None => spirv_headers::SourceLanguage::Unknown,
            },
            None => spirv_headers::SourceLanguage::Unknown,
        }
    }

    pub fn get_source_language_version(&self) -> u32 {
        match self.module {
            Some(module) => module.source_language_version,
            None => 0,
        }
    }

    pub fn get_source_file(&self) -> String {
        match self.module {
            Some(module) => ffi_to_string(module.source_file),
            None => String::new(),
        }
    }

    pub fn get_source_text(&self) -> String {
        match self.module {
            Some(module) => ffi_to_string(module.source_source),
            None => String::new(),
        }
    }

    pub fn get_spirv_execution_model(&self) -> spirv_headers::ExecutionModel {
        match self.module {
            Some(module) => {
                match spirv_headers::ExecutionModel::from_i32(module.spirv_execution_model) {
                    Some(model) => model,
                    None => spirv_headers::ExecutionModel::Vertex,
                }
            }
            None => spirv_headers::ExecutionModel::Vertex,
        }
    }

    /*

    pub entry_point_id: u32,
    pub entry_point_count: u32,
    pub entry_points: *mut SpvReflectEntryPoint,
    pub descriptor_binding_count: u32,
    pub descriptor_bindings: *mut SpvReflectDescriptorBinding,
    pub input_variable_count: u32,
    pub input_variables: *mut SpvReflectInterfaceVariable,
    pub output_variable_count: u32,
    pub output_variables: *mut SpvReflectInterfaceVariable,
    pub push_constant_block_count: u32,
    pub push_constant_blocks: *mut SpvReflectBlockVariable,
}
*/

    pub fn descriptor_set_count(&self) -> Result<u32, &str> {
        match self.module {
            Some(module) => {
                let mut count: u32 = 0;
                let result = unsafe {
                    ffi::spvReflectEnumerateDescriptorSets(
                        &module,
                        &mut count,
                        ::std::ptr::null_mut(),
                    )
                };
                match result {
                    ffi::SpvReflectResult_SPV_REFLECT_RESULT_SUCCESS => Ok(count),
                    _ => Err(convert::result_to_string(result)),
                }
            }
            None => Ok(0),
        }
    }

    pub fn descriptor_sets(&self) -> Result<Vec<types::ReflectDescriptorSet>, &str> {
        let count = self.descriptor_set_count()?;
        if let Some(ref module) = self.module {
            if count > 0 {
                let mut ffi_sets: Vec<*mut ffi::SpvReflectDescriptorSet> =
                    vec![::std::ptr::null_mut(); count as usize];
                let result = unsafe {
                    let mut out_count: u32 = count;
                    ffi::spvReflectEnumerateDescriptorSets(
                        module,
                        &mut out_count,
                        ffi_sets.as_mut_ptr(),
                    )
                };
                match result {
                    ffi::SpvReflectResult_SPV_REFLECT_RESULT_SUCCESS => {
                        let mut sets = Vec::new();
                        for ffi_set in ffi_sets {
                            let ffi_set_ref = unsafe { &*ffi_set };
                            let mut bindings: Vec<
                                types::ReflectDescriptorBinding,
                            > = Vec::with_capacity(ffi_set_ref.binding_count as usize);
                            let ffi_bindings = unsafe {
                                std::slice::from_raw_parts(
                                    ffi_set_ref.bindings,
                                    ffi_set_ref.binding_count as usize,
                                )
                            };
                            for ffi_binding in ffi_bindings {
                                bindings.push(convert::ffi_to_descriptor_binding(unsafe {
                                    &**ffi_binding
                                }));
                            }
                            sets.push(types::descriptor::ReflectDescriptorSet {
                                set: ffi_set_ref.set,
                                bindings,
                            });
                        }
                        Ok(sets)
                    }
                    _ => Err(convert::result_to_string(result)),
                }
            } else {
                // No descriptor sets
                Ok(Vec::new())
            }
        } else {
            // Invalid shader module
            Ok(Vec::new())
        }
    }

    pub fn get_entry_point_name(&self) -> String {
        match self.module {
            Some(module) => ffi_to_string(module.entry_point_name),
            None => String::new(),
        }
    }

    pub fn get_entry_point(&self, name: &str) -> Option<types::variable::ReflectEntryPoint> {
        Default::default()
    }
}

impl Drop for ShaderModule {
    fn drop(&mut self) {
        if let Some(mut module) = self.module {
            unsafe {
                ffi::spvReflectDestroyShaderModule(&mut module);
            }
        }
    }
}

pub fn create_shader_module(spv_data: &[u8]) -> Result<ShaderModule, &str> {
    let mut module: ffi::SpvReflectShaderModule = unsafe { std::mem::zeroed() };
    let result: ffi::SpvReflectResult = unsafe {
        ffi::spvReflectCreateShaderModule(
            spv_data.len(),
            spv_data.as_ptr() as *const std::os::raw::c_void,
            &mut module,
        )
    };
    match result {
        ffi::SpvReflectResult_SPV_REFLECT_RESULT_SUCCESS => Ok(ShaderModule {
            module: Some(module),
        }),
        _ => Err(convert::result_to_string(result)),
    }
}

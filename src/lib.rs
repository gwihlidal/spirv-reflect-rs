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

    pub fn enumerate_input_variables(
        &self,
        entry_point: Option<&str>,
    ) -> Result<Vec<types::ReflectInterfaceVariable>, &str> {
        if let Some(ref module) = self.module {
            let mut count: u32 = 0;
            let result = unsafe {
                match entry_point {
                    Some(entry_point) => {
                        let entry_point_cstr = std::ffi::CString::new(entry_point).unwrap();
                        ffi::spvReflectEnumerateEntryPointInputVariables(
                            module,
                            entry_point_cstr.as_ptr(),
                            &mut count,
                            ::std::ptr::null_mut(),
                        )
                    }
                    None => ffi::spvReflectEnumerateInputVariables(
                        module,
                        &mut count,
                        ::std::ptr::null_mut(),
                    ),
                }
            };
            if result == ffi::SpvReflectResult_SPV_REFLECT_RESULT_SUCCESS && count > 0 {
                let mut ffi_vars: Vec<*mut ffi::SpvReflectInterfaceVariable> =
                    vec![::std::ptr::null_mut(); count as usize];
                let result = unsafe {
                    let mut out_count: u32 = count;
                    match entry_point {
                        Some(entry_point) => {
                            let entry_point_cstr = std::ffi::CString::new(entry_point).unwrap();
                            ffi::spvReflectEnumerateEntryPointInputVariables(
                                module,
                                entry_point_cstr.as_ptr(),
                                &mut out_count,
                                ffi_vars.as_mut_ptr(),
                            )
                        }
                        None => ffi::spvReflectEnumerateInputVariables(
                            module,
                            &mut out_count,
                            ffi_vars.as_mut_ptr(),
                        ),
                    }
                };
                match result {
                    ffi::SpvReflectResult_SPV_REFLECT_RESULT_SUCCESS => {
                        let vars: Vec<types::ReflectInterfaceVariable> = ffi_vars
                            .iter()
                            .map(|&var| convert::ffi_to_interface_variable(var))
                            .collect();
                        Ok(vars)
                    }
                    _ => Err(convert::result_to_string(result)),
                }
            } else {
                Ok(Vec::new())
            }
        } else {
            Ok(Vec::new())
        }
    }

    pub fn enumerate_output_variables(
        &self,
        entry_point: Option<&str>,
    ) -> Result<Vec<types::ReflectInterfaceVariable>, &str> {
        if let Some(ref module) = self.module {
            let mut count: u32 = 0;
            let result = unsafe {
                match entry_point {
                    Some(entry_point) => {
                        let entry_point_cstr = std::ffi::CString::new(entry_point).unwrap();
                        ffi::spvReflectEnumerateEntryPointOutputVariables(
                            module,
                            entry_point_cstr.as_ptr(),
                            &mut count,
                            ::std::ptr::null_mut(),
                        )
                    }
                    None => ffi::spvReflectEnumerateOutputVariables(
                        module,
                        &mut count,
                        ::std::ptr::null_mut(),
                    ),
                }
            };
            if result == ffi::SpvReflectResult_SPV_REFLECT_RESULT_SUCCESS && count > 0 {
                let mut ffi_vars: Vec<*mut ffi::SpvReflectInterfaceVariable> =
                    vec![::std::ptr::null_mut(); count as usize];
                let result = unsafe {
                    let mut out_count: u32 = count;
                    match entry_point {
                        Some(entry_point) => {
                            let entry_point_cstr = std::ffi::CString::new(entry_point).unwrap();
                            ffi::spvReflectEnumerateEntryPointOutputVariables(
                                module,
                                entry_point_cstr.as_ptr(),
                                &mut out_count,
                                ffi_vars.as_mut_ptr(),
                            )
                        }
                        None => ffi::spvReflectEnumerateOutputVariables(
                            module,
                            &mut out_count,
                            ffi_vars.as_mut_ptr(),
                        ),
                    }
                };
                match result {
                    ffi::SpvReflectResult_SPV_REFLECT_RESULT_SUCCESS => {
                        let vars: Vec<types::ReflectInterfaceVariable> = ffi_vars
                            .iter()
                            .map(|&var| convert::ffi_to_interface_variable(unsafe { &*var }))
                            .collect();
                        Ok(vars)
                    }
                    _ => Err(convert::result_to_string(result)),
                }
            } else {
                Ok(Vec::new())
            }
        } else {
            Ok(Vec::new())
        }
    }

    pub fn enumerate_descriptor_bindings(
        &self,
        entry_point: Option<&str>,
    ) -> Result<Vec<types::ReflectDescriptorBinding>, &str> {
        if let Some(ref module) = self.module {
            let mut count: u32 = 0;
            let result = unsafe {
                match entry_point {
                    Some(entry_point) => {
                        let entry_point_cstr = std::ffi::CString::new(entry_point).unwrap();
                        ffi::spvReflectEnumerateEntryPointDescriptorBindings(
                            module,
                            entry_point_cstr.as_ptr(),
                            &mut count,
                            ::std::ptr::null_mut(),
                        )
                    }
                    None => ffi::spvReflectEnumerateDescriptorBindings(
                        module,
                        &mut count,
                        ::std::ptr::null_mut(),
                    ),
                }
            };
            if result == ffi::SpvReflectResult_SPV_REFLECT_RESULT_SUCCESS && count > 0 {
                let mut ffi_bindings: Vec<*mut ffi::SpvReflectDescriptorBinding> =
                    vec![::std::ptr::null_mut(); count as usize];
                let result = unsafe {
                    let mut out_count: u32 = count;
                    match entry_point {
                        Some(entry_point) => {
                            let entry_point_cstr = std::ffi::CString::new(entry_point).unwrap();
                            ffi::spvReflectEnumerateEntryPointDescriptorBindings(
                                module,
                                entry_point_cstr.as_ptr(),
                                &mut out_count,
                                ffi_bindings.as_mut_ptr(),
                            )
                        }
                        None => ffi::spvReflectEnumerateDescriptorBindings(
                            module,
                            &mut out_count,
                            ffi_bindings.as_mut_ptr(),
                        ),
                    }
                };
                match result {
                    ffi::SpvReflectResult_SPV_REFLECT_RESULT_SUCCESS => {
                        let bindings: Vec<types::ReflectDescriptorBinding> = ffi_bindings
                            .iter()
                            .map(|&binding| {
                                convert::ffi_to_descriptor_binding(unsafe { &*binding })
                            }).collect();
                        Ok(bindings)
                    }
                    _ => Err(convert::result_to_string(result)),
                }
            } else {
                Ok(Vec::new())
            }
        } else {
            Ok(Vec::new())
        }
    }

    pub fn enumerate_descriptor_sets(
        &self,
        entry_point: Option<&str>,
    ) -> Result<Vec<types::ReflectDescriptorSet>, &str> {
        if let Some(ref module) = self.module {
            let mut count: u32 = 0;
            let result = unsafe {
                match entry_point {
                    Some(entry_point) => {
                        let entry_point_cstr = std::ffi::CString::new(entry_point).unwrap();
                        ffi::spvReflectEnumerateEntryPointDescriptorSets(
                            module,
                            entry_point_cstr.as_ptr(),
                            &mut count,
                            ::std::ptr::null_mut(),
                        )
                    }
                    None => ffi::spvReflectEnumerateDescriptorSets(
                        module,
                        &mut count,
                        ::std::ptr::null_mut(),
                    ),
                }
            };
            if result == ffi::SpvReflectResult_SPV_REFLECT_RESULT_SUCCESS && count > 0 {
                let mut ffi_sets: Vec<*mut ffi::SpvReflectDescriptorSet> =
                    vec![::std::ptr::null_mut(); count as usize];
                let result = unsafe {
                    let mut out_count: u32 = count;
                    match entry_point {
                        Some(entry_point) => {
                            let entry_point_cstr = std::ffi::CString::new(entry_point).unwrap();
                            ffi::spvReflectEnumerateEntryPointDescriptorSets(
                                module,
                                entry_point_cstr.as_ptr(),
                                &mut out_count,
                                ffi_sets.as_mut_ptr(),
                            )
                        }
                        None => ffi::spvReflectEnumerateDescriptorSets(
                            module,
                            &mut out_count,
                            ffi_sets.as_mut_ptr(),
                        ),
                    }
                };
                match result {
                    ffi::SpvReflectResult_SPV_REFLECT_RESULT_SUCCESS => Ok(ffi_sets
                        .iter()
                        .map(|&set| convert::ffi_to_descriptor_set(unsafe { &*set }))
                        .collect()),
                    _ => Err(convert::result_to_string(result)),
                }
            } else {
                Ok(Vec::new())
            }
        } else {
            Ok(Vec::new())
        }
    }

    pub fn enumerate_push_constant_blocks(
        &self,
        entry_point: Option<&str>,
    ) -> Result<Vec<types::ReflectBlockVariable>, &str> {
        if let Some(ref module) = self.module {
            let mut count: u32 = 0;
            let result = unsafe {
                match entry_point {
                    Some(entry_point) => {
                        let entry_point_cstr = std::ffi::CString::new(entry_point).unwrap();
                        ffi::spvReflectEnumerateEntryPointPushConstantBlocks(
                            module,
                            entry_point_cstr.as_ptr(),
                            &mut count,
                            ::std::ptr::null_mut(),
                        )
                    }
                    None => ffi::spvReflectEnumeratePushConstantBlocks(
                        module,
                        &mut count,
                        ::std::ptr::null_mut(),
                    ),
                }
            };
            if result == ffi::SpvReflectResult_SPV_REFLECT_RESULT_SUCCESS && count > 0 {
                let mut ffi_blocks: Vec<*mut ffi::SpvReflectBlockVariable> =
                    vec![::std::ptr::null_mut(); count as usize];
                let result = unsafe {
                    let mut out_count: u32 = count;
                    match entry_point {
                        Some(entry_point) => {
                            let entry_point_cstr = std::ffi::CString::new(entry_point).unwrap();
                            ffi::spvReflectEnumerateEntryPointPushConstantBlocks(
                                module,
                                entry_point_cstr.as_ptr(),
                                &mut out_count,
                                ffi_blocks.as_mut_ptr(),
                            )
                        }
                        None => ffi::spvReflectEnumeratePushConstantBlocks(
                            module,
                            &mut out_count,
                            ffi_blocks.as_mut_ptr(),
                        ),
                    }
                };
                match result {
                    ffi::SpvReflectResult_SPV_REFLECT_RESULT_SUCCESS => {
                        let blocks: Vec<types::ReflectBlockVariable> = ffi_blocks
                            .iter()
                            .map(|&block| convert::ffi_to_block_variable(unsafe { &*block }))
                            .collect();
                        Ok(blocks)
                    }
                    _ => Err(convert::result_to_string(result)),
                }
            } else {
                Ok(Vec::new())
            }
        } else {
            Ok(Vec::new())
        }
    }

    pub fn change_descriptor_binding_numbers(
        &mut self,
        _binding: types::descriptor::ReflectDescriptorBinding,
        _new_binding: u32,
        _new_set: Option<u32>,
    ) -> Result<(), &str> {
        Ok(())
    }

    pub fn change_descriptor_set_number(
        &mut self,
        _set: types::descriptor::ReflectDescriptorSet,
        _new_set: u32,
    ) -> Result<(), &str> {
        Ok(())
    }

    pub fn change_input_variable_location(
        &mut self,
        variable: &types::variable::ReflectInterfaceVariable,
        new_location: u32,
    ) -> Result<(), &str> {
        match self.module {
            Some(mut module) => {
                let result = unsafe {
                    ffi::spvReflectChangeInputVariableLocation(
                        &mut module as *mut ffi::SpvReflectShaderModule,
                        variable.internal_data,
                        new_location,
                    )
                };
                match result {
                    ffi::SpvReflectResult_SPV_REFLECT_RESULT_SUCCESS => Ok(()),
                    _ => Err(convert::result_to_string(result)),
                }
            }
            None => Ok(()),
        }
    }

    pub fn change_output_variable_location(
        &mut self,
        variable: &types::variable::ReflectInterfaceVariable,
        new_location: u32,
    ) -> Result<(), &str> {
        match self.module {
            Some(mut module) => {
                let result = unsafe {
                    ffi::spvReflectChangeOutputVariableLocation(
                        &mut module as *mut ffi::SpvReflectShaderModule,
                        variable.internal_data,
                        new_location,
                    )
                };
                match result {
                    ffi::SpvReflectResult_SPV_REFLECT_RESULT_SUCCESS => Ok(()),
                    _ => Err(convert::result_to_string(result)),
                }
            }
            None => Ok(()),
        }
    }

    pub fn get_entry_point_name(&self) -> String {
        match self.module {
            Some(module) => ffi_to_string(module.entry_point_name),
            None => String::new(),
        }
    }

    pub fn enumerate_entry_points(&self) -> Result<Vec<types::ReflectEntryPoint>, &str> {
        if let Some(ref module) = self.module {
            let ffi_entry_points = unsafe {
                std::slice::from_raw_parts(module.entry_points, module.entry_point_count as usize)
            };
            let entry_points: Vec<types::ReflectEntryPoint> = ffi_entry_points
                .iter()
                .map(|&entry_point| convert::ffi_to_entry_point(&entry_point))
                .collect();
            Ok(entry_points)
        } else {
            Ok(Vec::new())
        }
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

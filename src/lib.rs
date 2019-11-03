#[macro_use]
extern crate bitflags;
extern crate num_traits;
extern crate spirv_headers;
#[macro_use]
extern crate serde_derive;

use num_traits::cast::FromPrimitive;

pub mod convert;
pub mod ffi;
pub mod types;

pub(crate) fn ffi_to_string(ffi: *const ::std::os::raw::c_char) -> String {
    if ffi.is_null() {
        String::new()
    } else {
        unsafe { std::ffi::CStr::from_ptr(ffi).to_string_lossy().into_owned() }
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
    pub fn load_u8_data(spv_data: &[u8]) -> Result<ShaderModule, &'static str> {
        Ok(create_shader_module(spv_data)?)
    }

    pub fn load_u32_data(spv_data: &[u32]) -> Result<ShaderModule, &'static str> {
        let u8_data: &[u8] = unsafe {
            std::slice::from_raw_parts(
                spv_data.as_ptr() as *const u8,
                spv_data.len() * std::mem::size_of::<u32>(),
            )
        };
        Ok(create_shader_module(u8_data)?)
    }

    pub fn get_code(&self) -> Vec<u32> {
        match self.module {
            Some(ref module) => {
                let code_size = unsafe { ffi::spvReflectGetCodeSize(module) as usize };
                let code_slice = unsafe {
                    std::slice::from_raw_parts(ffi::spvReflectGetCode(module), code_size / 4)
                };
                code_slice.to_owned()
            }
            None => Vec::new(),
        }
    }

    pub fn get_generator(&self) -> types::ReflectGenerator {
        match self.module {
            Some(ref module) => convert::ffi_to_generator(module.generator),
            None => types::ReflectGenerator::Unknown,
        }
    }

    pub fn get_shader_stage(&self) -> types::ReflectShaderStageFlags {
        match self.module {
            Some(ref module) => convert::ffi_to_shader_stage_flags(module.shader_stage),
            None => types::ReflectShaderStageFlags::UNDEFINED,
        }
    }

    pub fn get_source_language(&self) -> spirv_headers::SourceLanguage {
        match self.module {
            Some(ref module) => {
                match spirv_headers::SourceLanguage::from_u32(module.source_language as u32) {
                    Some(language) => language,
                    None => spirv_headers::SourceLanguage::Unknown,
                }
            }
            None => spirv_headers::SourceLanguage::Unknown,
        }
    }

    pub fn get_source_language_version(&self) -> u32 {
        match self.module {
            Some(ref module) => module.source_language_version,
            None => 0,
        }
    }

    pub fn get_source_file(&self) -> String {
        match self.module {
            Some(ref module) => ffi_to_string(module.source_file),
            None => String::new(),
        }
    }

    pub fn get_source_text(&self) -> String {
        match self.module {
            Some(ref module) => ffi_to_string(module.source_source),
            None => String::new(),
        }
    }

    pub fn get_spirv_execution_model(&self) -> spirv_headers::ExecutionModel {
        match self.module {
            Some(ref module) => {
                match spirv_headers::ExecutionModel::from_u32(module.spirv_execution_model as u32) {
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
    ) -> Result<Vec<types::ReflectInterfaceVariable>, &'static str> {
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
    ) -> Result<Vec<types::ReflectInterfaceVariable>, &'static str> {
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

    pub fn enumerate_descriptor_bindings(
        &self,
        entry_point: Option<&str>,
    ) -> Result<Vec<types::ReflectDescriptorBinding>, &'static str> {
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
                            .map(|&binding| convert::ffi_to_descriptor_binding(binding))
                            .collect();
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
    ) -> Result<Vec<types::ReflectDescriptorSet>, &'static str> {
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
                        .map(|&set| convert::ffi_to_descriptor_set(set))
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
    ) -> Result<Vec<types::ReflectBlockVariable>, &'static str> {
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

    pub fn enumerate_entry_points(&self) -> Result<Vec<types::ReflectEntryPoint>, &'static str> {
        if let Some(ref module) = self.module {
            let ffi_entry_points = unsafe {
                std::slice::from_raw_parts(module.entry_points, module.entry_point_count as usize)
            };
            let entry_points: Vec<types::ReflectEntryPoint> = ffi_entry_points
                .iter()
                .map(|entry_point| convert::ffi_to_entry_point(entry_point))
                .collect();
            Ok(entry_points)
        } else {
            Ok(Vec::new())
        }
    }

    pub fn get_entry_point_name(&self) -> String {
        match self.module {
            Some(ref module) => ffi_to_string(module.entry_point_name),
            None => String::new(),
        }
    }

    pub fn change_descriptor_binding_numbers(
        &mut self,
        binding: &types::descriptor::ReflectDescriptorBinding,
        new_binding: u32,
        new_set: Option<u32>,
    ) -> Result<(), &'static str> {
        match self.module {
            Some(ref mut module) => {
                let new_set = new_set.unwrap_or(ffi::SPV_REFLECT_SET_NUMBER_DONT_CHANGE as u32);
                let result = unsafe {
                    ffi::spvReflectChangeDescriptorBindingNumbers(
                        module as *mut ffi::SpvReflectShaderModule,
                        binding.internal_data,
                        new_binding,
                        new_set,
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

    pub fn change_descriptor_set_number(
        &mut self,
        set: &types::descriptor::ReflectDescriptorSet,
        new_set: u32,
    ) -> Result<(), &'static str> {
        match self.module {
            Some(ref mut module) => {
                let result = unsafe {
                    ffi::spvReflectChangeDescriptorSetNumber(
                        module as *mut ffi::SpvReflectShaderModule,
                        set.internal_data,
                        new_set,
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

    pub fn change_input_variable_location(
        &mut self,
        variable: &types::variable::ReflectInterfaceVariable,
        new_location: u32,
    ) -> Result<(), &'static str> {
        match self.module {
            Some(ref mut module) => {
                let result = unsafe {
                    ffi::spvReflectChangeInputVariableLocation(
                        module as *mut ffi::SpvReflectShaderModule,
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
    ) -> Result<(), &'static str> {
        match self.module {
            Some(ref mut module) => {
                let result = unsafe {
                    ffi::spvReflectChangeOutputVariableLocation(
                        module as *mut ffi::SpvReflectShaderModule,
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
}

impl Drop for ShaderModule {
    fn drop(&mut self) {
        if let Some(ref mut module) = self.module {
            unsafe {
                ffi::spvReflectDestroyShaderModule(module);
            }
        }
    }
}

/*
impl From<&[u8]> for ShaderModule {
    fn from(spv_data: &[u8]) -> Result<ShaderModule, &str> {
        create_shader_module(spv_data)?
    }
}
*/

/*impl<'a, T: AsRef<[u8]>> From<T> for ShaderModule {
    fn from(v: T) -> Result<ShaderModule, &'static str> {
        Ok(create_shader_module(v.as_ref())?)
    }
}*/

pub fn create_shader_module(spv_data: &[u8]) -> Result<ShaderModule, &'static str> {
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

extern crate spirv_reflect;

use spirv_reflect::*;

fn main() {
    println!("Testing");

    let spv_data = include_bytes!("./sample.spv");

    match create_shader_module(spv_data) {
        Ok(mut module) => {
            let entry_point_name = module.get_entry_point_name();
            println!("entry point name: {}", entry_point_name);

            let generator = module.get_generator();
            println!("generator: {:?}", generator);

            let shader_stage = module.get_shader_stage();
            println!("shader_stage: {:?}", shader_stage);

            let source_lang = module.get_source_language();
            println!("source_lang: {:?}", source_lang);

            let source_lang_ver = module.get_source_language_version();
            println!("source_lang_ver: {}", source_lang_ver);

            let source_file = module.get_source_file();
            println!("source_file: {}", source_file);

            let source_text = module.get_source_text();
            println!("source_text: {}", source_text);

            let spv_execution_model = module.get_spirv_execution_model();
            println!("spv_execution_model: {:?}", spv_execution_model);

            //let code_size = module.get_code_size();
            //let code_slice = module.get_code_slice();
            //println!("size is {}", code_size);
            //println!("slice is {:?}", code_slice);

            let input_vars = module.enumerate_input_variables(None).unwrap();
            println!("");
            //println!("");
            //println!("");
            //println!("input variables {:?}", input_vars);
            for var in &input_vars {
                println!("input var - name: {} location: {}", var.name, var.location);
                if var.name == "input.Alpha" {
                    module.change_input_variable_location(&var, 8).unwrap();
                }
            }

            let _output_vars = module.enumerate_output_variables(None).unwrap();
            println!("");
            //println!("");
            //println!("");
            //println!("output variables {:?}", output_vars);

            let _bindings = module.enumerate_descriptor_bindings(None).unwrap();
            println!("");
            //println!("");
            //println!("");
            //println!("descriptor bindings {:?}", bindings);

            let _sets = module.enumerate_descriptor_sets(None).unwrap();
            println!("");
            //println!("");
            //println!("");
            //println!("descriptor sets {:?}", sets);

            let input_vars2 = module.enumerate_input_variables(None).unwrap();
            println!("");
            //println!("");
            //println!("");
            //println!("input variables {:?}", input_vars);
            for var in &input_vars2 {
                println!("input var2 - name: {} location: {}", var.name, var.location);
            }
        }
        Err(err) => {
            panic!("Error occurred - {:?}", err);
        }
    }
}

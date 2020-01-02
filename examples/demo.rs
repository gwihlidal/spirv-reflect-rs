extern crate spirv_reflect;
use spirv_reflect::*;

fn main() {
    let spv_data = include_bytes!("./sample.spv");

    match ShaderModule::load_u8_data(spv_data) {
        Ok(ref mut module) => {
            let _entry_point_name = module.get_entry_point_name();
            let _generator = module.get_generator();
            let _shader_stage = module.get_shader_stage();
            let _source_lang = module.get_source_language();
            let _source_lang_ver = module.get_source_language_version();
            let _source_file = module.get_source_file();
            let _source_text = module.get_source_text();
            let _spv_execution_model = module.get_spirv_execution_model();
            let _output_vars = module.enumerate_output_variables(None).unwrap();
            let _bindings = module.enumerate_descriptor_bindings(None).unwrap();
            let _sets = module.enumerate_descriptor_sets(None).unwrap();

            println!("Original input variables (unmodified)");
            let input_vars = module.enumerate_input_variables(None).unwrap();
            for var in &input_vars {
                println!(
                    "   input var - name: {} location: {}",
                    var.name, var.location
                );
                if var.name == "input.Alpha" {
                    // Change alpha input variable location from 2 to 8
                    module.change_input_variable_location(&var, 8).unwrap();
                }
            }

            println!("Modified input variables (alpha location is now 8)");
            let input_vars2 = module.enumerate_input_variables(None).unwrap();
            for var in &input_vars2 {
                println!(
                    "   input var - name: {} location: {}",
                    var.name, var.location
                );
            }

            println!("Entry points (yaml)");
            let entry_points = module.enumerate_entry_points().unwrap();
            for entry_point in &entry_points {
                let output = serde_yaml::to_string(&entry_point).unwrap();
                println!("{}", output);
            }

            let _code = module.get_code();
        }
        Err(err) => {
            panic!("Error occurred - {:?}", err);
        }
    }
}

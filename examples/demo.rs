extern crate spirv_reflect;

use spirv_reflect::*;

fn main() {
    println!("Testing");

    let spv_data = include_bytes!("./sample.spv");

    match create_shader_module(spv_data) {
        Ok(module) => {
            let code_size = module.code_size();
            let code_slice = module.code_slice();
            println!("size is {}", code_size);
            //println!("slice is {:?}", code_slice);
            let count = module.descriptor_set_count().unwrap();
            println!("descriptor count is {}", count);

            let sets = module.descriptor_sets().unwrap();
            println!("descriptor sets {:?}", sets);
        }
        Err(err) => {
            panic!("Error occurred - {:?}", err);
        }
    }
}

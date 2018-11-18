extern crate cc;

use std::env;

fn main() {
    let mut build = cc::Build::new();

    build.include("src");

    // Add the files we build
    let source_files = [
        "vendor/spirv_reflect.c",
    ];

    for source_file in &source_files {
        build.file(&source_file);
    }

    let target = env::var("TARGET").unwrap();
    if target.contains("darwin") {
        build
            .flag("-std=c++11")
            .cpp_link_stdlib("c++")
            .cpp_set_stdlib("c++")
            .cpp(true);
    } else if target.contains("linux") {
        build.flag("-std=c++11").cpp_link_stdlib("stdc++").cpp(true);
    }

    build.compile("spirv_reflect_cpp");
}

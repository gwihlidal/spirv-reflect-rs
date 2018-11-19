pub mod image;
pub mod descriptor;
pub mod resource;
pub mod variable;
pub mod op;

pub use self::image::*;
pub use self::descriptor::*;
pub use self::resource::*;
pub use self::variable::*;
pub use self::op::*;

use ffi;

#[derive(Debug, Copy, Clone)]
pub enum ReflectGenerator {
    UNKNOWN,
    KHRONOS_LLVM_SPIRV_TRANSLATOR,
    KHRONOS_SPIRV_TOOLS_ASSEMBLER,
    KHRONOS_GLSLANG_REFERENCE_FRONT_END,
    GOOGLE_SHADERC_OVER_GLSLANG,
    GOOGLE_SPIREGG,
    GOOGLE_RSPIRV,
    X_LEGEND_MESA_MESAIR_SPIRV_TRANSLATOR,
    KHRONOS_SPIRV_TOOLS_LINKER,
    WINE_VKD3D_SHADER_COMPILER,
    CLAY_CLAY_SHADER_COMPILER,
}

impl Default for ReflectGenerator {
    fn default() -> Self {
        ReflectGenerator::UNKNOWN
    }
}

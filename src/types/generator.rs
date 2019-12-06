#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
pub enum ReflectGenerator {
    Unknown,
    KhronosLlvmSpirvTranslator,
    KhronosSpirvToolsAssembler,
    KhronosGlslangReferenceFrontEnd,
    GoogleShadercOverGlslang,
    GoogleSpiregg,
    GoogleRspirv,
    XLegendMesaMesairSpirvTranslator,
    KhronosSpirvToolsLinker,
    WineVkd3dShaderCompiler,
    ClayClayShaderCompiler,
}
/*
0 => "The Khronos Group",
1 => "LunarG",
2 => "Valve",
3 => "Codeplay",
4 => "NVIDIA",
5 => "ARM",
6 => "LLVM/SPIR-V Translator",
7 => "SPIR-V Tools Assembler",
8 => "Glslang",
9 => "Qualcomm",
10 => "AMD",
11 => "Intel",
12 => "Imagination",
13 => "Shaderc",
14 => "spiregg",
15 => "rspirv",
_ => "Unknown",
*/

impl Default for ReflectGenerator {
    fn default() -> Self {
        ReflectGenerator::Unknown
    }
}

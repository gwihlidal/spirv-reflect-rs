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

impl Default for ReflectGenerator {
    fn default() -> Self {
        ReflectGenerator::Unknown
    }
}

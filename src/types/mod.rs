pub mod descriptor;
pub mod image;
pub mod op;
pub mod resource;
pub mod traits;
pub mod variable;

pub use self::descriptor::*;
pub use self::image::*;
pub use self::op::*;
pub use self::resource::*;
pub use self::traits::*;
pub use self::variable::*;

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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

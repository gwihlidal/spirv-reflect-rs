use spirv_headers;
use std::ops::Deref;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ReflectOp(pub(crate) spirv_headers::Op);

impl Default for ReflectOp {
    fn default() -> Self {
        ReflectOp(spirv_headers::Op::Nop)
    }
}

impl Deref for ReflectOp {
    type Target = spirv_headers::Op;
    fn deref(&self) -> &spirv_headers::Op {
        &self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ReflectBuiltIn(pub(crate) spirv_headers::BuiltIn);

impl Default for ReflectBuiltIn {
    fn default() -> Self {
        ReflectBuiltIn(spirv_headers::BuiltIn::Position)
    }
}

impl Deref for ReflectBuiltIn {
    type Target = spirv_headers::BuiltIn;
    fn deref(&self) -> &spirv_headers::BuiltIn {
        &self.0
    }
}

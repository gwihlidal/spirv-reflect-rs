use crate::ffi;
use num_traits::cast::FromPrimitive;
use spirv_headers;
use std::ops::Deref;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ReflectOp(spirv_headers::Op);

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

impl From<ffi::SpvOp> for ReflectOp {
    fn from(raw_op: ffi::SpvOp) -> Self {
        match spirv_headers::Op::from_u32(raw_op as u32) {
            Some(op) => ReflectOp(op),
            None => Default::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ReflectBuiltIn(spirv_headers::BuiltIn);

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

impl From<ffi::SpvBuiltIn> for ReflectBuiltIn {
    fn from(raw_built_in: ffi::SpvBuiltIn) -> Self {
        match spirv_headers::BuiltIn::from_u32(raw_built_in as u32) {
            Some(built_in) => ReflectBuiltIn(built_in),
            None => Default::default(),
        }
    }
}

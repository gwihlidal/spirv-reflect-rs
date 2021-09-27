use crate::ffi;
use num_traits::cast::FromPrimitive;
use spirv;
use std::ops::Deref;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ReflectOp(spirv::Op);

impl Default for ReflectOp {
    fn default() -> Self {
        ReflectOp(spirv::Op::Nop)
    }
}

impl Deref for ReflectOp {
    type Target = spirv::Op;
    fn deref(&self) -> &spirv::Op {
        &self.0
    }
}

impl From<ffi::SpvOp> for ReflectOp {
    fn from(raw_op: ffi::SpvOp) -> Self {
        match spirv::Op::from_u32(raw_op as u32) {
            Some(op) => ReflectOp(op),
            None => Default::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ReflectBuiltIn(spirv::BuiltIn);

impl Default for ReflectBuiltIn {
    fn default() -> Self {
        ReflectBuiltIn(spirv::BuiltIn::Position)
    }
}

impl Deref for ReflectBuiltIn {
    type Target = spirv::BuiltIn;
    fn deref(&self) -> &spirv::BuiltIn {
        &self.0
    }
}

impl From<ffi::SpvBuiltIn> for ReflectBuiltIn {
    fn from(raw_built_in: ffi::SpvBuiltIn) -> Self {
        match spirv::BuiltIn::from_u32(raw_built_in as u32) {
            Some(built_in) => ReflectBuiltIn(built_in),
            None => Default::default(),
        }
    }
}

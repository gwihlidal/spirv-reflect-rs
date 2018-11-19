use ffi;
use num_traits::cast::FromPrimitive;
use spirv_headers;

#[derive(Debug, Clone)]
pub struct ReflectOp(spirv_headers::Op);

impl Default for ReflectOp {
    fn default() -> Self {
        ReflectOp(spirv_headers::Op::Nop)
    }
}

impl From<ffi::SpvOp> for ReflectOp {
    fn from(raw_op: ffi::SpvOp) -> Self {
        match spirv_headers::Op::from_i32(raw_op) {
            Some(op) => ReflectOp(op),
            None => Default::default(),
        }
    }
}

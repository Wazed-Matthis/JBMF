use java_rs_base::constant_pool::ConstantPoolIndex;
use java_rs_derive::ClassFilePart;

use crate::flags::AccessFlags;

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct MethodParameter {
    name: ConstantPoolIndex,
    access_flags: AccessFlags,
}

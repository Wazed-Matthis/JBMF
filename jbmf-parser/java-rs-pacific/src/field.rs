use java_rs_base::io::SizedVec;
use java_rs_derive::ClassFilePart;

use crate::flags::AccessFlags;
use crate::{Attribute, ConstantPoolIndex};

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct Field {
    pub access_flags: AccessFlags,
    pub name: ConstantPoolIndex,
    pub descriptor: ConstantPoolIndex,
    pub attributes: SizedVec<u16, Attribute>,
}

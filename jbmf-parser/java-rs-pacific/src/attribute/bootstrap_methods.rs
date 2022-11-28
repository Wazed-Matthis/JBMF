use java_rs_base::constant_pool::ConstantPoolIndex;
use java_rs_base::io::SizedVec;
use java_rs_derive::ClassFilePart;

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct BootstrapMethod {
    method_ref: ConstantPoolIndex,
    arguments: SizedVec<u16, ConstantPoolIndex>,
}

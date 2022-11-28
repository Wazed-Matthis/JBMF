use java_rs_base::constant_pool::ConstantPoolIndex;
use java_rs_derive::ClassFilePart;

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct LocalVariableTypeTable {
    start_pc: u16,
    length: u16,
    name: ConstantPoolIndex,
    signature: ConstantPoolIndex,
    index: u16,
}

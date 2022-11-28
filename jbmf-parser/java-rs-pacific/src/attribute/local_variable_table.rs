use java_rs_base::constant_pool::ConstantPoolIndex;
use java_rs_derive::ClassFilePart;

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct LocalVariableTable {
    pub start_pc: u16,
    pub length: u16,
    pub name: ConstantPoolIndex,
    pub descriptor: ConstantPoolIndex,
    pub index: u16,
}

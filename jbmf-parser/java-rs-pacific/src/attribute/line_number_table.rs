use java_rs_derive::ClassFilePart;

#[derive(Debug, ClassFilePart, Clone, Eq, PartialEq)]
pub struct LineNumberTable {
    start_pc: u16,
    line_number: u16,
}

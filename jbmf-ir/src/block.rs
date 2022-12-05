use crate::statement::Statement;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct BasicBlock {
    pub beg_index: u64,
    pub statements: Vec<Statement>,
}

use crate::statement::Statement;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct BasicBlock {
    pub ident: i64,
    pub statements: Vec<Statement>,
}

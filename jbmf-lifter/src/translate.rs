use jbmf_ir::statement::{Statement, StatementKind};
use jbmf_parser::java_rs_pacific::attribute::Instruction;

pub trait Translate {
    fn translate(instruction: Instruction) -> Self
    where
        Self: Sized;
}

impl Translate for Statement {
    fn translate(instruction: Instruction) -> Self
    where
        Self: Sized,
    {
        Self(Box::new(StatementKind::Field))
    }
}

impl Translate for StatementKind {
    fn translate(instruction: Instruction) -> Self
    where
        Self: Sized,
    {
        match instruction {
            _ => StatementKind::Field,
        }
    }
}

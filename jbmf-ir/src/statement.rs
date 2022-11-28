use crate::block::BasicBlock;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct Statement(pub Box<StatementKind>);

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum StatementKind {
    Arithmetic(ArithmeticStatementKind),
    Flow(FlowStatementKind),
    Field,
    Variable,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum FlowStatementKind {
    MethodCall,
    UnconditionalJump(BasicBlock),
    ConditionalJump {
        condition: Statement,
        target: BasicBlock,
    },
    Return(BasicBlock),
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum ArithmeticStatementKind {
    Unary {
        rhs: Statement,
        operation: UnaryOperation,
    },
    Binary {
        lhs: Statement,
        rhs: Statement,
        operation: BinaryOperation,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum UnaryOperation {
    LogicalNegate,
    ArithmeticNegate,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum BinaryOperation {
    Addition,
    Multiplication,
    Subtraction,
    Modulo,
    Division,
    LeftShift,
    RightShift,
    LeftShiftPadded,
    LOR,
    LAND,
    LXOR,
}

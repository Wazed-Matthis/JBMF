use crate::block::BasicBlock;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct Statement(pub Box<StatementKind>);

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum StatementKind {
    Arithmetic(ArithmeticStatementKind),
    Flow(FlowStatementKind),
    Field,
    Variable(u32, TypeSignature),
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

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum TypeSignature {
    Integer,
    Boolean,
    Long,
    Short,
    Float,
    Double,
    Arbitrary,

    IntegerArray,
    BooleanArray,
    LongArray,
    ShortArray,
    FloatArray,
    DoubleArray,
    ArbitraryArray,
}

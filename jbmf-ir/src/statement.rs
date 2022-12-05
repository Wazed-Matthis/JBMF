use crate::block::BasicBlock;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct Statement(pub Box<StatementKind>);

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum StatementKind {
    Arithmetic(ArithmeticStatementKind),
    Flow(FlowStatementKind),
    Field(FieldStatementKind),
    Variable(u32, TypeSignature),
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum FieldStatementKind {
    Store(u16, TypeSignature),
    Load(u16, TypeSignature),
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum FlowStatementKind {
    MethodCall,
    UnconditionalJump(u16),
    ConditionalJump { target: u16 },
    Return,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum ArithmeticStatementKind {
    Unary(UnaryOperation),
    Binary(BinaryOperation),
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
    RightShiftPadded,
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

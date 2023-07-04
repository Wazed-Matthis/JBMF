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
    MethodCall {
        owner: String,
        name: String,
        descriptor: String,
        arguments: Vec<TypeSignature>,
        return_type: TypeSignature,
    },
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
    Byte,
    Char,
    Integer,
    Boolean,
    Long,
    Short,
    Float,
    Double,

    Class(String),

    Arbitrary,

    Void,

    Array(Box<TypeSignature>),
}

impl From<String> for TypeSignature {
    fn from(descriptor: String) -> Self {
        let mut chars = descriptor.chars();
        match chars.next() {
            Some('Z') => TypeSignature::Boolean,
            Some('B') => TypeSignature::Byte,
            Some('C') => TypeSignature::Char,
            Some('S') => TypeSignature::Short,
            Some('I') => TypeSignature::Integer,
            Some('J') => TypeSignature::Long,
            Some('F') => TypeSignature::Float,
            Some('D') => TypeSignature::Double,
            Some('V') => TypeSignature::Void,
            Some('L') => {
                // Parse class name until ';' character
                let class_name: String = chars
                    .take_while(|&c| c != ';')
                    .collect();
                TypeSignature::Class(class_name)
            }
            Some('[') => {
                // Parse array type recursively
                let element_type = TypeSignature::from(descriptor[1..].to_string());
                TypeSignature::Array(Box::new(element_type))
            }
            Some(_) => TypeSignature::Arbitrary, // Handle unknown characters
            None => TypeSignature::Void, // Handle empty string (This might occur in this format: "()V")
        }
    }
}

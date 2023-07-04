use jbmf_ir::statement::{
    ArithmeticStatementKind, BinaryOperation, FieldStatementKind, FlowStatementKind, Statement,
    StatementKind, TypeSignature, UnaryOperation,
};
use jbmf_parser::java_rs_pacific::attribute::Instruction;
use jbmf_parser::java_rs_pacific::attribute::Instruction::{DLoad0, DLoad3};
use jbmf_parser::java_rs_pacific::{Constant, ConstantPool, ConstantPoolIndex};
use crate::extract_constant_fields;

pub trait Translate {
    fn translate(instructions: Instruction, cp: ConstantPool) -> Self
        where
            Self: Sized;
}

impl Translate for Statement {
    fn translate(instruction: Instruction, cp: ConstantPool) -> Self
        where
            Self: Sized,
    {
        Self(Box::new(StatementKind::translate(instruction, cp)))
    }
}

impl Translate for StatementKind {
    fn translate(instruction: Instruction, cp: ConstantPool) -> Self
        where
            Self: Sized,
    {
        match instruction {
            // Integer arithmetics
            Instruction::IAdd
            | Instruction::ISub
            | Instruction::IMul
            | Instruction::IDiv
            | Instruction::IRem
            | Instruction::IOr
            | Instruction::IXor
            | Instruction::IAnd
            | Instruction::INeg
            | Instruction::IShl
            | Instruction::IShr
            | Instruction::IUShr

            // Long arithmetics
            | Instruction::LAdd
            | Instruction::LSub
            | Instruction::LMul
            | Instruction::LDiv
            | Instruction::LRem
            | Instruction::LOr
            | Instruction::LXor
            | Instruction::LAnd
            | Instruction::LNeg
            | Instruction::LShl
            | Instruction::LShr
            | Instruction::LUShr

            // Float Arithmetics
            | Instruction::FAdd
            | Instruction::FSub
            | Instruction::FMul
            | Instruction::FDiv
            | Instruction::FRem

            // Double arithmetics
            | Instruction::DAdd
            | Instruction::DSub
            | Instruction::DMul
            | Instruction::DDiv
            | Instruction::DRem
            => StatementKind::Arithmetic(ArithmeticStatementKind::translate(
                instruction, cp
            )),

            // Conditionals
            Instruction::IfEq { .. }
            | Instruction::IfNe { .. }
            | Instruction::IfGe { .. }
            | Instruction::IfLe { .. }
            | Instruction::IfGt { .. }
            | Instruction::IfLt { .. }

            //Arbitrary
            | Instruction::IfACmpEq { .. }
            | Instruction::IfACmpNe { .. }

            //Integer
            | Instruction::IfICmpEq { .. }
            | Instruction::IfICmpNe { .. }
            | Instruction::IfICmpGe { .. }
            | Instruction::IfICmpLe { .. }
            | Instruction::IfICmpGt { .. }
            | Instruction::IfICmpLt { .. }

            //Null
            | Instruction::IfNonNull { .. }
            | Instruction::IfNull { .. } => StatementKind::Flow(FlowStatementKind::translate(
                instruction,
                cp
            )),

            Instruction::InvokeDynamic { .. }
            | Instruction::InvokeInterface { .. }
            | Instruction::InvokeSpecial { .. }
            | Instruction::InvokeStatic { .. }
            | Instruction::InvokeVirtual { .. }

            | Instruction::IReturn
            | Instruction::LReturn
            | Instruction::FReturn
            | Instruction::DReturn
            | Instruction::Ret { .. }
            | Instruction::Return
            | Instruction::AReturn
            => StatementKind::Flow(FlowStatementKind::translate(
                instruction,
                cp
            )),

            Instruction::ALoad { .. } => StatementKind::Field(FieldStatementKind::Load(0, TypeSignature::Arbitrary)),
            Instruction::ALoad0
            | Instruction::ALoad1
            | Instruction::ALoad2
            | Instruction::ALoad3

            | Instruction::ILoad0
            | Instruction::ILoad1
            | Instruction::ILoad2
            | Instruction::ILoad3

            | Instruction::LLoad0
            | Instruction::LLoad1
            | Instruction::LLoad2
            | Instruction::LLoad3

            | Instruction::FLoad0
            | Instruction::FLoad1
            | Instruction::FLoad2
            | Instruction::FLoad3

            | Instruction::DLoad0
            | Instruction::DLoad1
            | Instruction::DLoad2
            | Instruction::DLoad3 => StatementKind::Field(FieldStatementKind::Load(0, TypeSignature::Double)),

            _ => StatementKind::Field(FieldStatementKind::Load(0, TypeSignature::Arbitrary)),
        }
    }
}

impl Translate for BinaryOperation {
    fn translate(instruction: Instruction, cp: ConstantPool) -> Self
        where
            Self: Sized,
    {
        match instruction {
            Instruction::LAdd | Instruction::DAdd | Instruction::FAdd | Instruction::IAdd => {
                BinaryOperation::Addition
            }
            Instruction::IMul | Instruction::DMul | Instruction::FMul | Instruction::LMul => {
                BinaryOperation::Multiplication
            }
            Instruction::IDiv | Instruction::DDiv | Instruction::FDiv | Instruction::LDiv => {
                BinaryOperation::Division
            }
            Instruction::ISub | Instruction::DSub | Instruction::FSub | Instruction::LSub => {
                BinaryOperation::Subtraction
            }
            Instruction::IRem | Instruction::DRem | Instruction::FRem | Instruction::LRem => {
                BinaryOperation::Modulo
            }
            Instruction::IAnd | Instruction::LAnd => BinaryOperation::LAND,
            Instruction::IXor | Instruction::LXor => BinaryOperation::LXOR,
            Instruction::IOr | Instruction::LOr => BinaryOperation::LOR,
            Instruction::IShl | Instruction::LShl => BinaryOperation::LeftShift,
            Instruction::IUShr | Instruction::LUShr => BinaryOperation::RightShiftPadded,
            Instruction::IShr | Instruction::LShr => BinaryOperation::RightShift,
            _ => {
                unreachable!()
            }
        }
    }
}

impl Translate for ArithmeticStatementKind {
    fn translate(instruction: Instruction, cp: ConstantPool) -> Self
        where
            Self: Sized,
    {
        match instruction {
            Instruction::IAdd
            | Instruction::ISub
            | Instruction::IMul
            | Instruction::IDiv
            | Instruction::IRem
            | Instruction::IOr
            | Instruction::IXor
            | Instruction::IAnd
            | Instruction::IShl
            | Instruction::IShr
            | Instruction::IUShr

            // Long arithmetics
            | Instruction::LAdd
            | Instruction::LSub
            | Instruction::LMul
            | Instruction::LDiv
            | Instruction::LRem
            | Instruction::LOr
            | Instruction::LXor
            | Instruction::LAnd
            | Instruction::LShl
            | Instruction::LShr
            | Instruction::LUShr

            // Float Arithmetics
            | Instruction::FAdd
            | Instruction::FSub
            | Instruction::FMul
            | Instruction::FDiv
            | Instruction::FRem

            // Double arithmetics
            | Instruction::DAdd
            | Instruction::DSub
            | Instruction::DMul
            | Instruction::DDiv
            | Instruction::DRem => ArithmeticStatementKind::Binary(
                BinaryOperation::translate(instruction,cp),
            ),
            Instruction::INeg | Instruction::LNeg | Instruction::FNeg | Instruction::DNeg => ArithmeticStatementKind::Unary(
                UnaryOperation::translate(instruction, cp),
            ),
            _ => ArithmeticStatementKind::Unary(
                UnaryOperation::translate(instruction, cp),
            ),
        }
    }
}

impl Translate for FlowStatementKind {
    fn translate(instruction: Instruction, cp: ConstantPool) -> Self
        where
            Self: Sized,
    {
        match instruction {
            // Conditionals
            Instruction::IfEq { offset }
            | Instruction::IfNe { offset }
            | Instruction::IfGe { offset }
            | Instruction::IfLe { offset }
            | Instruction::IfGt { offset }
            | Instruction::IfLt { offset }

            //Arbitrary
            | Instruction::IfACmpEq { offset }
            | Instruction::IfACmpNe { offset }

            //Integer
            | Instruction::IfICmpEq { offset }
            | Instruction::IfICmpNe { offset }
            | Instruction::IfICmpGe { offset }
            | Instruction::IfICmpLe { offset }
            | Instruction::IfICmpGt { offset }
            | Instruction::IfICmpLt { offset }

            //Null
            | Instruction::IfNonNull { offset }
            | Instruction::IfNull { offset } => FlowStatementKind::ConditionalJump { target: offset },

            Instruction::InvokeDynamic { index, .. }
            | Instruction::InvokeInterface { index, .. }
            | Instruction::InvokeSpecial { index }
            | Instruction::InvokeStatic { index }
            | Instruction::InvokeVirtual { index } => {

                let (name_and_type_index, class_index) = extract_constant_fields!(cp, ConstantPoolIndex(index.0), Constant::MethodRef { name_and_type, class });
                let (name_index, descriptor_index) = extract_constant_fields!(cp, *name_and_type_index, Constant::NameAndType {name, descriptor});
                let name = extract_constant_fields!(cp, *name_index, Constant::Utf8 => (name)).clone();
                let descriptor = extract_constant_fields!(cp, *descriptor_index, Constant::Utf8 => (name)).clone();

                let class_name_index = extract_constant_fields!(cp, *class_index, Constant::Class => (name));
                let owner = extract_constant_fields!(cp, *class_name_index, Constant::Utf8 => (name)).clone();

                let (arguments, return_type) = parse_method_signature(&descriptor);

                FlowStatementKind::MethodCall {
                    owner,
                    name,
                    descriptor,
                    arguments,
                    return_type,
                }
            }

            Instruction::IReturn
            | Instruction::LReturn
            | Instruction::FReturn
            | Instruction::DReturn
            | Instruction::Ret { .. }
            | Instruction::Return
            | Instruction::AReturn
            => FlowStatementKind::Return,

            _ => unreachable!()
        }
    }
}

fn parse_method_signature(signature: &str) -> (Vec<TypeSignature>, TypeSignature) {
    let mut chars = signature.chars();
    let mut arguments = Vec::new();
    let mut return_type = None;
    let mut state = ParseState::Arguments;

    while let Some(ch) = chars.next() {
        match state {
            ParseState::Arguments => {
                if ch == ')' {
                    state = ParseState::ReturnType;
                } else {

                    let argument_type = TypeSignature::from(ch.to_string());
                    arguments.push(argument_type);
                }
            }
            ParseState::ReturnType => {
                let return_type_descriptor = format!("{}{}", ch, chars.collect::<String>());
                return_type = Some(TypeSignature::from(return_type_descriptor));
                break;
            }
        }
    }

    let return_type = return_type.unwrap_or(TypeSignature::Void);
    (arguments, return_type)
}

enum ParseState {
    Arguments,
    ReturnType,
}

impl Translate for UnaryOperation {
    fn translate(instruction: Instruction, cp: ConstantPool) -> Self
        where
            Self: Sized,
    {
        match instruction {
            Instruction::INeg | Instruction::FNeg | Instruction::DNeg | Instruction::LNeg => {
                UnaryOperation::ArithmeticNegate
            }
            _ => UnaryOperation::LogicalNegate,
        }
    }
}


impl Translate for TypeSignature {
    fn translate(instruction: Instruction, cp: ConstantPool) -> Self
        where
            Self: Sized,
    {
        TypeSignature::Arbitrary
    }
}

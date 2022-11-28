use jbmf_ir::block::BasicBlock;
use jbmf_ir::flow_graph::FlowGraph;
use jbmf_parser::java_rs_pacific::attribute::{Attribute, Compatibility, Instruction};
use jbmf_parser::java_rs_pacific::SizedVec;
use jbmf_parser::parse_class_file;
use std::collections::HashSet;

pub fn generate_flow_graph(
    instructions: &SizedVec<u32, Instruction>,
) -> FlowGraph<BasicBlock, (i16, i16)> {
    for (index, instruction) in instructions.iter().enumerate() {
        if is_flow_instruction(instruction) {
            println!("instruction: {instruction:?} is a flow instruction");
        }
    }

    FlowGraph {
        vertices: HashSet::new(),
        edges: vec![],
    }
}

pub fn is_flow_instruction(instruction: &Instruction) -> bool {
    match instruction {
        Instruction::AReturn
        | Instruction::AThrow
        | Instruction::DReturn
        | Instruction::FReturn
        | Instruction::Goto { .. }
        | Instruction::GotoW { .. }
        | Instruction::IfACmpEq { .. }
        | Instruction::IfACmpNe { .. }
        | Instruction::IfICmpEq { .. }
        | Instruction::IfICmpNe { .. }
        | Instruction::IfICmpLt { .. }
        | Instruction::IfICmpGe { .. }
        | Instruction::IfICmpGt { .. }
        | Instruction::IfICmpLe { .. }
        | Instruction::IfEq { .. }
        | Instruction::IfNe { .. }
        | Instruction::IfLt { .. }
        | Instruction::IfGe { .. }
        | Instruction::IfGt { .. }
        | Instruction::IfLe { .. }
        | Instruction::IfNonNull { .. }
        | Instruction::IfNull { .. }
        | Instruction::InvokeDynamic { .. }
        | Instruction::InvokeInterface { .. }
        | Instruction::InvokeSpecial { .. }
        | Instruction::InvokeStatic { .. }
        | Instruction::InvokeVirtual { .. }
        | Instruction::LookUpSwitch { .. }
        | Instruction::LReturn
        | Instruction::Ret { .. }
        | Instruction::Return
        | Instruction::TableSwitch { .. } => true,
        _ => false,
    }
}

#[test]
pub fn test_flow_graph() {
    let class = parse_class_file("/home/wazed/IdeaProjects/Eternal-v3/target/classes/Start.class");

    let class1 = class.unwrap();
    for method in class1.methods.iter() {
        let instructions = if let Some(Attribute::Code {
            code: Compatibility::Current(code),
            ..
        }) = method
            .attributes
            .iter()
            .find(|a| matches!(a, Attribute::Code { .. }))
        {
            code
        } else {
            panic!("No code attrib found");
        };
        generate_flow_graph(instructions);
    }
}

use crate::instruction_info::is_flow_instruction;
use crate::translate::Translate;
use jbmf_ir::block::BasicBlock;
use jbmf_ir::flow_graph::FlowGraph;
use jbmf_ir::statement::Statement;
use jbmf_parser::java_rs_pacific::attribute::{Attribute, Compatibility, Instruction};
use jbmf_parser::java_rs_pacific::{Constant, JavaClass, Method, SizedVec};
use jbmf_parser::parse_class_file;
use std::collections::HashSet;

pub fn generate_flow_graph(class: JavaClass) -> FlowGraph<BasicBlock, (i16, i16)> {
    let mut blocks = Vec::new();
    for method in class.methods.iter() {
        let method_name = if let Some(Constant::Utf8(var)) = class.constant_pool.get(method.name) {
            var
        } else {
            unreachable!()
        };

        method
            .attributes
            .iter()
            .filter(|attribute| matches!(attribute, Attribute::Code { .. }))
            .for_each(|attribute| {
                if let Attribute::Code {
                    code: Compatibility::Current(code),
                    ..
                } = attribute
                {
                    let mut instructions = Vec::new();
                    for (index, instruction) in code.iter().enumerate() {
                        let statement = Statement::translate(instruction.clone());
                        instructions.push(statement);
                        if is_flow_instruction(instruction) {
                            blocks.push(BasicBlock {
                                beg_index: index as u64,
                                statements: instructions.clone(),
                            });
                            instructions.clear();
                        }
                    }
                }
            });
    }
    for block in blocks {
        println!("Block {:#?}", block);
    }
    FlowGraph {
        vertices: HashSet::new(),
        edges: vec![],
    }
}

#[test]
pub fn test_flow_graph() {
    let class = parse_class_file(
        "/home/wazed/IdeaProjects/Eternal-v3/target/classes/dev/eternal/client/module/impl/combat/Aura.class",
    );

    let class1 = class.unwrap();
    generate_flow_graph(class1);
}

use crate::block::BasicBlock;
use crate::flow_graph::FlowGraph;

pub struct Function {
    pub owner: String,
    pub name: String,
    pub descriptor: String,
    pub blocks: FlowGraph<BasicBlock, (i64, i64)>,
    pub start: BasicBlock,
}

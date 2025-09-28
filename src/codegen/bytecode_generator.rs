#![allow(clippy::new_without_default)]

use crate::{
    cfg_ir::{basic_block, block_id::BlockId, cfg_ir::CfgIr, graph_traversal::Postorder},
    virtual_machine::value::Value,
};

use super::instruction::Instruction;

pub struct BytecodeGenerator {
    pub instructions: Vec<Instruction>,
    pub constant_pool: Vec<Value>,
    pub cfg_ir: CfgIr,
    pub traversal: Postorder,
}

impl BytecodeGenerator {
    pub fn new(cfg_ir: CfgIr) -> Self {
        Self {
            instructions: Vec::new(),
            constant_pool: Vec::new(),
            cfg_ir,
            traversal: Postorder::new(),
        }
    }

    pub fn generate(&mut self) {
        for cfg in &self.cfg_ir.cfgs {
            let reversed_post_order = self
                .traversal
                .reversed_postorder(cfg, &self.cfg_ir.basic_blocks);

            for block_id in reversed_post_order {
                self.visit_block(block_id);
            }
        }
    }

    fn visit_block(&self, basic_block: BlockId) {}
}

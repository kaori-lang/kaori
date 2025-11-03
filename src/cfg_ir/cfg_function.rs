use std::fmt::{self, Display, Formatter};

use crate::cfg_ir::graph_traversal::reversed_postorder;

use super::{
    basic_block::{BasicBlock, Terminator},
    cfg_constants::CfgConstants,
    cfg_instruction::CfgInstruction,
};

#[derive(Default)]
pub struct CfgFunction {
    pub basic_blocks: Vec<BasicBlock>,
    pub constants: CfgConstants,
}

impl Display for CfgFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Control Flow Graph (CFG):")?;

        for index in reversed_postorder(&self.basic_blocks) {
            let block = &self.basic_blocks[index];
            write!(f, "{}", block)?;
        }

        Ok(())
    }
}

impl CfgFunction {
    fn emit_instruction(&mut self, index: usize, instruction: CfgInstruction) {
        let basic_block = &mut self.basic_blocks[index];

        let Terminator::None = basic_block.terminator else {
            return;
        };

        basic_block.instructions.push(instruction);
    }

    fn set_terminator(&mut self, index: usize, terminator: Terminator) {
        let basic_block = &mut self.basic_blocks[index];

        let Terminator::None = basic_block.terminator else {
            return;
        };

        basic_block.terminator = terminator;
    }

    fn create_bb(&mut self) -> usize {
        let index = self.basic_blocks.len();

        let basic_block = BasicBlock::new(index);

        self.basic_blocks.push(basic_block);

        index
    }
}

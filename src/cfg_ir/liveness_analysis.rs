use std::collections::HashMap;

use super::{
    basic_block::{BasicBlock, Terminator},
    cfg::Cfg,
    cfg_instruction::CfgInstruction,
};

pub struct LivenessAnalysis<'a> {
    cfgs: &'a [Cfg],
    register_lifetime: HashMap<usize, usize>,
}

impl<'a> LivenessAnalysis<'a> {
    pub fn new(cfgs: &'a [Cfg]) -> Self {
        Self {
            cfgs,
            register_lifetime: HashMap::new(),
        }
    }

    pub fn analyze_cfg(&self, cfg: &Cfg) {}

    pub fn analyze_basic_block(&self, cfg: &Cfg, bb: &BasicBlock) {
        match &bb.terminator {
            Terminator::Branch { r#true, r#false } => {
                let left_bb = &cfg.basic_blocks[*r#true];
                let right_bb = &cfg.basic_blocks[*r#false];

                self.analyze_basic_block(cfg, left_bb);
                self.analyze_basic_block(cfg, right_bb);
            }
            Terminator::Goto(target) => {
                let bb = &cfg.basic_blocks[*target];

                self.analyze_basic_block(cfg, bb);
            }
            Terminator::Return => {}
            _ => unreachable!(),
        }
    }

    pub fn analyze_instructions(&self, instructions: &[CfgInstruction]) {}
}

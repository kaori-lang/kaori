use super::basic_block::BlockId;

#[derive(Debug)]
pub struct LoopLabel {
    pub increment_bb: BlockId,
    pub terminator_bb: BlockId,
}

#[derive(Default)]
pub struct ActiveLoops {
    active_loops: Vec<LoopLabel>,
}

impl ActiveLoops {
    pub fn push(&mut self, increment_bb: BlockId, terminator_bb: BlockId) {
        let active_loop = LoopLabel {
            increment_bb,
            terminator_bb,
        };

        self.active_loops.push(active_loop);
    }

    pub fn pop(&mut self) {
        self.active_loops.pop();
    }

    pub fn top(&self) -> &LoopLabel {
        self.active_loops.last().unwrap()
    }
}

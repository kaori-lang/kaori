use super::basic_block::BlockId;

#[derive(Debug)]
pub struct LoopLabel {
    pub start: BlockId,
    pub end: BlockId,
}

#[derive(Default)]
pub struct ActiveLoops {
    active_loops: Vec<LoopLabel>,
}

impl ActiveLoops {
    pub fn push(&mut self, start: BlockId, end: BlockId) {
        let active_loop = LoopLabel { start, end };

        self.active_loops.push(active_loop);
    }

    pub fn pop(&mut self) {
        self.active_loops.pop();
    }

    pub fn top(&self) -> &LoopLabel {
        self.active_loops.last().unwrap()
    }
}

#[derive(Debug)]
pub struct LoopLabel {
    pub increment_bb_index: usize,
    pub terminator_bb_index: usize,
}

#[derive(Default)]
pub struct ActiveLoops {
    active_loops: Vec<LoopLabel>,
}

impl ActiveLoops {
    pub fn push(&mut self, increment_bb_index: usize, terminator_bb_index: usize) {
        let active_loop = LoopLabel {
            increment_bb_index,
            terminator_bb_index,
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

use crate::bytecode::instruction::Instruction;

pub struct CallStack {
    pub function_frames: Vec<FunctionFrame>,
}

impl CallStack {
    pub fn new(return_address: *const Instruction) -> Self {
        let mut function_frames = Vec::new();

        let main_frame = FunctionFrame::new(0, return_address, 0);

        function_frames.push(main_frame);

        Self { function_frames }
    }

    pub fn pop_frame(&mut self) -> FunctionFrame {
        self.function_frames.pop().unwrap()
    }
}

#[derive(Clone, Copy)]
pub struct FunctionFrame {
    pub base_address: usize,
    pub return_address: *const Instruction,
    pub return_register: i16,
}

impl FunctionFrame {
    pub fn new(
        base_address: usize,
        return_address: *const Instruction,
        return_register: i16,
    ) -> Self {
        Self {
            base_address,
            return_address,
            return_register,
        }
    }
}

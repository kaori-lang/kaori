pub struct CallStack {
    pub function_frames: Vec<FunctionFrame>,
}

impl CallStack {
    pub fn new(return_address: usize) -> Self {
        let mut function_frames = Vec::new();

        let main_frame = FunctionFrame::new(0, return_address);

        function_frames.push(main_frame);

        Self { function_frames }
    }
}

pub struct FunctionFrame {
    pub base_address: usize,
    pub return_address: usize,
}

impl FunctionFrame {
    pub fn new(base_address: usize, return_address: usize) -> Self {
        Self {
            base_address,
            return_address,
        }
    }
}

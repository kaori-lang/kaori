use super::value::Value;

pub struct Function {
    pub ip: *const u16,
    pub frame_size: u8,
    pub constants: Vec<Value>,
}

impl Function {
    pub fn new(ip: *const u16, frame_size: u8, constants: Vec<Value>) -> Self {
        Self {
            ip,
            frame_size,
            constants,
        }
    }
}

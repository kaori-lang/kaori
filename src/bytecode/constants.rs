use ordered_float::OrderedFloat;

#[derive(Default)]
pub struct Constants(Vec<Constant>);

impl Constants {
    fn push_constant(&mut self, constant: Constant) -> u8 {
        if let Some(index) = self.0.iter().position(|c| *c == constant) {
            index as u8
        } else {
            let index = self.0.len();

            assert!(index < 256, "constant pool overflow (u8 limit)");

            self.0.push(constant);
            index as u8
        }
    }

    pub fn push_function_index(&mut self, value: usize) -> u8 {
        self.push_constant(Constant::FunctionIndex(value))
    }

    pub fn push_string(&mut self, value: String) -> u8 {
        self.push_constant(Constant::String(value))
    }

    pub fn push_number(&mut self, value: f64) -> u8 {
        self.push_constant(Constant::Number(OrderedFloat(value)))
    }

    pub fn push_boolean(&mut self, value: bool) -> u8 {
        self.push_constant(Constant::Boolean(value))
    }

    pub fn push_nil(&mut self) -> u8 {
        self.push_constant(Constant::Nil)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    String(String),
    Number(OrderedFloat<f64>),
    Boolean(bool),
    FunctionIndex(usize),
    Nil,
}

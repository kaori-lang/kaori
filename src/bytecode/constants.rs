use super::operand::Operand;

#[derive(Default)]
pub struct Constants(pub Vec<Constant>);

impl Constants {
    fn push_constant(&mut self, constant: Constant) -> Operand {
        let index = if let Some(index) = self.0.iter().position(|c| *c == constant) {
            index
        } else {
            let index = self.0.len();
            assert!(
                index < 128,
                "constant pool overflow (i8 negative space limit)"
            );
            self.0.push(constant);
            index
        };

        Operand::Constant(index as u8)
    }

    pub fn push_function_index(&mut self, value: usize) -> Operand {
        self.push_constant(Constant::FunctionIndex(value))
    }

    pub fn push_string(&mut self, value: String) -> Operand {
        self.push_constant(Constant::String(value))
    }

    pub fn push_number(&mut self, value: f64) -> Operand {
        self.push_constant(Constant::Number(value))
    }

    pub fn push_boolean(&mut self, value: bool) -> Operand {
        self.push_constant(Constant::Boolean(value))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    String(String),
    Number(f64),
    Boolean(bool),
    FunctionIndex(usize),
}

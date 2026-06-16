use crate::{
    codegen::{lower_ast::Lower, operand::Constant},
    runtime::value::Value,
    util::string_interner::Symbol,
};

impl<'a> Lower<'a> {
    fn get_or_insert(&mut self, value: Value) -> usize {
        if let Some(index) =
            self.constants.iter().copied().position(|c| c == value)
        {
            return index;
        }

        let index = self.constants.len();
        self.constants.push(value);

        index
    }

    pub fn store_constant(&mut self, constant: Constant) -> usize {
        match constant {
            Constant::Boolean(value) => self.get_or_insert(Value::bool(value)),
            Constant::Nil => self.store_nil_const(),
            Constant::Number(value) => self.get_or_insert(Value::number(value)),
            Constant::String(value) => self.get_or_insert(Value::string(value)),
        }
    }

    pub fn store_string_const(&mut self, value: Symbol) -> usize {
        self.get_or_insert(Value::string(value))
    }

    pub fn store_number_const(&mut self, value: f64) -> usize {
        self.get_or_insert(Value::number(value))
    }

    pub fn store_nil_const(&mut self) -> usize {
        self.get_or_insert(Value::nil())
    }
}

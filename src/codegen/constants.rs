use ordered_float::OrderedFloat;

use crate::{codegen::lower_ast::Lower, runtime::value::Value, util::string_interner::Symbol};

impl<'a> Lower<'a> {
    fn get_or_insert(&mut self, value: Value) -> usize {
        if let Some(index) = self.constants.iter().copied().position(|c| c == value) {
            return index;
        }

        let index = self.constants.len();
        self.constants.push(value);

        index
    }

    pub fn store_string_const(&mut self, value: Symbol) -> usize {
        self.get_or_insert(Value::string(value))
    }

    pub fn store_number_const(&mut self, value: f64) -> usize {
        self.get_or_insert(Value::number(OrderedFloat(value)))
    }

    pub fn store_nil_const(&mut self) -> usize {
        self.get_or_insert(Value::nil())
    }

    pub fn store_boolean_const(&mut self, value: bool) -> usize {
        self.get_or_insert(Value::bool(value))
    }
}

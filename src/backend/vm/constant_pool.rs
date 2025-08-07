pub struct ConstantPool {
    number_constants: Vec<f64>,
    boolean_constants: Vec<f64>,
}

impl ConstantPool {
    pub fn add_number(&mut self, other: f64) -> isize {
        for (index, current) in self.number_constants.iter().enumerate() {
            if *current == other {
                return index as isize;
            }
        }

        let index = self.number_constants.len();

        self.number_constants.push(other);

        index as isize
    }

    pub fn add_boolean(&mut self, other: f64) -> isize {
        for (index, current) in self.boolean_constants.iter().enumerate() {
            if *current == other {
                return index as isize;
            }
        }

        let index = self.number_constants.len();

        self.number_constants.push(other);

        index as isize
    }
}

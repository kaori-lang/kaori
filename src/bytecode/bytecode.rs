use super::function::Function;

use std::fmt;
pub struct Bytecode {
    pub functions: Vec<Function>,
}

impl Bytecode {
    pub fn new(functions: Vec<Function>) -> Self {
        Self { functions }
    }
}

impl fmt::Display for Bytecode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, function) in self.functions.iter().enumerate() {
            writeln!(f, "FUNCTION {}:", index)?;

            for instruction in &function.instructions {
                writeln!(f, "   {}", instruction)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

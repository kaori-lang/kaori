#![allow(clippy::new_without_default)]

use crate::error::kaori_error::KaoriError;

pub struct BytecodeGenerator<'a> {
    bytecode: &'a mut Bytecode,
}

impl<'a> BytecodeGenerator<'a> {
    pub fn new(bytecode: &'a mut Bytecode) -> Self {
        Self {
            bytecode,
            list_true: Vec::new(),
            list_false: Vec::new(),
        }
    }

    pub fn generate(&mut self, declarations: &[ResolvedDecl]) -> Result<(), KaoriError> {
        for declaration in declarations {
            self.visit_declaration(declaration)?;
        }

        Ok(())
    }
}

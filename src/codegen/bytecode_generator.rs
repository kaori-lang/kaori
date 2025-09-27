#![allow(clippy::new_without_default)]

use crate::error::kaori_error::KaoriError;

pub struct BytecodeGenerator {}

impl BytecodeGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&mut self) -> Result<(), KaoriError> {
        Ok(())
    }
}

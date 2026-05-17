mod function;

mod instruction;

pub mod lower_ast;

mod optimizations;
pub mod resolve;

pub use function::Function;
pub use instruction::Instruction;

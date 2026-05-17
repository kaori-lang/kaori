pub mod function;

pub mod instruction;
pub use function::Function;

pub mod lower_ast;

pub mod optimizations;
pub mod resolve;

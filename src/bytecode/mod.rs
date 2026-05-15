pub mod function;

pub mod instruction;
pub use function::Function;

pub mod emit_bytecode;

pub mod optimize_bytecode;
pub mod resolve;

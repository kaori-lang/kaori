pub mod function;

pub mod instruction;
pub mod operand;
pub use function::Function;

pub use constants::Constant;
pub mod constants;
pub mod emit_bytecode;
pub mod immediate;
pub mod optimize_bytecode;

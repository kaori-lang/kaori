pub mod basic_block;
pub mod build_cfgs;
#[allow(clippy::module_inception)]
pub mod function;

pub mod graph_traversal;
pub mod instruction;
//pub mod liveness_analysis;
pub mod active_loops;
pub mod constants;

pub mod jump_threading;
pub mod operand;

pub use instruction::Instruction;

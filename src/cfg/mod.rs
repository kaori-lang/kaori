pub mod basic_block;
pub mod build_functions_graph;
#[allow(clippy::module_inception)]
pub mod function;

pub mod graph_traversal;
pub mod instruction;
//pub mod liveness_analysis;
pub mod active_loops;
pub mod constant_pool;

pub mod jump_threading;
pub mod operand;
pub mod validate_functions_return;

pub use instruction::Instruction;

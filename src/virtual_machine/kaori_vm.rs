use std::hint::unreachable_unchecked;

use crate::bytecode::{instruction::Instruction, value::Value};

use super::call_stack::CallStack;

const DISPATCH_OP: [fn(ctx: &mut VMContext, index: usize); 1] = [instruction_add];

pub struct VMContext {
    pub call_stack: CallStack,
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub registers: Vec<Value>,
    pub instruction_index: usize,
}

impl VMContext {
    pub fn new(instructions: Vec<Instruction>, constants: Vec<Value>) -> Self {
        let return_address = instructions.len();
        Self {
            call_stack: CallStack::new(return_address),
            instructions,
            constants,
            registers: vec![Value::default(); 1024],
            instruction_index: 0,
        }
    }
}

pub fn get_value(ctx: &VMContext, register: i16) -> &Value {
    if register < 0 {
        &ctx.constants[-register as usize]
    } else {
        &ctx.registers[register as usize]
    }
}

pub fn set_value(ctx: &mut VMContext, register: i16, value: Value) {
    ctx.registers[register as usize] = value;
}

#[inline(never)]
fn instruction_move(ctx: &mut VMContext, index: usize) {
    let Instruction::Move { dest, src } = ctx.instructions[index] else {
        unsafe { unreachable_unchecked() }
    };

    let val = get_value(ctx, src);
    set_value(ctx, dest, *val);
}

#[inline(never)]
fn instruction_add(ctx: &mut VMContext, index: usize) {
    let Instruction::Add { dest, src1, src2 } = ctx.instructions[index] else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2 as i16).as_number();
    set_value(ctx, dest, Value::number(lhs + rhs));

    DISPATCH_OP[0](ctx, index + 1)
}

#[inline(never)]
fn instruction_subtract(ctx: &mut VMContext, index: usize) {
    let Instruction::Subtract { dest, src1, src2 } = ctx.instructions[index] else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2 as i16).as_number();
    set_value(ctx, dest, Value::number(lhs - rhs));
}

#[inline(never)]
fn instruction_multiply(ctx: &mut VMContext, index: usize) {
    let Instruction::Multiply { dest, src1, src2 } = ctx.instructions[index] else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2 as i16).as_number();
    set_value(ctx, dest, Value::number(lhs * rhs));
}

#[inline(never)]
fn instruction_divide(ctx: &mut VMContext, index: usize) {
    let Instruction::Divide { dest, src1, src2 } = ctx.instructions[index] else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2 as i16).as_number();
    set_value(ctx, dest, Value::number(lhs / rhs));
}

#[inline(never)]
fn instruction_modulo(ctx: &mut VMContext, index: usize) {
    let Instruction::Modulo { dest, src1, src2 } = ctx.instructions[index] else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::number(lhs % rhs));
}

#[inline(never)]
fn instruction_equal(ctx: &mut VMContext, index: usize) {
    let Instruction::Equal { dest, src1, src2 } = ctx.instructions[index] else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::boolean(lhs == rhs));
}

#[inline(never)]
fn instruction_not_equal(ctx: &mut VMContext, index: usize) {
    let Instruction::NotEqual { dest, src1, src2 } = ctx.instructions[index] else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::boolean(lhs != rhs));
}

#[inline(never)]
fn instruction_greater(ctx: &mut VMContext, index: usize) {
    let Instruction::Greater { dest, src1, src2 } = ctx.instructions[index] else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::boolean(lhs > rhs));
}

#[inline(never)]
fn instruction_greater_equal(ctx: &mut VMContext, index: usize) {
    let Instruction::GreaterEqual { dest, src1, src2 } = ctx.instructions[index] else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::boolean(lhs >= rhs));
}

#[inline(never)]
fn instruction_less(ctx: &mut VMContext, index: usize) {
    let Instruction::Less { dest, src1, src2 } = ctx.instructions[index] else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::boolean(lhs < rhs));
}

#[inline(never)]
fn instruction_less_equal(ctx: &mut VMContext, index: usize) {
    let Instruction::LessEqual { dest, src1, src2 } = ctx.instructions[index] else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::boolean(lhs <= rhs));
}

#[inline(never)]
fn instruction_negate(ctx: &mut VMContext, index: usize) {
    let Instruction::Negate { dest, src } = ctx.instructions[index] else {
        unsafe { unreachable_unchecked() }
    };

    let val = get_value(ctx, src).as_number();
    set_value(ctx, dest, Value::number(-val));
}

#[inline(never)]
fn instruction_not(ctx: &mut VMContext, index: usize) {
    let Instruction::Not { dest, src } = ctx.instructions[index] else {
        unsafe { unreachable_unchecked() }
    };

    let val = get_value(ctx, src).as_boolean();
    set_value(ctx, dest, Value::boolean(!val));
}

#[inline(never)]
fn instruction_jump(ctx: &mut VMContext, index: usize) {
    let Instruction::Jump { offset } = ctx.instructions[index] else {
        unsafe {
            unreachable_unchecked();
        }
    };

    ctx.instruction_index = ((ctx.instruction_index as isize) + (offset as isize)) as usize;
}

#[inline(never)]
fn instruction_print(ctx: &mut VMContext, index: usize) {
    let Instruction::Print { src } = ctx.instructions[index] else {
        unsafe { unreachable_unchecked() }
    };

    let val = get_value(ctx, src).as_number();
    println!("{val}");
}

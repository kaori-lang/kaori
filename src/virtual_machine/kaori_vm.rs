use std::hint::unreachable_unchecked;

use crate::bytecode::{instruction::Instruction, value::Value};

use super::call_stack::CallStack;

type InstructionHandler =
    fn(&mut VMContext, instruction: &Instruction, instructions: &[Instruction], index: usize);
pub struct VMContext {
    pub call_stack: CallStack,
    pub constants: Vec<Value>,
    pub registers: Vec<Value>,
    pub instruction_dispatch: [InstructionHandler; 20],
}

impl VMContext {
    pub fn new(return_address: usize, constants: Vec<Value>) -> Self {
        Self {
            call_stack: CallStack::new(return_address),
            constants,
            registers: vec![Value::default(); 1024],
            instruction_dispatch: [
                instruction_add,              // 0
                instruction_subtract,         // 1
                instruction_multiply,         // 2
                instruction_divide,           // 3
                instruction_modulo,           // 4
                instruction_equal,            // 5
                instruction_not_equal,        // 6
                instruction_greater,          // 7
                instruction_greater_equal,    // 8
                instruction_less,             // 9
                instruction_less_equal,       // 10
                instruction_negate,           // 11
                instruction_not,              // 12
                instruction_move,             // 13
                instruction_call,             // 14
                instruction_return,           // 15
                instruction_jump,             // 16
                instruction_conditional_jump, // 17
                instruction_print,            // 18
                instruction_halt,             // 19
            ],
        }
    }
}

pub fn run_vm(instructions: Vec<Instruction>, constants: Vec<Value>) {
    let mut ctx = VMContext::new(instructions.len(), constants);

    let instruction = &instructions[0];
    ctx.instruction_dispatch[instruction.discriminant()](&mut ctx, instruction, &instructions, 0);
}

#[inline(always)]
fn get_value(ctx: &VMContext, register: i16) -> &Value {
    if register < 0 {
        &ctx.constants[-register as usize]
    } else {
        &ctx.registers[register as usize]
    }
}
#[inline(always)]
fn set_value(ctx: &mut VMContext, register: i16, value: Value) {
    ctx.registers[register as usize] = value;
}

#[inline(never)]
fn instruction_move(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
    let Instruction::Move { dest, src } = *instruction else {
        unsafe { unreachable_unchecked() }
    };

    let value = get_value(ctx, src);
    set_value(ctx, dest, *value);

    let index = index + 1;
    let instruction = &instructions[index];

    become ctx.instruction_dispatch[instruction.discriminant()](
        ctx,
        instruction,
        instructions,
        index,
    )
}

#[inline(never)]
fn instruction_add(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
    let Instruction::Add { dest, src1, src2 } = *instruction else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::number(lhs + rhs));

    let index = index + 1;
    let instruction = &instructions[index];

    become ctx.instruction_dispatch[instruction.discriminant()](
        ctx,
        instruction,
        instructions,
        index,
    )
}

#[inline(never)]
fn instruction_subtract(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
    let Instruction::Subtract { dest, src1, src2 } = *instruction else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::number(lhs - rhs));

    let index = index + 1;
    let instruction = &instructions[index];

    become ctx.instruction_dispatch[instruction.discriminant()](
        ctx,
        instruction,
        instructions,
        index,
    )
}

#[inline(never)]
fn instruction_multiply(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
    let Instruction::Multiply { dest, src1, src2 } = *instruction else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::number(lhs * rhs));

    let index = index + 1;
    let instruction = &instructions[index];
    become ctx.instruction_dispatch[instruction.discriminant()](
        ctx,
        instruction,
        instructions,
        index,
    )
}

#[inline(never)]
fn instruction_divide(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
    let Instruction::Divide { dest, src1, src2 } = *instruction else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::number(lhs / rhs));

    let index = index + 1;
    let instruction = &instructions[index];
    become ctx.instruction_dispatch[instruction.discriminant()](
        ctx,
        instruction,
        instructions,
        index,
    )
}

#[inline(never)]
fn instruction_modulo(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
    let Instruction::Modulo { dest, src1, src2 } = *instruction else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::number(lhs % rhs));

    let index = index + 1;
    let instruction = &instructions[index];
    become ctx.instruction_dispatch[instruction.discriminant()](
        ctx,
        instruction,
        instructions,
        index,
    )
}

#[inline(never)]
fn instruction_equal(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
    let Instruction::Equal { dest, src1, src2 } = *instruction else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::boolean(lhs == rhs));

    let index = index + 1;
    let instruction = &instructions[index];
    become ctx.instruction_dispatch[instruction.discriminant()](
        ctx,
        instruction,
        instructions,
        index,
    )
}

#[inline(never)]
fn instruction_not_equal(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
    let Instruction::NotEqual { dest, src1, src2 } = *instruction else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::boolean(lhs != rhs));

    let index = index + 1;
    let instruction = &instructions[index];
    become ctx.instruction_dispatch[instruction.discriminant()](
        ctx,
        instruction,
        instructions,
        index,
    )
}

#[inline(never)]
fn instruction_greater(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
    let Instruction::Greater { dest, src1, src2 } = *instruction else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::boolean(lhs > rhs));

    let index = index + 1;
    let instruction = &instructions[index];
    become ctx.instruction_dispatch[instruction.discriminant()](
        ctx,
        instruction,
        instructions,
        index,
    )
}

#[inline(never)]
fn instruction_greater_equal(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
    let Instruction::GreaterEqual { dest, src1, src2 } = *instruction else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::boolean(lhs >= rhs));

    let index = index + 1;
    let instruction = &instructions[index];
    become ctx.instruction_dispatch[instruction.discriminant()](
        ctx,
        instruction,
        instructions,
        index,
    )
}

#[inline(never)]
fn instruction_less(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
    let Instruction::Less { dest, src1, src2 } = *instruction else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::boolean(lhs < rhs));

    let index = index + 1;
    let instruction = &instructions[index];
    become ctx.instruction_dispatch[instruction.discriminant()](
        ctx,
        instruction,
        instructions,
        index,
    )
}

#[inline(never)]
fn instruction_less_equal(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
    let Instruction::LessEqual { dest, src1, src2 } = *instruction else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = get_value(ctx, src1).as_number();
    let rhs = get_value(ctx, src2).as_number();
    set_value(ctx, dest, Value::boolean(lhs <= rhs));

    let index = index + 1;
    let instruction = &instructions[index];
    become ctx.instruction_dispatch[instruction.discriminant()](
        ctx,
        instruction,
        instructions,
        index,
    )
}

#[inline(never)]
fn instruction_negate(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
    let Instruction::Negate { dest, src } = *instruction else {
        unsafe { unreachable_unchecked() }
    };

    let value = get_value(ctx, src).as_number();
    set_value(ctx, dest, Value::number(-value));

    let index = index + 1;
    let instruction = &instructions[index];
    become ctx.instruction_dispatch[instruction.discriminant()](
        ctx,
        instruction,
        instructions,
        index,
    )
}

#[inline(never)]
fn instruction_not(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
    let Instruction::Not { dest, src } = *instruction else {
        unsafe { unreachable_unchecked() }
    };

    let value = get_value(ctx, src).as_boolean();
    set_value(ctx, dest, Value::boolean(!value));

    let index = index + 1;
    let instruction = &instructions[index];
    become ctx.instruction_dispatch[instruction.discriminant()](
        ctx,
        instruction,
        instructions,
        index,
    )
}

#[inline(never)]
fn instruction_jump(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
    let Instruction::Jump { offset } = *instruction else {
        unsafe { unreachable_unchecked() }
    };

    let jump_index = (index as i16 + offset) as usize;
    let instruction = &instructions[jump_index];
    become ctx.instruction_dispatch[instruction.discriminant()](
        ctx,
        instruction,
        instructions,
        jump_index,
    )
}

#[inline(never)]
fn instruction_conditional_jump(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
    let Instruction::ConditionalJump {
        src,
        true_offset,
        false_offset,
    } = *instruction
    else {
        unsafe { unreachable_unchecked() }
    };

    let value = get_value(ctx, src);
    let offset = if value.as_boolean() {
        true_offset
    } else {
        false_offset
    };
    let jump_index = (index as i16 + offset) as usize;
    let instruction = &instructions[jump_index];
    become ctx.instruction_dispatch[instruction.discriminant()](
        ctx,
        instruction,
        instructions,
        jump_index,
    )
}

#[inline(never)]
fn instruction_call(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
}

#[inline(never)]
fn instruction_return(
    ctx: &mut VMContext,
    _instruction: &Instruction,
    instructions: &[Instruction],
    _index: usize,
) {
}

#[inline(never)]
fn instruction_print(
    ctx: &mut VMContext,
    instruction: &Instruction,
    instructions: &[Instruction],
    index: usize,
) {
    let Instruction::Print { src } = *instruction else {
        unsafe { unreachable_unchecked() }
    };

    let value = get_value(ctx, src).as_number();
    println!("{value}");

    let index = index + 1;
    let instruction = &instructions[index];
    become ctx.instruction_dispatch[instruction.discriminant()](
        ctx,
        instruction,
        instructions,
        index,
    )
}

#[inline(never)]
fn instruction_halt(
    _ctx: &mut VMContext,
    _instruction: &Instruction,
    _instructions: &[Instruction],
    _index: usize,
) {
    println!("Program finished!");
}

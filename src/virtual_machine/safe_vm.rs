use std::hint::unreachable_unchecked;

use crate::bytecode::{instruction::Instruction, value::Value};

pub struct FunctionFrame {
    pub registers: *mut Value,
    pub return_address: usize,
    pub return_register: i16,
}

impl FunctionFrame {
    pub fn new(registers: *mut Value, return_address: usize, return_register: i16) -> Self {
        Self {
            registers,
            return_address,
            return_register,
        }
    }
}

type InstructionHandler =
    fn(ctx: &mut VMContext, index: usize, instruction: &Instruction, instructions: &[Instruction]);
pub struct VMContext {
    pub call_stack: Vec<FunctionFrame>,
    pub constants: Vec<Value>,
    pub registers: *mut Value,
    pub instruction_dispatch: [InstructionHandler; 20],
}

macro_rules! dispatch_next {
    ($ctx:ident, $instructions: expr, $index: expr) => {
        let instruction = &$instructions[$index + 1];
        let op_code = instruction.discriminant();

        become $ctx.instruction_dispatch[op_code]($ctx, $index + 1, instruction, $instructions);
    };
}

impl VMContext {
    pub fn new(constants: Vec<Value>, registers: *mut Value) -> Self {
        Self {
            call_stack: Vec::new(),
            constants,
            registers,
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

    #[inline(always)]
    fn get_value(&self, register: i16) -> Value {
        if register < 0 {
            self.constants[-register as usize]
        } else {
            unsafe { *self.registers.add(register as usize) }
        }
    }

    #[inline(always)]
    fn set_value(&mut self, register: i16, value: Value) {
        unsafe {
            *self.registers.add(register as usize) = value;
        }
    }

    #[inline(always)]
    fn pop_frame(&mut self) -> FunctionFrame {
        let frame = unsafe { self.call_stack.pop().unwrap_unchecked() };

        self.registers = unsafe { self.call_stack.last().unwrap_unchecked().registers };

        frame
    }

    #[inline(always)]
    fn push_frame(&mut self, return_register: i16, return_address: usize, caller_size: u16) {
        let registers = unsafe { self.registers.add(caller_size as usize) };

        self.registers = registers;

        let frame = FunctionFrame::new(registers, return_address, return_register);

        self.call_stack.push(frame);
    }
}

pub fn run_safe_vm(instructions: Vec<Instruction>, constants: Vec<Value>) {
    let halt = instructions.len() - 1;

    let mut registers = vec![Value::default(); 1024];

    let mut ctx = VMContext::new(constants, registers.as_mut_ptr());

    ctx.push_frame(0, halt, 0);
    ctx.push_frame(0, halt, 0);

    let instruction = instructions[0];
    let op_code = instructions[0].discriminant();

    ctx.instruction_dispatch[op_code](&mut ctx, 0, &instruction, &instructions)
}

#[inline(never)]
fn instruction_move(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::Move { dest, src } = *instruction else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let value = ctx.get_value(src);
    ctx.set_value(dest, value);

    dispatch_next!(ctx, instructions, index);
}

#[inline(never)]
fn instruction_add(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    // match by reference — no enum copy
    let Instruction::Add {
        dest: &dest,
        src1: &src1,
        src2: &src2,
    } = instruction
    else {
        unsafe { unreachable_unchecked() }
    };

    let lhs = ctx.get_value(src1).as_number();
    let rhs = ctx.get_value(src2).as_number();
    ctx.set_value(dest, Value::number(lhs + rhs));

    // get next instruction without bounds check
    let next_instruction = unsafe { instructions.get_unchecked(index + 1) };
    let op_code = next_instruction.discriminant();

    become ctx.instruction_dispatch[op_code](ctx, index + 1, next_instruction, instructions);
}

#[inline(never)]
fn instruction_subtract(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::Subtract { dest, src1, src2 } = *instruction else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let lhs = ctx.get_value(src1).as_number();
    let rhs = ctx.get_value(src2).as_number();
    ctx.set_value(dest, Value::number(lhs - rhs));

    dispatch_next!(ctx, instructions, index);
}

#[inline(never)]
fn instruction_multiply(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::Multiply { dest, src1, src2 } = *instruction else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let lhs = ctx.get_value(src1).as_number();
    let rhs = ctx.get_value(src2).as_number();
    ctx.set_value(dest, Value::number(lhs * rhs));

    dispatch_next!(ctx, instructions, index);
}

#[inline(never)]
fn instruction_divide(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::Divide { dest, src1, src2 } = *instruction else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let lhs = ctx.get_value(src1).as_number();
    let rhs = ctx.get_value(src2).as_number();
    ctx.set_value(dest, Value::number(lhs / rhs));

    dispatch_next!(ctx, instructions, index);
}

#[inline(never)]
fn instruction_modulo(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::Modulo { dest, src1, src2 } = *instruction else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let lhs = ctx.get_value(src1).as_number();
    let rhs = ctx.get_value(src2).as_number();
    ctx.set_value(dest, Value::number(lhs % rhs));

    dispatch_next!(ctx, instructions, index);
}

#[inline(never)]
fn instruction_equal(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::Equal { dest, src1, src2 } = *instruction else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let lhs = ctx.get_value(src1).as_number();
    let rhs = ctx.get_value(src2).as_number();
    ctx.set_value(dest, Value::boolean(lhs == rhs));

    dispatch_next!(ctx, instructions, index);
}

#[inline(never)]
fn instruction_not_equal(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::NotEqual { dest, src1, src2 } = *instruction else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let lhs = ctx.get_value(src1).as_number();
    let rhs = ctx.get_value(src2).as_number();
    ctx.set_value(dest, Value::boolean(lhs != rhs));

    dispatch_next!(ctx, instructions, index);
}

#[inline(never)]
fn instruction_greater(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::Greater { dest, src1, src2 } = *instruction else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let lhs = ctx.get_value(src1).as_number();
    let rhs = ctx.get_value(src2).as_number();
    ctx.set_value(dest, Value::boolean(lhs > rhs));

    dispatch_next!(ctx, instructions, index);
}

#[inline(never)]
fn instruction_greater_equal(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::GreaterEqual { dest, src1, src2 } = *instruction else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let lhs = ctx.get_value(src1).as_number();
    let rhs = ctx.get_value(src2).as_number();
    ctx.set_value(dest, Value::boolean(lhs >= rhs));

    dispatch_next!(ctx, instructions, index);
}

#[inline(never)]
fn instruction_less(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::Less { dest, src1, src2 } = *instruction else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let lhs = ctx.get_value(src1).as_number();
    let rhs = ctx.get_value(src2).as_number();
    ctx.set_value(dest, Value::boolean(lhs < rhs));

    dispatch_next!(ctx, instructions, index);
}

#[inline(never)]
fn instruction_less_equal(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::LessEqual { dest, src1, src2 } = *instruction else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let lhs = ctx.get_value(src1).as_number();
    let rhs = ctx.get_value(src2).as_number();
    ctx.set_value(dest, Value::boolean(lhs <= rhs));

    dispatch_next!(ctx, instructions, index);
}

#[inline(never)]
fn instruction_negate(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::Negate { dest, src } = *instruction else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let value = ctx.get_value(src).as_number();
    ctx.set_value(dest, Value::number(-value));

    dispatch_next!(ctx, instructions, index);
}

#[inline(never)]
fn instruction_not(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::Not { dest, src } = *instruction else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let value = ctx.get_value(src).as_boolean();
    ctx.set_value(dest, Value::boolean(!value));

    dispatch_next!(ctx, instructions, index);
}

#[inline(never)]
fn instruction_jump(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::Jump { offset } = *instruction else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let target_index = (index as isize + offset as isize) as usize;
    let next_instruction = unsafe { instructions.get_unchecked(target_index) };
    let op_code = next_instruction.discriminant();

    become ctx.instruction_dispatch[op_code](ctx, target_index, next_instruction, instructions)
}

#[inline(never)]
fn instruction_conditional_jump(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::ConditionalJump {
        src,
        true_offset,
        false_offset,
    } = *instruction
    else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let value = ctx.get_value(src);
    let target_index = if value.as_boolean() {
        (index as isize + true_offset as isize) as usize
    } else {
        (index as isize + false_offset as isize) as usize
    };

    let next_instruction = unsafe { instructions.get_unchecked(target_index) };
    let op_code = next_instruction.discriminant();

    become ctx.instruction_dispatch[op_code](ctx, target_index, next_instruction, instructions)
}

#[inline(never)]
fn instruction_call(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::Call {
        dest,
        src,
        caller_size,
    } = *instruction
    else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let return_register = dest;
    let return_index = index + 1;

    let function_index = ctx.get_value(src).as_instruction(); // keep as index if possible

    ctx.push_frame(return_register, return_index, caller_size);

    let next_instruction = unsafe { instructions.get_unchecked(function_index) };
    let op_code = next_instruction.discriminant();

    become ctx.instruction_dispatch[op_code](ctx, function_index, next_instruction, instructions)
}

#[inline(never)]
fn instruction_return(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::Return { src } = *instruction else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let value = ctx.get_value(src);
    let frame = ctx.pop_frame();

    ctx.set_value(frame.return_register, value);

    let return_index = frame.return_address;
    let next_instruction = unsafe { instructions.get_unchecked(return_index) };
    let op_code = next_instruction.discriminant();

    become ctx.instruction_dispatch[op_code](ctx, return_index, next_instruction, instructions)
}

#[inline(never)]
fn instruction_print(
    ctx: &mut VMContext,
    index: usize,
    instruction: &Instruction,
    instructions: &[Instruction],
) {
    let Instruction::Print { src } = *instruction else {
        unsafe {
            unreachable_unchecked();
        }
    };

    let value = ctx.get_value(src);
    println!("{}", value.as_number());

    dispatch_next!(ctx, instructions, index);
}

#[inline(never)]
fn instruction_halt(
    _ctx: &mut VMContext,
    _index: usize,
    _instruction: &Instruction,
    _instructions: &[Instruction],
) {
    // stops the VM
}

use std::hint::unreachable_unchecked;

use crate::bytecode::{instruction::Instruction, value::Value};

pub struct FunctionFrame {
    pub registers: *mut Value,
    pub return_address: *const Instruction,
    pub return_register: i16,
}

impl FunctionFrame {
    pub fn new(
        registers: *mut Value,
        return_address: *const Instruction,
        return_register: i16,
    ) -> Self {
        Self {
            registers,
            return_address,
            return_register,
        }
    }
}

type InstructionHandler = fn(&mut VMContext, ip: *const Instruction);

const INSTRUCTION_DISPATCH: [InstructionHandler; 22] = [
    instruction_add,           // 0
    instruction_subtract,      // 1
    instruction_multiply,      // 2
    instruction_divide,        // 3
    instruction_modulo,        // 4
    instruction_equal,         // 5
    instruction_not_equal,     // 6
    instruction_greater,       // 7
    instruction_greater_equal, // 8
    instruction_less,          // 9
    instruction_less_equal,    // 10
    instruction_negate,        // 11
    instruction_not,           // 12
    instruction_move,          // 13
    instruction_call,          // 14
    instruction_return,        // 15
    instruction_return_void,   // 16
    instruction_jump,          // 17
    instruction_jump_if_true,  // 18
    instruction_jump_if_false, // 19
    instruction_print,         // 20
    instruction_halt,          // 21
];

pub struct VMContext {
    pub call_stack: Vec<FunctionFrame>,
    pub constants: *const Value,
    pub registers: *mut Value,
}

macro_rules! dispatch {
    ($ctx:expr, $ip: expr, $op_code:expr) => {
        let _: &mut VMContext = $ctx;
        let _: *const Instruction = $ip;

        become INSTRUCTION_DISPATCH[$op_code]($ctx, $ip)
    };
}

macro_rules! dispatch_next {
    ($ctx:expr, $ip: expr) => {
        let _: &mut VMContext = $ctx;
        let _: *const Instruction = $ip;

        let ip = $ip.add(1);
        let op_code = (*ip).discriminant();

        dispatch!($ctx, ip, op_code);
    };
}

macro_rules! dispatch_to {
    ($ctx:expr, $ip:expr, $offset: expr) => {
        let _: &mut VMContext = $ctx;
        let _: *const Instruction = $ip;
        let _: isize = $offset;

        let ip = $ip.offset($offset);
        let op_code = (*ip).discriminant();

        dispatch!($ctx, ip, op_code);
    };
}

impl VMContext {
    pub fn new(constants: *const Value, registers: *mut Value) -> Self {
        Self {
            call_stack: Vec::new(),
            constants,
            registers,
        }
    }

    #[inline(always)]
    fn get_value(&self, register: i16) -> Value {
        unsafe {
            if register < 0 {
                *self.constants.offset(register as isize)
            } else {
                *self.registers.add(register as usize)
            }
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

        if let Some(frame) = self.call_stack.last() {
            self.registers = frame.registers;
        }

        frame
    }

    #[inline(always)]
    fn push_frame(
        &mut self,
        return_register: i16,
        return_address: *const Instruction,
        caller_size: u16,
    ) {
        let registers = unsafe { self.registers.add(caller_size as usize) };

        self.registers = registers;

        let frame = FunctionFrame::new(registers, return_address, return_register);

        self.call_stack.push(frame);
    }
}

pub fn run_kaori_vm(instructions: Vec<Instruction>, constants: Vec<Value>) {
    let halt_ip = unsafe { instructions.as_ptr().add(instructions.len() - 1) };

    let mut registers = vec![Value::default(); 1024];

    let registers_ptr = registers.as_mut_ptr();
    let constants_ptr = unsafe { constants.as_ptr().add(constants.len() - 1) };

    let mut ctx = VMContext::new(constants_ptr, registers_ptr);

    ctx.push_frame(0, halt_ip, 0);

    let ip = instructions.as_ptr();
    let op_code = instructions[0].discriminant();

    INSTRUCTION_DISPATCH[op_code](&mut ctx, ip);
}

#[inline(never)]
fn instruction_move(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Move { dest, src } = *ip else {
            unreachable_unchecked();
        };

        let value = ctx.get_value(src);
        ctx.set_value(dest, value);

        dispatch_next!(ctx, ip);
    }
}

#[inline(never)]
fn instruction_add(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Add { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };

        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs + rhs));

        dispatch_next!(ctx, ip);
    }
}

#[inline(never)]
fn instruction_subtract(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Subtract { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };

        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::number(lhs - rhs));

        dispatch_next!(ctx, ip);
    }
}

#[inline(never)]
fn instruction_multiply(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Multiply { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::number(lhs * rhs));

        dispatch_next!(ctx, ip);
    }
}

#[inline(never)]
fn instruction_divide(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Divide { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::number(lhs / rhs));

        dispatch_next!(ctx, ip);
    }
}

#[inline(never)]
fn instruction_modulo(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Modulo { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::number(lhs % rhs));

        dispatch_next!(ctx, ip);
    }
}

#[inline(never)]
fn instruction_equal(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Equal { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs == rhs));

        dispatch_next!(ctx, ip);
    }
}

#[inline(never)]
fn instruction_not_equal(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::NotEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::boolean(lhs != rhs));

        dispatch_next!(ctx, ip);
    }
}

#[inline(never)]
fn instruction_greater(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Greater { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::boolean(lhs > rhs));

        dispatch_next!(ctx, ip);
    }
}

#[inline(never)]
fn instruction_greater_equal(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::GreaterEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::boolean(lhs >= rhs));

        dispatch_next!(ctx, ip);
    }
}

#[inline(never)]
fn instruction_less(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Less { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::boolean(lhs < rhs));

        dispatch_next!(ctx, ip);
    }
}

#[inline(never)]
fn instruction_less_equal(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::LessEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::boolean(lhs <= rhs));

        dispatch_next!(ctx, ip);
    }
}

#[inline(never)]
fn instruction_negate(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Negate { dest, src } = *ip else {
            unreachable_unchecked();
        };
        let value = ctx.get_value(src).as_number();
        ctx.set_value(dest, Value::number(-value));

        dispatch_next!(ctx, ip);
    }
}

#[inline(never)]
fn instruction_not(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Not { dest, src } = *ip else {
            unreachable_unchecked();
        };
        let value = ctx.get_value(src).as_boolean();
        ctx.set_value(dest, Value::boolean(!value));

        dispatch_next!(ctx, ip);
    }
}

#[inline(never)]
fn instruction_jump(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Jump { offset } = *ip else {
            unreachable_unchecked();
        };

        dispatch_to!(ctx, ip, offset as isize);
    }
}

#[inline(never)]
fn instruction_jump_if_true(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::JumpIfTrue { src, offset } = *ip else {
            unreachable_unchecked();
        };

        match ctx.get_value(src).as_boolean() {
            true => {
                dispatch_to!(ctx, ip, offset as isize);
            }
            false => {
                dispatch_next!(ctx, ip);
            }
        };
    }
}

#[inline(never)]
fn instruction_jump_if_false(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::JumpIfFalse { src, offset } = *ip else {
            unreachable_unchecked();
        };

        match ctx.get_value(src).as_boolean() {
            true => {
                dispatch_next!(ctx, ip);
            }
            false => {
                dispatch_to!(ctx, ip, offset as isize);
            }
        };
    }
}

#[inline(never)]
fn instruction_call(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Call {
            dest,
            src,
            caller_size,
        } = *ip
        else {
            unreachable_unchecked();
        };

        let return_register = dest;
        let return_address = ip.add(1);

        let ip = ctx.get_value(src).as_instruction();
        let op_code = (*ip).discriminant();

        ctx.push_frame(return_register, return_address, caller_size);

        dispatch!(ctx, ip, op_code);
    }
}

#[inline(never)]
fn instruction_return(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Return { src } = *ip else {
            unreachable_unchecked();
        };

        let value = ctx.get_value(src);

        let frame = ctx.pop_frame();

        let dest = frame.return_register;

        ctx.set_value(dest, value);

        let ip = frame.return_address;
        let op_code = (*ip).discriminant();

        dispatch!(ctx, ip, op_code);
    }
}

#[inline(never)]
fn instruction_return_void(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::ReturnVoid = *ip else {
            unreachable_unchecked();
        };

        let frame = ctx.pop_frame();

        let ip = frame.return_address;
        let op_code = (*ip).discriminant();

        dispatch!(ctx, ip, op_code);
    }
}

#[inline(never)]
fn instruction_print(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Print { src } = *ip else {
            unreachable_unchecked();
        };

        let value = ctx.get_value(src).as_number();

        println!("{}", value);

        dispatch_next!(ctx, ip);
    }
}

#[inline(never)]
fn instruction_halt(_ctx: &mut VMContext, _ip: *const Instruction) {}

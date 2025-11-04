use std::hint::unreachable_unchecked;

use crate::bytecode::{function::Function, instruction::Instruction, value::Value};

pub struct FunctionFrame {
    pub size: u8,
    pub registers_ptr: *mut Value,
    pub constants_ptr: *const Value,
    pub return_address: *const Instruction,
    pub return_register: i16,
}

impl FunctionFrame {
    pub fn new(
        size: u8,
        registers_ptr: *mut Value,
        constants_ptr: *const Value,
        return_address: *const Instruction,
        return_register: i16,
    ) -> Self {
        Self {
            size,
            registers_ptr,
            constants_ptr,
            return_address,
            return_register,
        }
    }
}

type InstructionHandler = fn(&mut VMContext, ip: *const Instruction);

const OPCODE_HANDLERS: [InstructionHandler; 22] = [
    opcode_add,           // 0
    opcode_subtract,      // 1
    opcode_multiply,      // 2
    opcode_divide,        // 3
    opcode_modulo,        // 4
    opcode_equal,         // 5
    opcode_not_equal,     // 6
    opcode_greater,       // 7
    opcode_greater_equal, // 8
    opcode_less,          // 9
    opcode_less_equal,    // 10
    opcode_negate,        // 11
    opcode_not,           // 12
    opcode_move,          // 13
    opcode_call,          // 14
    opcode_return,        // 15
    opcode_return_void,   // 16
    opcode_jump,          // 17
    opcode_jump_if_true,  // 18
    opcode_jump_if_false, // 19
    opcode_print,         // 20
    opcode_halt,          // 21
];

pub struct VMContext<'a> {
    pub functions: &'a [Function],
    pub call_stack: Vec<FunctionFrame>,
    pub registers: Vec<Value>,
    pub frame_size: u8,
    pub registers_ptr: *mut Value,
    pub constants_ptr: *const Value,
}

macro_rules! dispatch {
    ($ctx:expr, $ip: expr, $op_code:expr) => {
        let _: &mut VMContext = $ctx;
        let _: *const Instruction = $ip;

        become OPCODE_HANDLERS[$op_code]($ctx, $ip)
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

impl<'a> VMContext<'a> {
    pub fn new(
        functions: &'a [Function],
        registers: Vec<Value>,
        frame_size: u8,
        registers_ptr: *mut Value,
        constants_ptr: *const Value,
        main_frame: FunctionFrame,
    ) -> Self {
        Self {
            functions,
            call_stack: vec![main_frame],
            registers,
            frame_size,
            registers_ptr,
            constants_ptr,
        }
    }

    #[inline(always)]
    fn get_value(&self, register: i16) -> Value {
        unsafe {
            if register < 0 {
                *self.constants_ptr.add(-(register + 1) as usize)
            } else {
                *self.registers_ptr.add(register as usize)
            }
        }
    }

    #[inline(always)]
    fn set_value(&mut self, register: i16, value: Value) {
        unsafe {
            *self.registers_ptr.add(register as usize) = value;
        }
    }

    #[inline(always)]
    fn pop_frame(&mut self) -> FunctionFrame {
        let frame = unsafe { self.call_stack.pop().unwrap_unchecked() };

        if let Some(frame) = self.call_stack.last() {
            self.registers_ptr = frame.registers_ptr;
            self.constants_ptr = frame.constants_ptr;
        }

        frame
    }

    #[inline(always)]
    fn push_frame(
        &mut self,
        return_register: i16,
        return_address: *const Instruction,
        frame_size: u8,
        constants_ptr: *const Value,
    ) {
        let size = self.call_stack.last().unwrap().size;

        let registers_ptr = unsafe { self.registers_ptr.add(size as usize) };

        let frame = FunctionFrame::new(
            frame_size,
            registers_ptr,
            constants_ptr,
            return_address,
            return_register,
        );

        self.registers_ptr = registers_ptr;

        self.call_stack.push(frame);
    }
}

pub fn run_kaori_vm(instructions: Vec<Instruction>, functions: Vec<Function>) {
    let mut registers = vec![Value::default(); 1024];
    let Function {
        ip,
        frame_size,
        ref constants,
    } = functions[0];

    let registers_ptr = registers.as_mut_ptr();
    let constants_ptr = (*constants).as_ptr();

    let return_address = unsafe { instructions.as_ptr().add(instructions.len() - 1) };
    let main_frame =
        FunctionFrame::new(frame_size, registers_ptr, constants_ptr, return_address, 0);

    let mut ctx = VMContext::new(
        &functions,
        registers,
        frame_size,
        registers_ptr,
        constants_ptr,
        main_frame,
    );

    let op_code = unsafe { (*ip).discriminant() };

    OPCODE_HANDLERS[op_code](&mut ctx, ip);
}

#[inline(never)]
fn opcode_move(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_add(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_subtract(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_multiply(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_divide(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_modulo(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_equal(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_not_equal(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_greater(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_greater_equal(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_less(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_less_equal(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_negate(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_not(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_jump(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Jump { offset } = *ip else {
            unreachable_unchecked();
        };

        dispatch_to!(ctx, ip, offset as isize);
    }
}

#[inline(never)]
fn opcode_jump_if_true(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_jump_if_false(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_call(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Call {
            dest: return_register,
            src,
        } = *ip
        else {
            unreachable_unchecked();
        };

        let return_address = ip.add(1);

        let function_index = ctx.get_value(src).as_function();

        let Function {
            ip,
            frame_size,
            ref constants,
        } = ctx.functions[function_index];
        let constants_ptr = (*constants).as_ptr();

        let op_code = (*ip).discriminant();

        ctx.push_frame(return_register, return_address, frame_size, constants_ptr);

        ctx.constants_ptr = constants_ptr;

        dispatch!(ctx, ip, op_code);
    }
}

#[inline(never)]
fn opcode_return(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Return { src } = *ip else {
            unreachable_unchecked();
        };

        let value = ctx.get_value(src);

        let FunctionFrame {
            return_address: ip,
            return_register: dest,
            ..
        } = ctx.pop_frame();

        ctx.set_value(dest, value);

        let op_code = (*ip).discriminant();

        dispatch!(ctx, ip, op_code);
    }
}

#[inline(never)]
fn opcode_return_void(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_print(ctx: &mut VMContext, ip: *const Instruction) {
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
fn opcode_halt(_ctx: &mut VMContext, _ip: *const Instruction) {}

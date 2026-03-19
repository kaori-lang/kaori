use crate::{
    bytecode::{function::Function, value::Value},
    virtual_machine::vm_context::FunctionFrame,
};

use super::vm_context::VMContext;

type InstructionHandler = fn(&mut VMContext, ip: *const i16);

const OPCODE_HANDLERS: [InstructionHandler; 22] = [
    opcode_add,
    opcode_subtract,
    opcode_multiply,
    opcode_divide,
    opcode_modulo,
    opcode_equal,
    opcode_not_equal,
    opcode_greater,
    opcode_greater_equal,
    opcode_less,
    opcode_less_equal,
    opcode_negate,
    opcode_not,
    opcode_move,
    opcode_call,
    opcode_return,
    opcode_return_void,
    opcode_jump,
    opcode_jump_if_true,
    opcode_jump_if_false,
    opcode_print,
    opcode_halt,
];

pub fn run_vm(bytes: Vec<i16>, functions: Vec<Function>) {
    let mut registers = vec![Value::default(); 1024];
    let Function {
        ip,
        frame_size,
        ref constants,
    } = functions[0];

    let registers_ptr = registers.as_mut_ptr();
    let constants_ptr = (*constants).as_ptr();

    let return_address = unsafe { bytes.as_ptr().add(bytes.len() - 1) };
    let main_frame =
        FunctionFrame::new(frame_size, registers_ptr, constants_ptr, return_address, 0);

    let mut ctx = VMContext::new(
        &functions,
        registers,
        registers_ptr,
        constants_ptr,
        main_frame,
    );

    let op_code = unsafe { *ip };
    OPCODE_HANDLERS[op_code as usize](&mut ctx, ip)
}

macro_rules! dispatch {
    ($ctx:expr, $ip: expr) => {{
        let _: &mut VMContext = $ctx;
        let _: *const i16 = $ip;
        let op_code: i16 = *$ip;

        become OPCODE_HANDLERS[op_code as usize]($ctx, $ip);
    }};
}

macro_rules! dispatch_to {
    ($ctx:expr, $ip:expr, $offset: expr) => {{
        let _: &mut VMContext = $ctx;
        let _: *const i16 = $ip;
        let _: i16 = $offset;

        let offset = $offset as i16;

        let ip = $ip.offset(offset as isize);
        dispatch!($ctx, ip);
    }};
}

#[inline(never)]
fn opcode_move(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let dest = *ip.add(1);
        let src = *ip.add(2);

        let value = ctx.get_value(src);
        ctx.set_value(dest, value);

        let ip = ip.add(3);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_add(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs + rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_subtract(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs - rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_multiply(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs * rhs));

        dispatch!(ctx, ip.add(4));
    }
}

#[inline(never)]
fn opcode_divide(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs / rhs));

        dispatch!(ctx, ip.add(4));
    }
}

#[inline(never)]
fn opcode_modulo(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs % rhs));

        dispatch!(ctx, ip.add(4));
    }
}

#[inline(never)]
fn opcode_equal(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs == rhs));

        dispatch!(ctx, ip.add(4));
    }
}

#[inline(never)]
fn opcode_greater(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs > rhs));

        dispatch!(ctx, ip.add(4));
    }
}

#[inline(never)]
fn opcode_greater_equal(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs >= rhs));

        dispatch!(ctx, ip.add(4));
    }
}

#[inline(never)]
fn opcode_less(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs < rhs));

        dispatch!(ctx, ip.add(4));
    }
}

#[inline(never)]
fn opcode_less_equal(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs <= rhs));

        dispatch!(ctx, ip.add(4));
    }
}

#[inline(never)]
fn opcode_not_equal(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs != rhs));

        dispatch!(ctx, ip.add(4));
    }
}

#[inline(never)]
fn opcode_negate(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let dest = *ip.add(1);
        let src = *ip.add(2);

        let value = ctx.get_value(src).as_number();
        ctx.set_value(dest, Value::number(-value));

        let ip = ip.add(3);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_not(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let dest = *ip.add(1);
        let src = *ip.add(2);

        let value = ctx.get_value(src).as_boolean();
        ctx.set_value(dest, Value::boolean(!value));

        let ip = ip.add(3);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_call(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let dest = *ip.add(1);
        let src = *ip.add(2);

        let return_address = ip.add(3);

        let function_index = ctx.get_value(src).as_function();

        let Function {
            ip,
            frame_size,
            ref constants,
        } = ctx.functions[function_index];
        let constants_ptr = (*constants).as_ptr();

        ctx.push_frame(dest, return_address, frame_size, constants_ptr);

        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_return(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let src = *ip.add(1);
        let value = ctx.get_value(src);

        let FunctionFrame {
            return_address: ip,
            return_register: dest,
            ..
        } = ctx.pop_frame();

        ctx.set_value(dest, value);

        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_return_void(ctx: &mut VMContext, _ip: *const i16) {
    unsafe {
        let frame = ctx.pop_frame();
        let ip = frame.return_address;

        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_jump(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let offset = *ip.add(1);

        dispatch_to!(ctx, ip, offset);
    }
}

#[inline(never)]
fn opcode_jump_if_true(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let src = *ip.add(1);
        let offset = *ip.add(2);

        match ctx.get_value(src).as_boolean() {
            true => {
                dispatch_to!(ctx, ip, offset);
            }
            false => {
                let ip = ip.add(3);
                dispatch!(ctx, ip);
            }
        }
    }
}

#[inline(never)]
fn opcode_jump_if_false(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let src = *ip.add(1);
        let offset = *ip.add(2);

        match ctx.get_value(src).as_boolean() {
            true => {
                let ip = ip.add(3);
                dispatch!(ctx, ip);
            }
            false => {
                dispatch_to!(ctx, ip, offset);
            }
        }
    }
}

#[inline(never)]
fn opcode_print(ctx: &mut VMContext, ip: *const i16) {
    unsafe {
        let src = *ip.add(1);

        let value = ctx.get_value(src).as_number();

        println!("{}", value);

        let ip = ip.add(2);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_halt(_ctx: &mut VMContext, _ip: *const i16) {}

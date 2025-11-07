use crate::{
    bytecode::{function::Function, value::Value},
    virtual_machine::vm_context::FunctionFrame,
};

use super::vm_context::VMContext;

type InstructionHandler = fn(&mut VMContext, ip: *const u16);

const OPCODE_HANDLERS: [InstructionHandler; 63] = [
    opcode_add_rr,           // 0
    opcode_add_rk,           // 1
    opcode_add_kr,           // 2
    opcode_add_kk,           // 3
    opcode_subtract_rr,      // 4
    opcode_subtract_rk,      // 5
    opcode_subtract_kr,      // 6
    opcode_subtract_kk,      // 7
    opcode_multiply_rr,      // 8
    opcode_multiply_rk,      // 9
    opcode_multiply_kr,      // 10
    opcode_multiply_kk,      // 11
    opcode_divide_rr,        // 12
    opcode_divide_rk,        // 13
    opcode_divide_kr,        // 14
    opcode_divide_kk,        // 15
    opcode_modulo_rr,        // 16
    opcode_modulo_rk,        // 17
    opcode_modulo_kr,        // 18
    opcode_modulo_kk,        // 19
    opcode_equal_rr,         // 20
    opcode_equal_rk,         // 21
    opcode_equal_kr,         // 22
    opcode_equal_kk,         // 23
    opcode_not_equal_rr,     // 24
    opcode_not_equal_rk,     // 25
    opcode_not_equal_kr,     // 26
    opcode_not_equal_kk,     // 27
    opcode_greater_rr,       // 28
    opcode_greater_rk,       // 29
    opcode_greater_kr,       // 30
    opcode_greater_kk,       // 31
    opcode_greater_equal_rr, // 32
    opcode_greater_equal_rk, // 33
    opcode_greater_equal_kr, // 34
    opcode_greater_equal_kk, // 35
    opcode_less_rr,          // 36
    opcode_less_rk,          // 37
    opcode_less_kr,          // 38
    opcode_less_kk,          // 39
    opcode_less_equal_rr,    // 40
    opcode_less_equal_rk,    // 41
    opcode_less_equal_kr,    // 42
    opcode_less_equal_kk,    // 43
    opcode_negate_r,         // 44
    opcode_negate_k,         // 45
    opcode_not_r,            // 46
    opcode_not_k,            // 47
    opcode_move_r,           // 48
    opcode_move_k,           // 49
    opcode_call_r,           // 50
    opcode_call_k,           // 51
    opcode_return_r,         // 52
    opcode_return_k,         // 53
    opcode_return_void,      // 54
    opcode_jump,             // 55
    opcode_jump_if_true_r,   // 56
    opcode_jump_if_true_k,   // 57
    opcode_jump_if_false_r,  // 58
    opcode_jump_if_false_k,  // 59
    opcode_print_r,          // 60
    opcode_print_k,          // 61
    opcode_halt,             // 62
];

pub fn run_vm(bytes: Vec<u16>, functions: Vec<Function>) {
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
        let _: *const u16 = $ip;
        let op_code: u16 = *$ip;

        become OPCODE_HANDLERS[op_code as usize]($ctx, $ip);
    }};
}

macro_rules! dispatch_to {
    ($ctx:expr, $ip:expr, $offset: expr) => {{
        let _: &mut VMContext = $ctx;
        let _: *const u16 = $ip;
        let _: u16 = $offset;

        let offset = $offset as i16;

        let ip = $ip.offset(offset as isize);
        dispatch!($ctx, ip);
    }};
}

#[inline(never)]
fn opcode_move_r(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src = *ip.add(2);

        let value = ctx.get_register_value(src);
        ctx.set_value(dest, value);

        let ip = ip.add(3);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_move_k(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src = *ip.add(2);

        let value = ctx.get_constant_value(src);
        ctx.set_value(dest, value);

        let ip = ip.add(3);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_add_rr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs + rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_add_rk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs + rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_add_kr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs + rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_add_kk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs + rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_subtract_rr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs - rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_subtract_rk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs - rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_subtract_kr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs - rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_subtract_kk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs - rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_multiply_rr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs * rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_multiply_rk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs * rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_multiply_kr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs * rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_multiply_kk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs * rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_divide_rr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs / rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_divide_rk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs / rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_divide_kr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs / rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_divide_kk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs / rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_modulo_rr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs % rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_modulo_rk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs % rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_modulo_kr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs % rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_modulo_kk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs % rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_equal_rr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs == rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_equal_rk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs == rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_equal_kr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs == rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_equal_kk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs == rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_not_equal_rr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs != rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_not_equal_rk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs != rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_not_equal_kr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs != rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_not_equal_kk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs != rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_greater_rr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs > rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_greater_rk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs > rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_greater_kr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs > rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_greater_kk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs > rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_greater_equal_rr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs >= rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_greater_equal_rk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs >= rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_greater_equal_kr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs >= rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_greater_equal_kk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs >= rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_less_rr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs < rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_less_rk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs < rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_less_kr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs < rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_less_kk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs < rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_less_equal_rr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs <= rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_less_equal_rk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs <= rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_less_equal_kr(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs <= rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_less_equal_kk(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src1 = *ip.add(2);
        let src2 = *ip.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs <= rhs));

        let ip = ip.add(4);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_negate_r(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src = *ip.add(2);

        let value = ctx.get_register_value(src).as_number();
        ctx.set_value(dest, Value::number(-value));

        let ip = ip.add(3);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_negate_k(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src = *ip.add(2);

        let value = ctx.get_constant_value(src).as_number();
        ctx.set_value(dest, Value::number(-value));

        let ip = ip.add(3);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_not_r(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src = *ip.add(2);

        let value = ctx.get_register_value(src).as_boolean();
        ctx.set_value(dest, Value::boolean(!value));

        let ip = ip.add(3);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_not_k(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src = *ip.add(2);

        let value = ctx.get_constant_value(src).as_boolean();
        ctx.set_value(dest, Value::boolean(!value));

        let ip = ip.add(3);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_call_r(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src = *ip.add(2);

        let return_address = ip.add(3);

        let function_index = ctx.get_register_value(src).as_function();

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
fn opcode_call_k(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let dest = *ip.add(1);
        let src = *ip.add(2);

        let return_address = ip.add(3);

        let function_index = ctx.get_constant_value(src).as_function();

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
fn opcode_return_r(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let src = *ip.add(1);
        let value = ctx.get_register_value(src);

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
fn opcode_return_k(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let src = *ip.add(1);
        let value = ctx.get_constant_value(src);

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
fn opcode_return_void(ctx: &mut VMContext, _ip: *const u16) {
    unsafe {
        let frame = ctx.pop_frame();
        let ip = frame.return_address;

        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_jump(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let offset = *ip.add(1);

        dispatch_to!(ctx, ip, offset);
    }
}

#[inline(never)]
fn opcode_jump_if_true_r(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let src = *ip.add(1);
        let offset = *ip.add(2);

        match ctx.get_register_value(src).as_boolean() {
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
fn opcode_jump_if_true_k(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let src = *ip.add(1);
        let offset = *ip.add(2);

        match ctx.get_constant_value(src).as_boolean() {
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
fn opcode_jump_if_false_r(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let src = *ip.add(1);
        let offset = *ip.add(2);

        match ctx.get_register_value(src).as_boolean() {
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
fn opcode_jump_if_false_k(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let src = *ip.add(1);
        let offset = *ip.add(2);

        match ctx.get_constant_value(src).as_boolean() {
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
fn opcode_print_r(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let src = *ip.add(1);
        let value = ctx.get_register_value(src).as_number();

        println!("{}", value);

        let ip = ip.add(2);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_print_k(ctx: &mut VMContext, ip: *const u16) {
    unsafe {
        let src = *ip.add(1);
        let value = ctx.get_constant_value(src).as_number();

        println!("{}", value);

        let ip = ip.add(2);
        dispatch!(ctx, ip);
    }
}

#[inline(never)]
fn opcode_halt(_ctx: &mut VMContext, _ip: *const u16) {}

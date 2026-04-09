use std::hint::unreachable_unchecked;

use crate::{
    bytecode::{function::Function, instruction::Instruction, value::Value},
    vm::{debug_value::DebugValue, vm_context::FunctionFrame},
};

use super::{gc::Gc, vm_context::VMContext};

type InstructionHandler = fn(&mut VMContext, ip: *const Instruction, gc: &mut Gc);

const OPCODE_HANDLERS: [InstructionHandler; 26] = [
    opcode_add,
    opcode_subtract,
    opcode_multiply,
    opcode_divide,
    opcode_modulo,
    opcode_power,
    opcode_equal,
    opcode_not_equal,
    opcode_greater,
    opcode_greater_equal,
    opcode_less,
    opcode_less_equal,
    opcode_negate,
    opcode_not,
    opcode_move,
    opcode_create_dict,
    opcode_set_field,
    opcode_get_field,
    opcode_call,
    opcode_return,
    opcode_return_void,
    opcode_jump,
    opcode_jump_if_true,
    opcode_jump_if_false,
    opcode_print,
    opcode_halt,
];

pub fn run_vm(functions: Vec<Function>, mut gc: Gc) {
    let mut registers = vec![Value::default(); 1024];
    let Function {
        instructions,
        registers_count,
        constant_pool,
    } = &functions[0];

    let registers_ptr = registers.as_mut_ptr();
    let constant_pool_ptr = (*constant_pool).as_ptr();

    let return_address = unsafe { instructions.last().unwrap_unchecked() };

    let main_frame = FunctionFrame::new(
        *registers_count,
        registers_ptr,
        constant_pool_ptr,
        return_address,
        0,
    );

    let mut ctx = VMContext::new(
        &functions,
        registers,
        registers_ptr,
        constant_pool_ptr,
        main_frame,
    );

    let index = instructions[0].discriminant();
    let entry = instructions.as_ptr();

    OPCODE_HANDLERS[index](&mut ctx, entry, &mut gc)
}

macro_rules! dispatch {
    ($ctx:expr, $ip: expr, $gc: expr) => {{
        let _: &mut VMContext = $ctx;
        let _: *const Instruction = $ip;

        let ip: *const Instruction = $ip.add(1);
        let index = (*ip).discriminant();

        become OPCODE_HANDLERS[index]($ctx, ip, $gc);
    }};
}

macro_rules! dispatch_to {
    ($ctx:expr, $ip:expr, $gc:expr, $offset: expr) => {{
        let _: &mut VMContext = $ctx;
        let _: *const Instruction = $ip;
        let _: i16 = $offset;

        let offset = $offset as i16;

        let ip: *const Instruction = $ip.offset(offset as isize);
        let index = (*ip).discriminant();

        become OPCODE_HANDLERS[index]($ctx, ip, $gc);
    }};
}

#[inline(never)]
fn opcode_move(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::Move { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let value = ctx.get_value(src);
        ctx.set_value(dest, value);

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_add(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::Add { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = ctx.get_value(src1).expect_number();
        let rhs = ctx.get_value(src2).expect_number();

        ctx.set_value(dest, Value::number(lhs + rhs));

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_subtract(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::Subtract { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = ctx.get_value(src1).expect_number();
        let rhs = ctx.get_value(src2).expect_number();

        ctx.set_value(dest, Value::number(lhs - rhs));

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_multiply(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::Multiply { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = ctx.get_value(src1).expect_number();
        let rhs = ctx.get_value(src2).expect_number();

        ctx.set_value(dest, Value::number(lhs * rhs));

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_divide(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::Divide { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = ctx.get_value(src1).expect_number();
        let rhs = ctx.get_value(src2).expect_number();

        ctx.set_value(dest, Value::number(lhs / rhs));

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_modulo(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::Modulo { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = ctx.get_value(src1).expect_number();
        let rhs = ctx.get_value(src2).expect_number();

        ctx.set_value(dest, Value::number(lhs % rhs));

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_power(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::Power { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = ctx.get_value(src1).expect_number();
        let rhs = ctx.get_value(src2).expect_number();

        ctx.set_value(dest, Value::number(lhs.powf(rhs)));

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_equal(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::Equal { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = ctx.get_value(src1).expect_number();
        let rhs = ctx.get_value(src2).expect_number();

        ctx.set_value(dest, Value::boolean(lhs == rhs));

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_not_equal(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::NotEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = ctx.get_value(src1).expect_number();
        let rhs = ctx.get_value(src2).expect_number();

        ctx.set_value(dest, Value::boolean(lhs != rhs));

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_greater(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::Greater { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = ctx.get_value(src1).expect_number();
        let rhs = ctx.get_value(src2).expect_number();

        ctx.set_value(dest, Value::boolean(lhs > rhs));

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_greater_equal(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::GreaterEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = ctx.get_value(src1).expect_number();
        let rhs = ctx.get_value(src2).expect_number();

        ctx.set_value(dest, Value::boolean(lhs >= rhs));

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_less(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::Less { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = ctx.get_value(src1).expect_number();
        let rhs = ctx.get_value(src2).expect_number();

        ctx.set_value(dest, Value::boolean(lhs < rhs));

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_less_equal(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::LessEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = ctx.get_value(src1).expect_number();
        let rhs = ctx.get_value(src2).expect_number();

        ctx.set_value(dest, Value::boolean(lhs <= rhs));

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_negate(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::Negate { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let value = ctx.get_value(src).expect_number();
        ctx.set_value(dest, Value::number(-value));

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_not(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::Not { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let value = ctx.get_value(src).expect_boolean();
        ctx.set_value(dest, Value::boolean(!value));

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_call(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::Call { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let return_address = ip.add(1);

        let function_index = ctx.get_value(src).expect_function();

        let Function {
            instructions,
            registers_count,
            constant_pool,
        } = &ctx.functions[function_index];
        let constants_ptr = (*constant_pool).as_ptr();

        ctx.push_frame(dest, return_address, *registers_count, constants_ptr);

        let index = instructions[0].discriminant();
        let ip = instructions.as_ptr();

        become OPCODE_HANDLERS[index](ctx, ip, gc)
    }
}

#[inline(never)]
fn opcode_return(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::Return { src } = *ip else {
            unreachable_unchecked()
        };

        let value = ctx.get_value(src);

        let FunctionFrame {
            return_address: ip,
            return_register: dest,
            ..
        } = ctx.pop_frame();

        ctx.set_value(dest, value);

        let index = (*ip).discriminant();

        become OPCODE_HANDLERS[index](ctx, ip, gc)
    }
}

#[inline(never)]
fn opcode_return_void(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let FunctionFrame {
            return_address: ip, ..
        } = ctx.pop_frame();

        let index = (*ip).discriminant();

        become OPCODE_HANDLERS[index](ctx, ip, gc)
    }
}

#[inline(never)]
fn opcode_jump(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::Jump { offset } = *ip else {
            unreachable_unchecked()
        };

        dispatch_to!(ctx, ip, gc, offset);
    }
}

#[inline(never)]
fn opcode_jump_if_true(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::JumpIfTrue { src, offset } = *ip else {
            unreachable_unchecked()
        };

        match ctx.get_value(src).expect_boolean() {
            true => dispatch_to!(ctx, ip, gc, offset),
            false => dispatch!(ctx, ip, gc),
        }
    }
}

#[inline(never)]
fn opcode_jump_if_false(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::JumpIfFalse { src, offset } = *ip else {
            unreachable_unchecked()
        };

        match ctx.get_value(src).expect_boolean() {
            true => dispatch!(ctx, ip, gc),
            false => dispatch_to!(ctx, ip, gc, offset),
        }
    }
}

#[inline(never)]
fn opcode_print(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::Print { src } = *ip else {
            unreachable_unchecked()
        };

        let value = ctx.get_value(src);
        let debug_value = DebugValue { value, gc };

        println!("{:?}", debug_value);

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_create_dict(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::CreateDict { dest } = *ip else {
            unreachable_unchecked()
        };

        let value = gc.allocate_dict();

        ctx.set_value(dest, value);

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_set_field(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::SetField { object, key, value } = *ip else {
            unreachable_unchecked()
        };

        let key = ctx.get_value(key);
        let value = ctx.get_value(value);

        let dict_ref = ctx.get_value(object as i16);
        let dict = gc.get_mut_dict(dict_ref);

        dict.insert(key, value);

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_get_field(ctx: &mut VMContext, ip: *const Instruction, gc: &mut Gc) {
    unsafe {
        let Instruction::GetField { dest, object, key } = *ip else {
            unreachable_unchecked()
        };

        let object = ctx.get_value(object);
        let key = ctx.get_value(key);

        let dict = gc.get_dict(object);

        let value = dict.get(&key).unwrap();

        ctx.set_value(dest, *value);

        dispatch!(ctx, ip, gc);
    }
}

#[inline(never)]
fn opcode_halt(_ctx: &mut VMContext, _ip: *const Instruction, gc: &mut Gc) {}

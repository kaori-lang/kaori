use std::hint::unreachable_unchecked;

use crate::{
    bytecode::instruction::Instruction,
    runtime::{debug_value::DebugValue, value::Value},
};

use super::{function::Function, gc::Gc};

type Handler = fn(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
);

const OPCODE_HANDLERS: [Handler; 25] = [
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
];

pub struct StackFrame {
    pub registers_count: u8,
    pub return_address: *const Instruction,
    pub return_register: u16,
    pub registers: *mut Value,
    pub constants: *const Value,
}

pub fn run_vm(functions: Vec<Function>, mut gc: Gc) {
    let mut registers = vec![Value::default(); 1024];
    let Function {
        ref instructions,
        registers_count,
        ref constant_pool,
    } = functions[0];

    let registers = registers.as_mut_ptr();
    let constants = (*constant_pool).as_ptr();

    let return_address = unsafe { instructions.last().unwrap_unchecked() };

    let main_frame = StackFrame {
        registers_count,
        return_address,
        return_register: 0,
        registers,
        constants,
    };

    let mut stack = vec![main_frame];

    let index = instructions[0].discriminant();
    let entry = instructions.as_ptr();

    OPCODE_HANDLERS[index](&functions, &mut stack, entry, &mut gc, registers, constants)
}

macro_rules! dispatch_next {
    ($functions:expr, $stack:expr, $ip:expr, $gc:expr, $registers:expr, $constants:expr) => {{
        let functions: &[Function] = $functions;
        let stack: &mut Vec<StackFrame> = $stack;
        let ip: *const Instruction = $ip.add(1);
        let gc: &mut Gc = $gc;
        let registers: *mut Value = $registers;
        let constants: *const Value = $constants;
        let index = (*ip).discriminant();
        become OPCODE_HANDLERS[index](functions, stack, ip, gc, registers, constants);
    }};
}

macro_rules! dispatch_offset {
    ($functions:expr, $stack:expr, $ip:expr, $gc:expr, $registers:expr, $constants:expr, $offset:expr) => {{
        let functions: &[Function] = $functions;
        let stack: &mut Vec<StackFrame> = $stack;
        let ip: *const Instruction = $ip.offset($offset as i16 as isize);
        let gc: &mut Gc = $gc;
        let registers: *mut Value = $registers;
        let constants: *const Value = $constants;
        let index = (*ip).discriminant();
        become OPCODE_HANDLERS[index](functions, stack, ip, gc, registers, constants);
    }};
}

macro_rules! dispatch_to {
    ($functions:expr, $stack:expr, $ip:expr, $gc:expr, $registers:expr, $constants:expr) => {{
        let functions: &[Function] = $functions;
        let stack: &mut Vec<StackFrame> = $stack;
        let ip: *const Instruction = $ip;
        let gc: &mut Gc = $gc;
        let registers: *mut Value = $registers;
        let constants: *const Value = $constants;
        let index = (*ip).discriminant();
        become OPCODE_HANDLERS[index](functions, stack, ip, gc, registers, constants);
    }};
}

#[inline(always)]
fn get_value(index: i16, registers: *mut Value, constants: *const Value) -> Value {
    unsafe {
        if index < 0 {
            *constants.add(-(index + 1) as usize)
        } else {
            *registers.add(index as usize)
        }
    }
}

#[inline(always)]
fn set_value(index: u16, value: Value, registers: *mut Value) {
    unsafe {
        *registers.add(index as usize) = value;
    }
}

#[inline(never)]
fn opcode_move(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::Move { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let value = get_value(src, registers, constants);
        set_value(dest, value, registers);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_add(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::Add { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();

        set_value(dest, Value::number(lhs + rhs), registers);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_subtract(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::Subtract { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();

        set_value(dest, Value::number(lhs - rhs), registers);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_multiply(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::Multiply { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();

        set_value(dest, Value::number(lhs * rhs), registers);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_divide(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::Divide { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();

        set_value(dest, Value::number(lhs / rhs), registers);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_modulo(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::Modulo { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();

        set_value(dest, Value::number(lhs % rhs), registers);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_power(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::Power { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();

        set_value(dest, Value::number(lhs.powf(rhs)), registers);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_equal(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::Equal { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();

        set_value(dest, Value::boolean(lhs == rhs), registers);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_not_equal(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::NotEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();

        set_value(dest, Value::boolean(lhs != rhs), registers);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_greater(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::Greater { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();

        set_value(dest, Value::boolean(lhs > rhs), registers);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_greater_equal(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::GreaterEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();

        set_value(dest, Value::boolean(lhs >= rhs), registers);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_less(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::Less { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();

        set_value(dest, Value::boolean(lhs < rhs), registers);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_less_equal(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::LessEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();

        set_value(dest, Value::boolean(lhs <= rhs), registers);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_negate(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::Negate { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let value = get_value(src, registers, constants).expect_number();
        set_value(dest, Value::number(-value), registers);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_not(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::Not { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let value = get_value(src, registers, constants).expect_boolean();
        set_value(dest, Value::boolean(!value), registers);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_call(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::Call { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let return_address = ip.add(1);
        let function_index = get_value(src, registers, constants).expect_function();

        let Function {
            ref instructions,
            registers_count,
            ref constant_pool,
        } = functions[function_index];

        let current_registers_count = stack.last().unwrap_unchecked().registers_count;
        let registers = registers.add(current_registers_count as usize);
        let constants = (*constant_pool).as_ptr();

        let function_frame = StackFrame {
            registers_count,
            return_address,
            return_register: dest,
            registers,
            constants,
        };

        stack.push(function_frame);

        let ip = instructions.as_ptr();

        dispatch_to!(functions, stack, ip, gc, registers, constants)
    }
}

#[inline(never)]
fn opcode_return(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::Return { src } = *ip else {
            unreachable_unchecked()
        };

        let value = get_value(src, registers, constants);

        let StackFrame {
            return_address: ip,
            return_register: dest,
            ..
        } = stack.pop().unwrap_unchecked();

        if let Some(StackFrame {
            registers,
            constants,
            ..
        }) = stack.last()
        {
            let registers: *mut Value = *registers;
            let constants: *const Value = *constants;

            set_value(dest, value, registers);

            dispatch_to!(functions, stack, ip, gc, registers, constants)
        }
    }
}

#[inline(never)]
fn opcode_return_void(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    _registers: *mut Value,
    _constants: *const Value,
) {
    unsafe {
        let Instruction::ReturnVoid = *ip else {
            unreachable_unchecked()
        };

        let StackFrame {
            return_address: ip, ..
        } = stack.pop().unwrap_unchecked();

        if let Some(StackFrame {
            registers,
            constants,
            ..
        }) = stack.last()
        {
            let registers: *mut Value = *registers;
            let constants: *const Value = *constants;

            dispatch_to!(functions, stack, ip, gc, registers, constants)
        }
    }
}

#[inline(never)]
fn opcode_jump(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::Jump { offset } = *ip else {
            unreachable_unchecked()
        };

        dispatch_offset!(functions, stack, ip, gc, registers, constants, offset);
    }
}

#[inline(never)]
fn opcode_jump_if_true(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::JumpIfTrue { src, offset } = *ip else {
            unreachable_unchecked()
        };

        match get_value(src, registers, constants).expect_boolean() {
            true => dispatch_offset!(functions, stack, ip, gc, registers, constants, offset),
            false => dispatch_next!(functions, stack, ip, gc, registers, constants),
        }
    }
}

#[inline(never)]
fn opcode_jump_if_false(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::JumpIfFalse { src, offset } = *ip else {
            unreachable_unchecked()
        };

        match get_value(src, registers, constants).expect_boolean() {
            true => dispatch_next!(functions, stack, ip, gc, registers, constants),
            false => dispatch_offset!(functions, stack, ip, gc, registers, constants, offset),
        }
    }
}

#[inline(never)]
fn opcode_print(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::Print { src } = *ip else {
            unreachable_unchecked()
        };

        let value = get_value(src, registers, constants);
        let debug_value = DebugValue { value, gc };

        println!("{:?}", debug_value);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_create_dict(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::CreateDict { dest } = *ip else {
            unreachable_unchecked()
        };

        let value = gc.allocate_dict();
        set_value(dest, value, registers);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_set_field(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::SetField { object, key, value } = *ip else {
            unreachable_unchecked()
        };

        let key = get_value(key, registers, constants);
        let value = get_value(value, registers, constants);
        let object = get_value(object as i16, registers, constants);

        gc.get_mut_dict(object).insert(key, value);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

#[inline(never)]
fn opcode_get_field(
    functions: &[Function],
    stack: &mut Vec<StackFrame>,
    ip: *const Instruction,
    gc: &mut Gc,
    registers: *mut Value,
    constants: *const Value,
) {
    unsafe {
        let Instruction::GetField { dest, object, key } = *ip else {
            unreachable_unchecked()
        };

        let object = get_value(object, registers, constants);
        let key = get_value(key, registers, constants);

        let value = gc.get_dict(object).get(&key).unwrap();

        set_value(dest, *value, registers);

        dispatch_next!(functions, stack, ip, gc, registers, constants);
    }
}

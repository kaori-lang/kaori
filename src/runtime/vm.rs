use std::hint::unreachable_unchecked;

use super::gc::Gc;
use crate::bytecode::Function;
use crate::diagnostics::error::Error;

use crate::program::{CONSTANT_POOL, FUNCTIONS};
use crate::report_error;
use crate::runtime::debug_value::DebugValue;
use crate::runtime::gc::Closure;
use crate::{bytecode::instruction::Instruction, runtime::value::Value};

type Handler = fn(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>>;

macro_rules! dispatch_next {
    ($ip:expr, $registers:expr, $gc:expr, $size:expr) => {{
        let ip: *const Instruction = $ip.add(1);
        let index = (*ip).discriminant();
        become HANDLERS[index](ip, $registers, $gc, $size);
    }};
}

macro_rules! dispatch_offset {
    ($ip:expr, $registers:expr, $gc:expr, $offset:expr, $size:expr) => {{
        let ip: *const Instruction = $ip.offset($offset as isize);
        let index = (*ip).discriminant();
        become HANDLERS[index](ip, $registers, $gc, $size);
    }};
}

macro_rules! type_check {
    ($cond:expr, $($arg:tt)*) => {
        if  std::hint::unlikely(!$cond) {
            return Err(Box::new(report_error!($($arg)*)));
        }
    };
}

const REGISTER: u8 = 0;
const IMMEDIATE: u8 = 1;

const HANDLERS: [Handler; 55] = [
    opcode_add::<REGISTER, REGISTER>,
    opcode_add::<REGISTER, IMMEDIATE>,
    opcode_subtract::<REGISTER, REGISTER>,
    opcode_subtract::<REGISTER, IMMEDIATE>,
    opcode_subtract::<IMMEDIATE, REGISTER>,
    opcode_multiply::<REGISTER, REGISTER>,
    opcode_multiply::<REGISTER, IMMEDIATE>,
    opcode_divide::<REGISTER, REGISTER>,
    opcode_divide::<REGISTER, IMMEDIATE>,
    opcode_divide::<IMMEDIATE, REGISTER>,
    opcode_modulo::<REGISTER, REGISTER>,
    opcode_modulo::<REGISTER, IMMEDIATE>,
    opcode_modulo::<IMMEDIATE, REGISTER>,
    opcode_equal::<REGISTER, REGISTER>,
    opcode_equal::<REGISTER, IMMEDIATE>,
    opcode_not_equal::<REGISTER, REGISTER>,
    opcode_not_equal::<REGISTER, IMMEDIATE>,
    opcode_less::<REGISTER, REGISTER>,
    opcode_less::<REGISTER, IMMEDIATE>,
    opcode_less_equal::<REGISTER, REGISTER>,
    opcode_less_equal::<REGISTER, IMMEDIATE>,
    opcode_greater::<REGISTER, REGISTER>,
    opcode_greater::<REGISTER, IMMEDIATE>,
    opcode_greater_equal::<REGISTER, REGISTER>,
    opcode_greater_equal::<REGISTER, IMMEDIATE>,
    opcode_not,
    opcode_negate,
    opcode_move,
    opcode_move_arg,
    opcode_load_k,
    opcode_load_imm,
    opcode_create_dict,
    opcode_set_field::<REGISTER>,
    opcode_set_field::<IMMEDIATE>,
    opcode_get_field,
    opcode_create_closure,
    opcode_nop,
    opcode_call,
    opcode_return,
    opcode_jump,
    opcode_jump_if_false,
    opcode_jump_if_true,
    opcode_jump_if_less::<REGISTER, REGISTER>,
    opcode_jump_if_less::<REGISTER, IMMEDIATE>,
    opcode_jump_if_less_equal::<REGISTER, REGISTER>,
    opcode_jump_if_less_equal::<REGISTER, IMMEDIATE>,
    opcode_jump_if_greater::<REGISTER, REGISTER>,
    opcode_jump_if_greater::<REGISTER, IMMEDIATE>,
    opcode_jump_if_greater_equal::<REGISTER, REGISTER>,
    opcode_jump_if_greater_equal::<REGISTER, IMMEDIATE>,
    opcode_jump_if_equal::<REGISTER, REGISTER>,
    opcode_jump_if_equal::<REGISTER, IMMEDIATE>,
    opcode_jump_if_not_equal::<REGISTER, REGISTER>,
    opcode_jump_if_not_equal::<REGISTER, IMMEDIATE>,
    opcode_nop,
];

pub fn run_vm() -> Result<Value, Error> {
    let mut registers = vec![Value::default(); 4096];

    let Function {
        ref instructions,
        registers_count,
        ..
    } = FUNCTIONS.get().unwrap()[0];

    let mut gc = Gc::default();

    let registers = registers.as_mut_ptr();
    let ip = instructions.as_ptr();
    let index = unsafe { (*ip).discriminant() };

    let value = HANDLERS[index](ip, registers, &mut gc, registers_count).map_err(|e| *e)?;

    println!("{:?}", DebugValue::new(value, &gc));

    Ok(value)
}

unsafe fn set_value(dest: u8, value: Value, registers: *mut Value) {
    unsafe {
        *registers.add(dest as usize) = value;
    }
}

#[inline(never)]
fn opcode_add<const SRC1: u8, const SRC2: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::Add { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = *registers.add(src2 as usize);
                (dest, src1, src2)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::AddI { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = Value::number(src2.decode());
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot add, both operands must be numbers",
        );

        set_value(
            dest,
            Value::number(src1.as_number() + src2.as_number()),
            registers,
        );

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_subtract<const SRC1: u8, const SRC2: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::Subtract { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = *registers.add(src2 as usize);
                (dest, src1, src2)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::SubtractRI { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = Value::number(src2.decode());
                (dest, src1, src2)
            }
            (IMMEDIATE, REGISTER) => {
                let Instruction::SubtractIR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = Value::number(src1.decode());
                let src2 = *registers.add(src2 as usize);
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot subtract, both operands must be numbers",
        );

        set_value(
            dest,
            Value::number(src1.as_number() - src2.as_number()),
            registers,
        );

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_multiply<const SRC1: u8, const SRC2: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::Multiply { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = *registers.add(src2 as usize);
                (dest, src1, src2)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::MultiplyI { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = Value::number(src2.decode());
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot multiply, both operands must be numbers",
        );

        set_value(
            dest,
            Value::number(src1.as_number() * src2.as_number()),
            registers,
        );

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_divide<const SRC1: u8, const SRC2: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::Divide { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = *registers.add(src2 as usize);
                (dest, src1, src2)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::DivideRI { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = Value::number(src2.decode());
                (dest, src1, src2)
            }
            (IMMEDIATE, REGISTER) => {
                let Instruction::DivideIR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = Value::number(src1.decode());
                let src2 = *registers.add(src2 as usize);
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot divide, both operands must be numbers",
        );

        set_value(
            dest,
            Value::number(src1.as_number() / src2.as_number()),
            registers,
        );

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_modulo<const SRC1: u8, const SRC2: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::Modulo { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = *registers.add(src2 as usize);
                (dest, src1, src2)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::ModuloRI { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = Value::number(src2.decode());
                (dest, src1, src2)
            }
            (IMMEDIATE, REGISTER) => {
                let Instruction::ModuloIR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = Value::number(src1.decode());
                let src2 = *registers.add(src2 as usize);
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compute, both operands must be numbers",
        );

        set_value(
            dest,
            Value::number(src1.as_number() % src2.as_number()),
            registers,
        );

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_equal<const SRC1: u8, const SRC2: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::Equal { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = *registers.add(src2 as usize);
                (dest, src1, src2)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::EqualI { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = Value::number(src2.decode());
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        set_value(dest, Value::number((src1 == src2) as u8 as f64), registers);

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_not_equal<const SRC1: u8, const SRC2: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::NotEqual { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = *registers.add(src2 as usize);
                (dest, src1, src2)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::NotEqualI { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = Value::number(src2.decode());
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        set_value(dest, Value::number((src1 != src2) as u8 as f64), registers);

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_less<const SRC1: u8, const SRC2: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::Less { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = *registers.add(src2 as usize);
                (dest, src1, src2)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::LessI { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = Value::number(src2.decode());
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );

        set_value(
            dest,
            Value::number((src1.as_number() < src2.as_number()) as u8 as f64),
            registers,
        );

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_less_equal<const SRC1: u8, const SRC2: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::LessEqual { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = *registers.add(src2 as usize);
                (dest, src1, src2)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::LessEqualI { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = Value::number(src2.decode());
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );

        set_value(
            dest,
            Value::number((src1.as_number() <= src2.as_number()) as u8 as f64),
            registers,
        );

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_greater<const SRC1: u8, const SRC2: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::Greater { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = *registers.add(src2 as usize);
                (dest, src1, src2)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::GreaterI { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = Value::number(src2.decode());
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );

        set_value(
            dest,
            Value::number((src1.as_number() > src2.as_number()) as u8 as f64),
            registers,
        );

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_greater_equal<const SRC1: u8, const SRC2: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::GreaterEqual { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = *registers.add(src2 as usize);
                (dest, src1, src2)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::GreaterEqualI { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = Value::number(src2.decode());
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );

        set_value(
            dest,
            Value::number((src1.as_number() >= src2.as_number()) as u8 as f64),
            registers,
        );

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_not(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let Instruction::Not { dest, src } = *ip else {
            unreachable_unchecked()
        };
        let src = *registers.add(src as usize);

        type_check!(
            src.is_number(),
            "cannot apply not, operand must be a boolean",
        );

        set_value(
            dest,
            Value::number((src.as_number() == 0.0) as u8 as f64),
            registers,
        );

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_negate(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let Instruction::Negate { dest, src } = *ip else {
            unreachable_unchecked()
        };
        let src = *registers.add(src as usize);

        type_check!(src.is_number(), "cannot negate, operand must be a number",);

        set_value(dest, Value::number(-src.as_number()), registers);

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_move(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let Instruction::Move { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let src = *registers.add(src as usize);
        set_value(dest, src, registers);

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_move_arg(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let Instruction::MoveArg { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let src = *registers.add(src as usize);
        set_value(dest, src, registers);

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_load_k(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let Instruction::LoadK { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let constant = CONSTANT_POOL.get().unwrap_unchecked()[src as usize];
        set_value(dest, constant, registers);

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_load_imm(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let Instruction::LoadImm { dest, src } = *ip else {
            unreachable_unchecked()
        };
        set_value(dest, Value::number(src.decode()), registers);

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_create_dict(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let Instruction::CreateDict { dest } = *ip else {
            unreachable_unchecked()
        };
        let value = gc.allocate_dict();
        set_value(dest, value, registers);

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_set_field<const VALUE: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (object, key, value) = match VALUE {
            REGISTER => {
                let Instruction::SetField { object, key, value } = *ip else {
                    unreachable_unchecked()
                };
                let object = *registers.add(object as usize);
                let key = *registers.add(key as usize);
                let value = *registers.add(value as usize);
                (object, key, value)
            }
            IMMEDIATE => {
                let Instruction::SetFieldI { object, key, src } = *ip else {
                    unreachable_unchecked()
                };
                let object = *registers.add(object as usize);
                let key = *registers.add(key as usize);
                let value = Value::number(src.decode());
                (object, key, value)
            }
            _ => unreachable_unchecked(),
        };

        type_check!(object.is_dict(), "cannot set field, value is not a dict",);

        gc.get_mut_dict(object).insert(key, value);

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_get_field(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let Instruction::GetField { dest, object, key } = *ip else {
            unreachable_unchecked()
        };
        let object = *registers.add(object as usize);
        let key = *registers.add(key as usize);

        type_check!(object.is_dict(), "cannot get field, value is not a dict",);

        let value = gc.get_dict(object).get(&key).copied().unwrap_or_default();
        set_value(dest, value, registers);

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_create_closure(
    mut ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let Instruction::CreateClosure {
            dest,
            captures: captures_count,
            src,
        } = *ip
        else {
            unreachable_unchecked()
        };

        {
            let Function {
                ref instructions,
                registers_count,
                parameters,
            } = FUNCTIONS.get().unwrap_unchecked()[src as usize];

            let closure = Closure {
                instructions: instructions.as_ptr(),
                parameters,
                size: registers_count,
                captured: vec![Value::default(); captures_count as usize].into_boxed_slice(),
            };

            let closure = gc.allocate_closure(closure);

            set_value(dest, closure, registers);

            let mut captured_values = Vec::new();

            for _ in 0..captures_count {
                ip = ip.add(1);

                let Instruction::CaptureValue { src } = *ip else {
                    unreachable_unchecked()
                };

                let capture = *registers.add(src as usize);
                captured_values.push(capture);
            }

            let closure = gc.get_mut_closure(closure);
            closure.captured = captured_values.into_boxed_slice();
        }

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_call(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let Instruction::Call { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let src = *registers.add(src as usize);

        type_check!(src.is_function(), "cannot call, value is not a function",);

        let return_value = {
            let Closure {
                instructions: callee_ip,
                parameters,
                size: callee_size,
                ref captured,
            } = *gc.get_closure(src);

            let registers = registers.add(size as usize);

            for (dest, value) in captured.iter().copied().enumerate() {
                set_value(parameters + dest as u8, value, registers);
            }

            HANDLERS[(*callee_ip).discriminant()](callee_ip, registers, gc, callee_size)
        }?;

        set_value(dest, return_value, registers);

        dispatch_next!(ip, registers, gc, size)
    }
}

#[inline(never)]
fn opcode_return(
    ip: *const Instruction,
    registers: *mut Value,
    _gc: &mut Gc,
    _size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let Instruction::Return { src } = *ip else {
            unreachable_unchecked()
        };

        Ok(*registers.add(src as usize))
    }
}

#[inline(never)]
fn opcode_jump(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let Instruction::Jump { offset } = *ip else {
            unreachable_unchecked()
        };

        dispatch_offset!(ip, registers, gc, offset, size)
    }
}

#[inline(never)]
fn opcode_jump_if_false(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let Instruction::JumpIfFalse { src, offset } = *ip else {
            unreachable_unchecked()
        };

        let src = *registers.add(src as usize);

        type_check!(
            src.is_number(),
            "cannot use this as a condition, value must be a boolean",
        );

        if src.is_truthy() {
            dispatch_next!(ip, registers, gc, size)
        } else {
            dispatch_offset!(ip, registers, gc, offset, size)
        }
    }
}

#[inline(never)]
fn opcode_jump_if_true(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let Instruction::JumpIfTrue { src, offset } = *ip else {
            unreachable_unchecked()
        };

        let src = *registers.add(src as usize);

        type_check!(
            src.is_number(),
            "cannot use this as a condition, value must be a boolean",
        );

        if src.is_truthy() {
            dispatch_offset!(ip, registers, gc, offset, size)
        } else {
            dispatch_next!(ip, registers, gc, size)
        }
    }
}

#[inline(never)]
fn opcode_jump_if_less<const SRC1: u8, const SRC2: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (src1, src2, offset) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::JumpIfLess { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = *registers.add(src2 as usize);
                (src1, src2, offset)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::JumpIfLessI { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = Value::number(src2.decode());
                (src1, src2, offset)
            }
            _ => unreachable_unchecked(),
        };

        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );

        if src1.as_number() < src2.as_number() {
            dispatch_offset!(ip, registers, gc, offset, size)
        } else {
            dispatch_next!(ip, registers, gc, size)
        }
    }
}

#[inline(never)]
fn opcode_jump_if_less_equal<const SRC1: u8, const SRC2: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (src1, src2, offset) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::JumpIfLessEqual { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = *registers.add(src2 as usize);
                (src1, src2, offset)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::JumpIfLessEqualI { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = Value::number(src2.decode());
                (src1, src2, offset)
            }
            _ => unreachable_unchecked(),
        };

        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );

        if src1.as_number() <= src2.as_number() {
            dispatch_offset!(ip, registers, gc, offset, size)
        } else {
            dispatch_next!(ip, registers, gc, size)
        }
    }
}

#[inline(never)]
fn opcode_jump_if_greater<const SRC1: u8, const SRC2: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (src1, src2, offset) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::JumpIfGreater { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = *registers.add(src2 as usize);
                (src1, src2, offset)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::JumpIfGreaterI { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = Value::number(src2.decode());
                (src1, src2, offset)
            }
            _ => unreachable_unchecked(),
        };

        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );

        if src1.as_number() > src2.as_number() {
            dispatch_offset!(ip, registers, gc, offset, size)
        } else {
            dispatch_next!(ip, registers, gc, size)
        }
    }
}

#[inline(never)]
fn opcode_jump_if_greater_equal<const SRC1: u8, const SRC2: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (src1, src2, offset) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::JumpIfGreaterEqual { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = *registers.add(src2 as usize);
                (src1, src2, offset)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::JumpIfGreaterEqualI { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = Value::number(src2.decode());
                (src1, src2, offset)
            }
            _ => unreachable_unchecked(),
        };

        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );

        if src1.as_number() >= src2.as_number() {
            dispatch_offset!(ip, registers, gc, offset, size)
        } else {
            dispatch_next!(ip, registers, gc, size)
        }
    }
}

#[inline(never)]
fn opcode_jump_if_equal<const SRC1: u8, const SRC2: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (src1, src2, offset) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::JumpIfEqual { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = *registers.add(src2 as usize);
                (src1, src2, offset)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::JumpIfEqualI { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = Value::number(src2.decode());
                (src1, src2, offset)
            }
            _ => unreachable_unchecked(),
        };

        if src1 == src2 {
            dispatch_offset!(ip, registers, gc, offset, size)
        } else {
            dispatch_next!(ip, registers, gc, size)
        }
    }
}

#[inline(never)]
fn opcode_jump_if_not_equal<const SRC1: u8, const SRC2: u8>(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe {
        let (src1, src2, offset) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::JumpIfNotEqual { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = *registers.add(src2 as usize);
                (src1, src2, offset)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::JumpIfNotEqualI { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                let src1 = *registers.add(src1 as usize);
                let src2 = Value::number(src2.decode());
                (src1, src2, offset)
            }
            _ => unreachable_unchecked(),
        };

        if src1 != src2 {
            dispatch_offset!(ip, registers, gc, offset, size)
        } else {
            dispatch_next!(ip, registers, gc, size)
        }
    }
}

#[inline(never)]
fn opcode_nop(
    ip: *const Instruction,
    registers: *mut Value,
    gc: &mut Gc,
    size: u8,
) -> Result<Value, Box<Error>> {
    unsafe { dispatch_next!(ip, registers, gc, size) }
}

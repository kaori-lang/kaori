use std::hint::unreachable_unchecked;

use super::gc::Gc;
use crate::diagnostics::error::Error;

use crate::runtime::function::Function;
use crate::runtime::instruction::{Const, Instruction, Reg};
use crate::runtime::value::Value;
use crate::syntax::token::Span;
use crate::util::string_interner::Symbol;

type Handler = unsafe extern "rust-preserve-none" fn(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error>;

static HANDLERS: [Handler; 54] = [
    opcode_add_rr,
    opcode_add_rk,
    opcode_subtract_rr,
    opcode_subtract_rk,
    opcode_subtract_kr,
    opcode_multiply_rr,
    opcode_multiply_rk,
    opcode_divide_rr,
    opcode_divide_rk,
    opcode_divide_kr,
    opcode_modulo_rr,
    opcode_modulo_rk,
    opcode_modulo_kr,
    opcode_equal_rr,
    opcode_equal_rk,
    opcode_not_equal_rr,
    opcode_not_equal_rk,
    opcode_less_rr,
    opcode_less_rk,
    opcode_less_equal_rr,
    opcode_less_equal_rk,
    opcode_greater_rr,
    opcode_greater_rk,
    opcode_greater_equal_rr,
    opcode_greater_equal_rk,
    opcode_not,
    opcode_negate,
    opcode_move,
    opcode_load_const,
    opcode_create_map,
    opcode_set_field,
    opcode_get_field,
    opcode_create_closure,
    opcode_capture_value,
    opcode_create_ref,
    opcode_deref_set,
    opcode_deref,
    opcode_call,
    opcode_return,
    opcode_jump,
    opcode_jump_if_false,
    opcode_jump_if_true,
    opcode_jump_if_less_rr,
    opcode_jump_if_less_rk,
    opcode_jump_if_less_equal_rr,
    opcode_jump_if_less_equal_rk,
    opcode_jump_if_greater_rr,
    opcode_jump_if_greater_rk,
    opcode_jump_if_greater_equal_rr,
    opcode_jump_if_greater_equal_rk,
    opcode_jump_if_equal_rr,
    opcode_jump_if_equal_rk,
    opcode_jump_if_not_equal_rr,
    opcode_jump_if_not_equal_rk,
];

macro_rules! dispatch_next {
    ($ip:expr, $registers:expr, $constants:expr, $state:expr, $frame_size:expr) => {
        unsafe {
            let ip: *const Instruction = $ip.add(1);
            let index = (*ip).discriminant();
            let handler = *HANDLERS.get_unchecked(index);
            become handler(ip, $registers, $constants, $state, $frame_size);
        }
    };
}

macro_rules! dispatch_offset {
    ($ip:expr, $registers:expr, $constants:expr, $state:expr, $frame_size:expr, $offset:expr) => {
        unsafe {
            let ip: *const Instruction = $ip.offset($offset as isize);
            let index = (*ip).discriminant();
            let handler = *HANDLERS.get_unchecked(index);
            become handler(ip, $registers, $constants, $state, $frame_size);
        }
    };
}

#[inline(always)]
fn type_check(cond: bool, message: &'static str) -> Result<(), Error> {
    if std::hint::unlikely(!cond) {
        Err(Error::new(
            Span::default(),
            Symbol::default(),
            message.to_string(),
        ))
    } else {
        Ok(())
    }
}

pub fn run_vm(index: usize, functions: Vec<Function>) -> Result<Value, Error> {
    let Function {
        ref instructions,
        ref constants,
        frame_size,
        ..
    } = functions[index];

    let ip = instructions.as_ptr();
    let index = unsafe { (*ip).discriminant() };

    let constants: Constants = constants.into();
    let mut state = VmState::new(functions);
    let registers = Registers(state.registers.as_mut_ptr());

    let value = unsafe { HANDLERS[index](ip, registers, constants, &mut state, frame_size)? };

    Ok(value)
}

const MAX_REGISTERS: usize = 1024;

pub struct VmState {
    pub functions: Vec<Function>,
    pub registers: [Value; MAX_REGISTERS],
    pub remaining_registers: usize,
    pub gc: Gc,
}

impl VmState {
    pub fn new(functions: Vec<Function>) -> Self {
        Self {
            functions,
            registers: [Value::default(); MAX_REGISTERS],
            remaining_registers: MAX_REGISTERS,
            gc: Gc::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Registers(*mut Value);

impl Registers {
    unsafe fn set_value(&mut self, dest: Reg, value: Value) {
        unsafe { *self.0.add(dest.0 as usize) = value }
    }

    unsafe fn get_value(&self, src: Reg) -> Value {
        unsafe { *self.0.add(src.0 as usize) }
    }
}

#[derive(Clone, Copy)]
struct Constants(*const Value);

impl Constants {
    unsafe fn get_value(&self, src: Const) -> Value {
        unsafe { *self.0.add(src.0 as usize) }
    }
}

impl From<&Vec<Value>> for Constants {
    fn from(value: &Vec<Value>) -> Self {
        Self(value.as_ptr())
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_add_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::Add { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot add, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::number(src1.add(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_add_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::AddK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number(),
        "cannot add, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::number(src1.add(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_subtract_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::Subtract { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot subtract, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::number(src1.sub(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_subtract_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::SubtractRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number(),
        "cannot subtract, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::number(src1.sub(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_subtract_kr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::SubtractKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { constants.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src2.is_number(),
        "cannot subtract, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::number(src1.sub(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_multiply_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::Multiply { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot multiply, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::number(src1.mul(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_multiply_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::MultiplyK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number(),
        "cannot multiply, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::number(src1.mul(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_divide_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::Divide { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot divide, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::number(src1.div(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_divide_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::DivideRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number(),
        "cannot divide, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::number(src1.div(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_divide_kr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::DivideKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { constants.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src2.is_number(),
        "cannot divide, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::number(src1.div(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_modulo_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::Modulo { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compute modulo, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::number(src1.rem(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_modulo_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::ModuloRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number(),
        "cannot compute modulo, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::number(src1.rem(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_modulo_kr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::ModuloKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { constants.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src2.is_number(),
        "cannot compute modulo, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::number(src1.rem(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_equal_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::Equal { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    unsafe { registers.set_value(dest, Value::bool(src1.equal(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_equal_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::EqualK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    unsafe { registers.set_value(dest, Value::bool(src1.equal(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_not_equal_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::NotEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    unsafe { registers.set_value(dest, Value::bool(src1.not_equal(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_not_equal_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::NotEqualK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    unsafe { registers.set_value(dest, Value::bool(src1.not_equal(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::Less { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::bool(src1.less(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::LessK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::bool(src1.less(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_equal_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::LessEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::bool(src1.less_equal(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_equal_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::LessEqualK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::bool(src1.less_equal(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_greater_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::Greater { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::bool(src1.greater(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_greater_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::GreaterK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::bool(src1.greater(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_greater_equal_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::GreaterEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::bool(src1.greater_equal(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_greater_equal_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src1, src2) = unsafe {
        let Instruction::GreaterEqualK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    unsafe { registers.set_value(dest, Value::bool(src1.greater_equal(src2))) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_not(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src) = unsafe {
        let Instruction::Not { dest, src } = *ip else {
            unreachable_unchecked()
        };
        (dest, src)
    };

    let src = unsafe { registers.get_value(src) };

    type_check(src.is_bool(), "cannot apply not, operand must be a boolean")?;

    unsafe { registers.set_value(dest, Value::bool(src.not())) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_negate(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src) = unsafe {
        let Instruction::Negate { dest, src } = *ip else {
            unreachable_unchecked()
        };
        (dest, src)
    };

    let src = unsafe { registers.get_value(src) };

    type_check(src.is_number(), "cannot negate, operand must be a number")?;

    unsafe { registers.set_value(dest, Value::number(src.neg())) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_move(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src) = unsafe {
        let Instruction::Move { dest, src } = *ip else {
            unreachable_unchecked()
        };
        (dest, src)
    };

    let src = unsafe { registers.get_value(src) };
    unsafe { registers.set_value(dest, src) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_load_const(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src) = unsafe {
        let Instruction::LoadConst { dest, src } = *ip else {
            unreachable_unchecked()
        };
        (dest, src)
    };

    let constant = unsafe { constants.get_value(src) };
    unsafe { registers.set_value(dest, constant) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_create_map(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let dest = unsafe {
        let Instruction::CreateMap { dest } = *ip else {
            unreachable_unchecked()
        };
        dest
    };

    let value = state.gc.allocate_map();
    unsafe { registers.set_value(dest, value) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_set_field(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (object, key, value) = unsafe {
        let Instruction::SetField { object, key, value } = *ip else {
            unreachable_unchecked()
        };
        (object, key, value)
    };

    let object = unsafe { registers.get_value(object) };
    let key = unsafe { registers.get_value(key) };
    let value = unsafe { registers.get_value(value) };

    type_check(object.is_map(), "cannot set field, value is not a map")?;

    state.gc.get_mut_map(object).insert(key, value);

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_get_field(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, object, key) = unsafe {
        let Instruction::GetField { dest, object, key } = *ip else {
            unreachable_unchecked()
        };
        (dest, object, key)
    };

    let object = unsafe { registers.get_value(object) };
    let key = unsafe { registers.get_value(key) };

    type_check(object.is_map(), "cannot get field, value is not a map")?;

    let value = state
        .gc
        .get_map(object)
        .get(&key)
        .copied()
        .unwrap_or_default();
    unsafe { registers.set_value(dest, value) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_create_closure(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, captures) = unsafe {
        let Instruction::CreateClosure { dest, captures } = *ip else {
            unreachable_unchecked()
        };
        (dest, captures)
    };

    let function = unsafe { registers.get_value(dest) };
    let mut closure = vec![function];

    for offset in 1..(captures + 1) as usize {
        let ip = unsafe { ip.add(offset) };
        let src = unsafe {
            let Instruction::CaptureValue { src } = *ip else {
                unreachable_unchecked()
            };
            src
        };
        let value = unsafe { registers.get_value(src) };
        closure.push(value);
    }

    let value = state.gc.allocate_closure(closure);
    unsafe { registers.set_value(dest, value) };

    let ip = unsafe { ip.add(captures as usize) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_capture_value(
    _ip: *const Instruction,
    _registers: Registers,
    _constants: Constants,
    _state: &mut VmState,
    _frame_size: usize,
) -> Result<Value, Error> {
    unreachable!("Handler capture by value should not be reached!")
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_create_ref(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src) = unsafe {
        let Instruction::CreateRef { dest, src } = *ip else {
            unreachable_unchecked()
        };
        (dest, src)
    };

    let value = unsafe { registers.get_value(src) };
    let cell = state.gc.allocate_cell(value);
    unsafe { registers.set_value(dest, cell) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_deref_set(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src) = unsafe {
        let Instruction::DerefSet { dest, src } = *ip else {
            unreachable_unchecked()
        };
        (dest, src)
    };

    let cell = unsafe { registers.get_value(dest) };
    type_check(cell.is_cell(), "cannot dereference a non cell")?;

    let value = unsafe { registers.get_value(src) };
    state.gc.set_cell(cell, value);

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_deref(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src) = unsafe {
        let Instruction::Deref { dest, src } = *ip else {
            unreachable_unchecked()
        };
        (dest, src)
    };

    let src = unsafe { registers.get_value(src) };
    type_check(src.is_cell(), "cannot dereference a non cell")?;

    let value = state.gc.get_cell(src);
    unsafe { registers.set_value(dest, value) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_call(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (dest, src) = unsafe {
        let Instruction::Call { dest, src } = *ip else {
            unreachable_unchecked()
        };
        (dest, src)
    };

    let src = unsafe { registers.get_value(src) };

    type_check(
        src.is_closure() || src.is_function(),
        "value is not a callable",
    )?;

    let return_value = {
        state.remaining_registers -= frame_size;

        const MAX_REGISTERS: usize = 256;

        if state.remaining_registers < MAX_REGISTERS {
            return Err(Error::new(
                Span::default(),
                Symbol::default(),
                "the call stack ran out of memory".to_string(),
            ));
        }

        let mut callee_registers = unsafe { Registers(registers.0.add(frame_size)) };
        unsafe { callee_registers.set_value(Reg(0), src) };

        if src.is_closure() {
            let closure = state.gc.get_closure(src);
            let index = closure[0].as_index();

            let Function {
                ref instructions,
                ref constants,
                frame_size,
                arity,
            } = state.functions[index];

            for (index, value) in closure.iter().copied().enumerate().skip(1) {
                let offset = arity + index;
                let dest = Reg(offset as u8);
                unsafe { callee_registers.set_value(dest, value) };
            }

            let constants: Constants = constants.into();

            unsafe {
                let ip = instructions.as_ptr();
                let index = (*ip).discriminant();
                HANDLERS.get_unchecked(index)(ip, callee_registers, constants, state, frame_size)?
            }
        } else {
            let index = src.as_index();

            let Function {
                ref instructions,
                ref constants,
                frame_size,
                ..
            } = state.functions[index];

            let constants: Constants = constants.into();

            unsafe {
                let ip = instructions.as_ptr();
                let index = (*ip).discriminant();
                HANDLERS.get_unchecked(index)(ip, callee_registers, constants, state, frame_size)?
            }
        }
    };

    state.remaining_registers += frame_size;
    unsafe { registers.set_value(dest, return_value) };

    dispatch_next!(ip, registers, constants, state, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_return(
    ip: *const Instruction,
    mut registers: Registers,
    _constants: Constants,
    _state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let src = unsafe {
        let Instruction::Return { src } = *ip else {
            unreachable_unchecked()
        };
        src
    };

    let value = unsafe { registers.get_value(src) };

    for index in 0..frame_size {
        let dest = Reg(index as u8);
        unsafe { registers.set_value(dest, Value::nil()) };
    }

    Ok(value)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let offset = unsafe {
        let Instruction::Jump { offset } = *ip else {
            unreachable_unchecked()
        };
        offset
    };

    dispatch_offset!(ip, registers, constants, state, frame_size, offset)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_false(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (src, offset) = unsafe {
        let Instruction::JumpIfFalse { src, offset } = *ip else {
            unreachable_unchecked()
        };
        (src, offset)
    };

    let src = unsafe { registers.get_value(src) };

    type_check(
        src.is_bool(),
        "cannot use this as a condition, value must be a boolean",
    )?;

    if src.is_false() {
        dispatch_offset!(ip, registers, constants, state, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, state, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_true(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (src, offset) = unsafe {
        let Instruction::JumpIfTrue { src, offset } = *ip else {
            unreachable_unchecked()
        };
        (src, offset)
    };

    let src = unsafe { registers.get_value(src) };

    type_check(
        src.is_bool(),
        "cannot use this as a condition, value must be a boolean",
    )?;

    if src.is_true() {
        dispatch_offset!(ip, registers, constants, state, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, state, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_rr(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfLess { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };
        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    if src1.less(src2) {
        dispatch_offset!(ip, registers, constants, state, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, state, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_rk(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfLessK { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };
        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    if src1.less(src2) {
        dispatch_offset!(ip, registers, constants, state, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, state, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_equal_rr(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfLessEqual { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };
        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    if src1.less_equal(src2) {
        dispatch_offset!(ip, registers, constants, state, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, state, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_equal_rk(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfLessEqualK { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };
        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    if src1.less_equal(src2) {
        dispatch_offset!(ip, registers, constants, state, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, state, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_greater_rr(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfGreater { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };
        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    if src1.greater(src2) {
        dispatch_offset!(ip, registers, constants, state, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, state, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_greater_rk(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfGreaterK { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };
        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    if src1.greater(src2) {
        dispatch_offset!(ip, registers, constants, state, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, state, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_greater_equal_rr(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfGreaterEqual { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };
        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    if src1.greater_equal(src2) {
        dispatch_offset!(ip, registers, constants, state, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, state, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_greater_equal_rk(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfGreaterEqualK { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };
        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    if src1.greater_equal(src2) {
        dispatch_offset!(ip, registers, constants, state, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, state, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_equal_rr(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfEqual { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };
        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    if src1.equal(src2) {
        dispatch_offset!(ip, registers, constants, state, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, state, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_equal_rk(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfEqualK { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };
        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    if src1.equal(src2) {
        dispatch_offset!(ip, registers, constants, state, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, state, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_not_equal_rr(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfNotEqual { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };
        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    if src1.not_equal(src2) {
        dispatch_offset!(ip, registers, constants, state, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, state, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_not_equal_rk(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    state: &mut VmState,
    frame_size: usize,
) -> Result<Value, Error> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfNotEqualK { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };
        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    if src1.not_equal(src2) {
        dispatch_offset!(ip, registers, constants, state, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, state, frame_size)
    }
}

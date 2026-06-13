use super::heap::Heap;
use crate::diagnostics::error::Error;

use crate::runtime::function::Function;
use crate::runtime::heap::Closure;
use crate::runtime::instruction::Instruction;
use crate::runtime::operands::{Const, Reg};
use crate::runtime::value::Value;
use crate::syntax::token::Span;
use crate::util::string_interner::Symbol;
use std::hint::unreachable_unchecked;

type Handler = unsafe extern "rust-preserve-none" fn(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error>;

static HANDLERS: [Handler; 51] = [
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
    opcode_less_kr,
    opcode_less_equal_rr,
    opcode_less_equal_rk,
    opcode_less_equal_kr,
    opcode_not,
    opcode_negate,
    opcode_move,
    opcode_load_const,
    opcode_create_map,
    opcode_set_property,
    opcode_get_property,
    opcode_set_element,
    opcode_create_closure,
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
    opcode_jump_if_less_kr,
    opcode_jump_if_less_equal_rr,
    opcode_jump_if_less_equal_rk,
    opcode_jump_if_less_equal_kr,
    opcode_jump_if_equal_rr,
    opcode_jump_if_equal_rk,
    opcode_jump_if_not_equal_rr,
    opcode_jump_if_not_equal_rk,
    opcode_unreachable,
];

macro_rules! dispatch_to {
    ($ip:expr, $registers:expr, $constants:expr, $thread:expr, $frame_size:expr) => {
        unsafe {
            let index = (*$ip).discriminant();
            let handler = *HANDLERS.get_unchecked(index);
            become handler($ip, $registers, $constants, $thread, $frame_size);
        }
    };
}

macro_rules! dispatch_next {
    ($ip:expr, $registers:expr, $constants:expr, $thread:expr, $frame_size:expr) => {
        unsafe {
            let ip: *const Instruction = $ip.add(1);
            let index = (*ip).discriminant();
            let handler = *HANDLERS.get_unchecked(index);
            become handler(ip, $registers, $constants, $thread, $frame_size);
        }
    };
}

macro_rules! dispatch_offset {
    ($ip:expr, $registers:expr, $constants:expr, $thread:expr, $frame_size:expr, $offset:expr) => {
        unsafe {
            let ip: *const Instruction = $ip.offset($offset as isize);
            let index = (*ip).discriminant();
            let handler = *HANDLERS.get_unchecked(index);
            become handler(ip, $registers, $constants, $thread, $frame_size);
        }
    };
}

#[cold]
#[inline(never)]
fn runtime_error(message: &'static str) -> Error {
    Error::new(Span::default(), Symbol::default(), message.to_string())
}

#[inline(always)]
fn type_check(condition: bool, message: &'static str) -> Result<(), Error> {
    if condition { Ok(()) } else { Err(runtime_error(message)) }
}

pub fn run_vm(index: usize, functions: Vec<Function>) -> Result<(), Error> {
    let Function { ref instructions, ref constants, frame_size, .. } =
        functions[index];

    let ip = instructions.as_ptr();
    let index = unsafe { (*ip).discriminant() };

    let constants: Constants = constants.into();
    let mut thread = Thread::new(functions);
    let registers = Registers(thread.registers.as_mut_ptr());

    unsafe {
        HANDLERS[index](ip, registers, constants, &mut thread, frame_size)?
    };

    let src = 0.into();
    let value = unsafe { registers.get_value(src) };
    //println!("{:?}", value);
    Ok(())
}

const MAX_REGISTERS: usize = 1024;

struct Frame {
    pub dest: Reg,
    pub return_address: *const Instruction,
    pub registers: Registers,
    pub constants: Constants,
    pub size: usize,
}

struct Thread {
    pub functions: Vec<Function>,
    pub registers: [Value; MAX_REGISTERS],
    pub stack: Vec<Frame>,
    pub heap: Heap,
}

impl Thread {
    pub fn new(functions: Vec<Function>) -> Self {
        Self {
            functions,
            registers: [Value::nil(); MAX_REGISTERS],
            stack: Vec::with_capacity(128),
            heap: Heap::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Registers(*mut Value);

impl Registers {
    #[inline(always)]
    unsafe fn set_value(&mut self, dest: Reg, value: Value) {
        unsafe { *self.0.add(dest.0 as usize) = value }
    }

    #[inline(always)]
    unsafe fn get_value(&self, src: Reg) -> Value {
        unsafe { *self.0.add(src.0 as usize) }
    }
}

#[derive(Clone, Copy)]
struct Constants(*const Value);

impl Constants {
    #[inline(always)]
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
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::Add { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_add_rr called on non-Add instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot add, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::number(src1.as_number() + src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_add_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::AddK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_add_rk called on non-AddK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot add, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::number(src1.as_number() + src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_subtract_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::Subtract { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_subtract_rr called on non-Subtract instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot subtract, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::number(src1.as_number() - src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_subtract_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::SubtractRK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_subtract_rk called on non-SubtractRK instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot subtract, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::number(src1.as_number() - src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_subtract_kr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::SubtractKR { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_subtract_kr called on non-SubtractKR instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { constants.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot subtract, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::number(src1.as_number() - src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_multiply_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::Multiply { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_multiply_rr called on non-Multiply instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot multiply, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::number(src1.as_number() * src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_multiply_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::MultiplyK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_multiply_rk called on non-MultiplyK instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot multiply, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::number(src1.as_number() * src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_divide_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::Divide { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_divide_rr called on non-Divide instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot divide, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::number(src1.as_number() / src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_divide_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::DivideRK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_divide_rk called on non-DivideRK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot divide, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::number(src1.as_number() / src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_divide_kr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::DivideKR { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_divide_kr called on non-DivideKR instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { constants.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot divide, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::number(src1.as_number() / src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_modulo_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::Modulo { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_modulo_rr called on non-Modulo instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compute modulo, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::number(src1.as_number() % src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_modulo_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::ModuloRK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_modulo_rk called on non-ModuloRK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compute modulo, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::number(src1.as_number() % src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_modulo_kr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::ModuloKR { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_modulo_kr called on non-ModuloKR instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { constants.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compute modulo, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::number(src1.as_number() % src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_equal_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::Equal { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_equal_rr called on non-Equal instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    unsafe { registers.set_value(dest, Value::bool(src1 == src2)) };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_equal_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::EqualK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_equal_rk called on non-EqualK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    unsafe { registers.set_value(dest, Value::bool(src1 == src2)) };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_not_equal_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::NotEqual { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_not_equal_rr called on non-NotEqual instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    unsafe { registers.set_value(dest, Value::bool(src1 != src2)) };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_not_equal_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::NotEqualK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_not_equal_rk called on non-NotEqualK instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    unsafe { registers.set_value(dest, Value::bool(src1 != src2)) };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::Less { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_less_rr called on non-Less instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::bool(src1.as_number() < src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::LessRK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_less_rk called on non-LessRK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::bool(src1.as_number() < src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_kr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::LessKR { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_less_kr called on non-LessKR instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { constants.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::bool(src1.as_number() < src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_equal_rr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::LessEqual { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_less_equal_rr called on non-LessEqual instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::bool(src1.as_number() <= src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_equal_rk(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::LessEqualRK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_less_equal_rk called on non-LessEqualRK instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::bool(src1.as_number() <= src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_equal_kr(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::LessEqualKR { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_less_equal_kr called on non-LessEqualKR instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { constants.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    unsafe {
        registers
            .set_value(dest, Value::bool(src1.as_number() <= src2.as_number()))
    };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_not(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::Not { dest, src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_not called on non-Not instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src = unsafe { registers.get_value(src) };

    type_check(src.is_bool(), "cannot apply not, operand must be a boolean")?;

    unsafe { registers.set_value(dest, Value::bool(!src.as_bool())) };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_negate(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::Negate { dest, src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_negate called on non-Negate instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src = unsafe { registers.get_value(src) };

    type_check(src.is_number(), "cannot negate, operand must be a number")?;

    unsafe { registers.set_value(dest, Value::number(-src.as_number())) };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_move(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::Move { dest, src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_move called on non-Move instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src = unsafe { registers.get_value(src) };
    unsafe { registers.set_value(dest, src) };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_load_const(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::LoadConst { dest, src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_load_const called on non-LoadConst instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let constant = unsafe { constants.get_value(src) };
    unsafe { registers.set_value(dest, constant) };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_create_map(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::CreateMap { dest } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_create_map called on non-CreateMap instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let index = thread.heap.alloc_map();
    unsafe { registers.set_value(dest, Value::map(index)) };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_set_property(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::SetProperty { object, key, value } = (unsafe { *ip })
    else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_map_set called on non-MapSet instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let object = unsafe { registers.get_value(object) };

    type_check(object.is_map(), "cannot set field, value is not a map")?;

    let key = unsafe { constants.get_value(key) };
    let value = unsafe { registers.get_value(value) };

    thread.heap.get_map_mut(object.as_map()).insert(key, value);

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_get_property(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::GetProperty { dest, object, key } = (unsafe { *ip })
    else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_map_get called on non-MapGet instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let object = unsafe { registers.get_value(object) };

    type_check(object.is_map(), "cannot get field, value is not a map")?;

    let key = unsafe { constants.get_value(key) };
    let object = thread.heap.get_map(object.as_map());

    if let Some(value) = object.get(&key) {
        unsafe { registers.set_value(dest, *value) };

        dispatch_next!(ip, registers, constants, thread, frame_size)
    } else {
        Err(runtime_error("key not present in object"))
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_set_element(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::SetElement { object, key, value } = (unsafe { *ip })
    else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_map_set called on non-MapSet instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let object = unsafe { registers.get_value(object) };

    type_check(object.is_map(), "cannot set field, value is not a map")?;

    let key = unsafe { registers.get_value(key) };
    let value = unsafe { registers.get_value(value) };

    thread.heap.get_map_mut(object.as_map()).insert(key, value);

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_create_closure(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::CreateClosure { dest, src, captures } = (unsafe { *ip })
    else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_create_closure called on non-CreateClosure instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let function = unsafe { thread.functions.get_unchecked(src as usize) };

    let index = thread.heap.alloc_closure(Closure::default());

    unsafe { registers.set_value(dest, Value::closure(index)) };

    let mut capture_values = Box::new_uninit_slice(captures as usize);
    let len = capture_values.len();

    for i in 0..len {
        let Instruction::CaptureValue { src } = (unsafe { *ip.add(i + 1) })
        else {
            if cfg!(debug_assertions) {
                unreachable!(
                    "expected CaptureValue instruction after CreateClosure"
                );
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        let src = unsafe { registers.get_value(src) };
        capture_values[i].write(src);
    }

    let capture_values = unsafe { capture_values.assume_init() };

    let closure = Closure::new(function, capture_values);

    *thread.heap.get_closure_mut(index) = closure;

    let ip = unsafe { ip.add(len) };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_create_ref(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::CreateRef { dest, src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_create_ref called on non-CreateRef instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src = unsafe { registers.get_value(src) };

    let index = thread.heap.alloc_cell(src);

    unsafe { registers.set_value(dest, Value::cell(index)) };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_deref_set(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::DerefSet { dest, src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_deref_set called on non-DerefSet instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let dest = unsafe { registers.get_value(dest) };

    type_check(dest.is_cell(), "cannot dereference a non cell")?;

    let index = dest.as_cell();
    let src = unsafe { registers.get_value(src) };

    *thread.heap.get_cell_mut(index) = src;

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_deref(
    ip: *const Instruction,
    mut registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::Deref { dest, src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_deref called on non-Deref instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src = unsafe { registers.get_value(src) };

    type_check(src.is_cell(), "cannot dereference a non cell")?;

    let index = src.as_cell();
    let value = *thread.heap.get_cell(index);

    unsafe { registers.set_value(dest, value) };

    dispatch_next!(ip, registers, constants, thread, frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_call(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::Call { dest, src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_call called on non-Call instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src = unsafe { registers.get_value(src) };

    type_check(src.is_closure(), "value is not a callable")?;

    let frame = Frame {
        dest,
        return_address: ip,
        registers,
        constants,
        size: frame_size,
    };

    let index = src.as_closure();
    let registers = unsafe { registers.0.add(frame_size) };

    let Closure { function, captures } = thread.heap.get_closure(index);

    let Function { instructions, constants, frame_size, arity } =
        unsafe { &**function };

    unsafe {
        let registers = registers.add(*arity);

        for index in 0..captures.len() {
            let value = captures.get_unchecked(index);
            *registers.add(index) = *value;
        }
    };

    let constants: Constants = constants.into();
    let registers = Registers(registers);
    thread.stack.push(frame);

    let ip = instructions.as_ptr();

    dispatch_to!(ip, registers, constants, thread, *frame_size)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_return(
    ip: *const Instruction,
    mut registers: Registers,
    _constants: Constants,
    thread: &mut Thread,
    _frame_size: usize,
) -> Result<(), Error> {
    let Instruction::Return { src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_return called on non-Return instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let value = unsafe { registers.get_value(src) };

    if let Some(Frame {
        dest,
        return_address,
        mut registers,
        constants,
        size,
    }) = thread.stack.pop()
    {
        unsafe { registers.set_value(dest, value) };

        dispatch_next!(return_address, registers, constants, thread, size)
    } else {
        unsafe { registers.set_value(Reg(0), value) };

        Ok(())
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::Jump { offset } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("opcode_jump called on non-Jump instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    dispatch_offset!(ip, registers, constants, thread, frame_size, offset)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_false(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::JumpIfFalse { src, offset } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_jump_if_false called on non-JumpIfFalse instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src = unsafe { registers.get_value(src) };

    type_check(
        src.is_bool(),
        "cannot use this as a condition, value must be a boolean",
    )?;

    if !src.as_bool() {
        dispatch_offset!(ip, registers, constants, thread, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, thread, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_true(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::JumpIfTrue { src, offset } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_jump_if_true called on non-JumpIfTrue instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src = unsafe { registers.get_value(src) };

    type_check(
        src.is_bool(),
        "cannot use this as a condition, value must be a boolean",
    )?;

    if src.as_bool() {
        dispatch_offset!(ip, registers, constants, thread, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, thread, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_rr(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::JumpIfLess { src1, src2, offset } = (unsafe { *ip })
    else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_jump_if_less_rr called on non-JumpIfLess instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    if src1.as_number() < src2.as_number() {
        dispatch_offset!(ip, registers, constants, thread, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, thread, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_rk(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::JumpIfLessRK { src1, src2, offset } = (unsafe { *ip })
    else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_jump_if_less_rk called on non-JumpIfLessRK instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    if src1.as_number() < src2.as_number() {
        dispatch_offset!(ip, registers, constants, thread, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, thread, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_kr(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::JumpIfLessKR { src1, src2, offset } = (unsafe { *ip })
    else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_jump_if_less_kr called on non-JumpIfLessKR instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { constants.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    if src1.as_number() < src2.as_number() {
        dispatch_offset!(ip, registers, constants, thread, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, thread, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_equal_rr(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::JumpIfLessEqual { src1, src2, offset } = (unsafe { *ip })
    else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_jump_if_less_equal_rr called on non-JumpIfLessEqual instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    if src1.as_number() <= src2.as_number() {
        dispatch_offset!(ip, registers, constants, thread, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, thread, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_equal_rk(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::JumpIfLessEqualRK { src1, src2, offset } =
        (unsafe { *ip })
    else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_jump_if_less_equal_rk called on non-JumpIfLessEqualRK instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    if src1.as_number() <= src2.as_number() {
        dispatch_offset!(ip, registers, constants, thread, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, thread, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_equal_kr(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::JumpIfLessEqualKR { src1, src2, offset } =
        (unsafe { *ip })
    else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_jump_if_less_equal_kr called on non-JumpIfLessEqualKR instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { constants.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    )?;

    if src1.as_number() <= src2.as_number() {
        dispatch_offset!(ip, registers, constants, thread, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, thread, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_equal_rr(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::JumpIfEqual { src1, src2, offset } = (unsafe { *ip })
    else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_jump_if_equal_rr called on non-JumpIfEqual instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    if src1 == src2 {
        dispatch_offset!(ip, registers, constants, thread, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, thread, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_equal_rk(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::JumpIfEqualK { src1, src2, offset } = (unsafe { *ip })
    else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_jump_if_equal_rk called on non-JumpIfEqualK instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    if src1 == src2 {
        dispatch_offset!(ip, registers, constants, thread, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, thread, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_not_equal_rr(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::JumpIfNotEqual { src1, src2, offset } = (unsafe { *ip })
    else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_jump_if_not_equal_rr called on non-JumpIfNotEqual instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    if src1 != src2 {
        dispatch_offset!(ip, registers, constants, thread, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, thread, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_not_equal_rk(
    ip: *const Instruction,
    registers: Registers,
    constants: Constants,
    thread: &mut Thread,
    frame_size: usize,
) -> Result<(), Error> {
    let Instruction::JumpIfNotEqualK { src1, src2, offset } = (unsafe { *ip })
    else {
        if cfg!(debug_assertions) {
            unreachable!(
                "opcode_jump_if_not_equal_rk called on non-JumpIfNotEqualK instruction"
            );
        } else {
            unsafe { unreachable_unchecked() }
        }
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { constants.get_value(src2) };

    if src1 != src2 {
        dispatch_offset!(ip, registers, constants, thread, frame_size, offset)
    } else {
        dispatch_next!(ip, registers, constants, thread, frame_size)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_unreachable(
    ip: *const Instruction,
    _registers: Registers,
    _constants: Constants,
    _thread: &mut Thread,
    _frame_size: usize,
) -> Result<(), Error> {
    unreachable!(
        "Instruction should never be reached on dispatch: {}",
        unsafe { *ip }
    )
}

use std::hint::unreachable_unchecked;

use super::gc::Gc;
use crate::bytecode::Function;
use crate::diagnostics::error::Error;

use crate::program::{CONSTANT_POOL, FUNCTIONS};
use crate::report_error;

use crate::runtime::gc::Closure;
use crate::{bytecode::instruction::Instruction, runtime::value::Value};

type Handler = unsafe extern "rust-preserve-none" fn(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>>;

static HANDLERS: [Handler; 55] = [
    opcode_add_rr,
    opcode_add_ri,
    opcode_subtract_rr,
    opcode_subtract_ri,
    opcode_subtract_ir,
    opcode_multiply_rr,
    opcode_multiply_ri,
    opcode_divide_rr,
    opcode_divide_ri,
    opcode_divide_ir,
    opcode_modulo_rr,
    opcode_modulo_ri,
    opcode_modulo_ir,
    opcode_equal_rr,
    opcode_equal_ri,
    opcode_not_equal_rr,
    opcode_not_equal_ri,
    opcode_less_rr,
    opcode_less_ri,
    opcode_less_equal_rr,
    opcode_less_equal_ri,
    opcode_greater_rr,
    opcode_greater_ri,
    opcode_greater_equal_rr,
    opcode_greater_equal_ri,
    opcode_not,
    opcode_negate,
    opcode_move,
    opcode_move_arg,
    opcode_load_k,
    opcode_load_imm,
    opcode_create_dict,
    opcode_set_field_r,
    opcode_set_field_i,
    opcode_get_field,
    opcode_create_closure,
    opcode_nop,
    opcode_call,
    opcode_return,
    opcode_jump,
    opcode_jump_if_false,
    opcode_jump_if_true,
    opcode_jump_if_less_rr,
    opcode_jump_if_less_ri,
    opcode_jump_if_less_equal_rr,
    opcode_jump_if_less_equal_ri,
    opcode_jump_if_greater_rr,
    opcode_jump_if_greater_ri,
    opcode_jump_if_greater_equal_rr,
    opcode_jump_if_greater_equal_ri,
    opcode_jump_if_equal_rr,
    opcode_jump_if_equal_ri,
    opcode_jump_if_not_equal_rr,
    opcode_jump_if_not_equal_ri,
    opcode_nop,
];

macro_rules! dispatch_next {
    ($ip:expr, $registers:expr, $vm:expr, $frame_size:expr) => {{
        let ip: *const Instruction = $ip.add(1);
        let instruction = *ip;
        let index = instruction.discriminant();
        let handler = *HANDLERS.get_unchecked(index);

        become handler(ip, $registers, $vm, $frame_size);
    }};
}

macro_rules! dispatch_offset {
    ($ip:expr, $registers:expr, $vm:expr, $frame_size:expr, $offset:expr) => {{
        let ip: *const Instruction = $ip.offset($offset as isize);
        let instruction = *ip;
        let index = instruction.discriminant();
        let handler = *HANDLERS.get_unchecked(index);

        become handler(ip, $registers, $vm, $frame_size);
    }};
}

macro_rules! type_check {
    ($cond:expr, $($arg:tt)*) => {{
        if std::hint::unlikely(!$cond) {
            return Err(Box::new(report_error!($($arg)*)));
        }
    }};
}

pub struct Vm {
    gc: Gc,
}

impl Vm {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { gc: Gc::default() }
    }

    pub fn run(&mut self) -> Result<Value, Error> {
        let mut registers = [Value::default(); 4096];

        let Function {
            ref instructions,
            registers_count,
            ..
        } = FUNCTIONS.get().unwrap()[0];

        let registers = Registers(&mut registers);
        let ip = instructions.as_ptr();

        let index = unsafe { (*ip).discriminant() };

        let result =
            unsafe { HANDLERS[index](ip, registers, self, registers_count).map_err(|e| *e)? };

        Ok(result)
    }
}

struct Registers<'a>(pub &'a mut [Value]);

impl<'a> Registers<'a> {
    fn set_value(&mut self, dest: u8, value: Value) {
        unsafe { *self.0.get_unchecked_mut(dest as usize) = value }
    }

    unsafe fn get_value(&self, src: u8) -> Value {
        unsafe { *self.0.get_unchecked(src as usize) }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_add_rr(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::Add { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot add, both operands must be numbers",
    );

    registers.set_value(dest, Value::number(src1.as_number() + src2.as_number()));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_add_ri(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::AddI { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, Value::number(src2.decode()))
    };

    let src1 = unsafe { registers.get_value(src1) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot add, both operands must be numbers",
    );

    registers.set_value(dest, Value::number(src1.as_number() + src2.as_number()));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_subtract_rr(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::Subtract { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot subtract, both operands must be numbers",
    );

    registers.set_value(dest, Value::number(src1.as_number() - src2.as_number()));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_subtract_ri(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::SubtractRI { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, Value::number(src2.decode()))
    };

    let src1 = unsafe { registers.get_value(src1) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot subtract, both operands must be numbers",
    );

    registers.set_value(dest, Value::number(src1.as_number() - src2.as_number()));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_subtract_ir(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::SubtractIR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, Value::number(src1.decode()), src2)
    };

    let src2 = unsafe { registers.get_value(src2) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot subtract, both operands must be numbers",
    );

    registers.set_value(dest, Value::number(src1.as_number() - src2.as_number()));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_multiply_rr(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::Multiply { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot multiply, both operands must be numbers",
    );

    registers.set_value(dest, Value::number(src1.as_number() * src2.as_number()));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_multiply_ri(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::MultiplyI { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, Value::number(src2.decode()))
    };

    let src1 = unsafe { registers.get_value(src1) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot multiply, both operands must be numbers",
    );

    registers.set_value(dest, Value::number(src1.as_number() * src2.as_number()));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_divide_rr(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::Divide { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot divide, both operands must be numbers",
    );

    registers.set_value(dest, Value::number(src1.as_number() / src2.as_number()));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_divide_ri(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::DivideRI { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, Value::number(src2.decode()))
    };

    let src1 = unsafe { registers.get_value(src1) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot divide, both operands must be numbers",
    );

    registers.set_value(dest, Value::number(src1.as_number() / src2.as_number()));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_divide_ir(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::DivideIR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, Value::number(src1.decode()), src2)
    };

    let src2 = unsafe { registers.get_value(src2) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot divide, both operands must be numbers",
    );

    registers.set_value(dest, Value::number(src1.as_number() / src2.as_number()));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_modulo_rr(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::Modulo { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compute modulo, both operands must be numbers",
    );

    registers.set_value(dest, Value::number(src1.as_number() % src2.as_number()));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_modulo_ri(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::ModuloRI { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, Value::number(src2.decode()))
    };

    let src1 = unsafe { registers.get_value(src1) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compute modulo, both operands must be numbers",
    );

    registers.set_value(dest, Value::number(src1.as_number() % src2.as_number()));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_modulo_ir(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::ModuloIR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, Value::number(src1.decode()), src2)
    };

    let src2 = unsafe { registers.get_value(src2) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compute modulo, both operands must be numbers",
    );

    registers.set_value(dest, Value::number(src1.as_number() % src2.as_number()));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_equal_rr(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::Equal { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    registers.set_value(dest, Value::number((src1 == src2) as u8 as f64));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_equal_ri(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::EqualI { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, Value::number(src2.decode()))
    };

    let src1 = unsafe { registers.get_value(src1) };

    registers.set_value(dest, Value::number((src1 == src2) as u8 as f64));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_not_equal_rr(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::NotEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    registers.set_value(dest, Value::number((src1 != src2) as u8 as f64));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_not_equal_ri(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::NotEqualI { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, Value::number(src2.decode()))
    };

    let src1 = unsafe { registers.get_value(src1) };

    registers.set_value(dest, Value::number((src1 != src2) as u8 as f64));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_rr(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::Less { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    );

    registers.set_value(
        dest,
        Value::number((src1.as_number() < src2.as_number()) as u8 as f64),
    );

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_ri(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::LessI { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, Value::number(src2.decode()))
    };

    let src1 = unsafe { registers.get_value(src1) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    );

    registers.set_value(
        dest,
        Value::number((src1.as_number() < src2.as_number()) as u8 as f64),
    );

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_equal_rr(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::LessEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    );

    registers.set_value(
        dest,
        Value::number((src1.as_number() <= src2.as_number()) as u8 as f64),
    );

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_equal_ri(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::LessEqualI { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, Value::number(src2.decode()))
    };

    let src1 = unsafe { registers.get_value(src1) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    );

    registers.set_value(
        dest,
        Value::number((src1.as_number() <= src2.as_number()) as u8 as f64),
    );

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_greater_rr(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::Greater { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    );

    registers.set_value(
        dest,
        Value::number((src1.as_number() > src2.as_number()) as u8 as f64),
    );

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_greater_ri(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::GreaterI { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, Value::number(src2.decode()))
    };

    let src1 = unsafe { registers.get_value(src1) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    );

    registers.set_value(
        dest,
        Value::number((src1.as_number() > src2.as_number()) as u8 as f64),
    );

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_greater_equal_rr(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::GreaterEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, src2)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    );

    registers.set_value(
        dest,
        Value::number((src1.as_number() >= src2.as_number()) as u8 as f64),
    );

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_greater_equal_ri(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src1, src2) = unsafe {
        let Instruction::GreaterEqualI { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        (dest, src1, Value::number(src2.decode()))
    };

    let src1 = unsafe { registers.get_value(src1) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    );

    registers.set_value(
        dest,
        Value::number((src1.as_number() >= src2.as_number()) as u8 as f64),
    );

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_not(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src) = unsafe {
        let Instruction::Not { dest, src } = *ip else {
            unreachable_unchecked()
        };

        (dest, src)
    };

    let src = unsafe { registers.get_value(src) };

    type_check!(
        src.is_number(),
        "cannot apply not, operand must be a boolean",
    );

    registers.set_value(dest, Value::number((src.as_number() == 0.0) as u8 as f64));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_negate(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src) = unsafe {
        let Instruction::Negate { dest, src } = *ip else {
            unreachable_unchecked()
        };

        (dest, src)
    };

    let src = unsafe { registers.get_value(src) };

    type_check!(src.is_number(), "cannot negate, operand must be a number",);

    registers.set_value(dest, Value::number(-src.as_number()));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_move(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src) = unsafe {
        let Instruction::Move { dest, src } = *ip else {
            unreachable_unchecked()
        };

        (dest, src)
    };

    let src = unsafe { registers.get_value(src) };

    registers.set_value(dest, src);

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_move_arg(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src) = unsafe {
        let Instruction::MoveArg { dest, src } = *ip else {
            unreachable_unchecked()
        };

        (dest, src)
    };

    let src = unsafe { registers.get_value(src) };

    registers.set_value(dest, src);

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_load_k(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src) = unsafe {
        let Instruction::LoadK { dest, src } = *ip else {
            unreachable_unchecked()
        };

        (dest, src)
    };

    let constant = unsafe { CONSTANT_POOL.get().unwrap_unchecked()[src as usize] };

    registers.set_value(dest, constant);

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_load_imm(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src) = unsafe {
        let Instruction::LoadImm { dest, src } = *ip else {
            unreachable_unchecked()
        };

        (dest, src)
    };

    registers.set_value(dest, Value::number(src.decode()));

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_create_dict(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let dest = unsafe {
        let Instruction::CreateDict { dest } = *ip else {
            unreachable_unchecked()
        };

        dest
    };

    let value = vm.gc.allocate_dict();

    registers.set_value(dest, value);

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_set_field_r(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (object, key, value) = unsafe {
        let Instruction::SetField { object, key, value } = *ip else {
            unreachable_unchecked()
        };

        (object, key, value)
    };

    let object = unsafe { registers.get_value(object) };
    let key = unsafe { registers.get_value(key) };
    let value = unsafe { registers.get_value(value) };

    type_check!(object.is_dict(), "cannot set field, value is not a dict",);

    vm.gc.get_mut_dict(object).insert(key, value);

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_set_field_i(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (object, key, value) = unsafe {
        let Instruction::SetFieldI { object, key, src } = *ip else {
            unreachable_unchecked()
        };

        (object, key, Value::number(src.decode()))
    };

    let object = unsafe { registers.get_value(object) };
    let key = unsafe { registers.get_value(key) };

    type_check!(object.is_dict(), "cannot set field, value is not a dict",);

    vm.gc.get_mut_dict(object).insert(key, value);

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_get_field(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, object, key) = unsafe {
        let Instruction::GetField { dest, object, key } = *ip else {
            unreachable_unchecked()
        };

        (dest, object, key)
    };

    let object = unsafe { registers.get_value(object) };
    let key = unsafe { registers.get_value(key) };

    type_check!(object.is_dict(), "cannot get field, value is not a dict",);

    let value = vm
        .gc
        .get_dict(object)
        .get(&key)
        .copied()
        .unwrap_or_default();

    registers.set_value(dest, value);

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_create_closure(
    mut ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, captures_count, src) = unsafe {
        let Instruction::CreateClosure {
            dest,
            captures: captures_count,
            src,
        } = *ip
        else {
            unreachable_unchecked()
        };

        (dest, captures_count, src)
    };

    let Function {
        ref instructions,
        registers_count,
        arity,
    } = FUNCTIONS.get().unwrap()[src as usize];

    let closure = Closure {
        instructions: instructions.as_ptr(),
        arity,
        size: registers_count,
        captured: vec![Value::default(); captures_count as usize].into_boxed_slice(),
    };

    let closure_val = vm.gc.allocate_closure(closure);

    registers.set_value(dest, closure_val);

    let mut captured_values = Vec::with_capacity(captures_count as usize);

    for _ in 0..captures_count {
        let capture = unsafe {
            ip = ip.add(1);

            let Instruction::CaptureValue { src } = *ip else {
                unreachable_unchecked()
            };

            registers.get_value(src)
        };

        captured_values.push(capture);
    }

    vm.gc.get_mut_closure(closure_val).captured = captured_values.into_boxed_slice();

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_call(
    ip: *const Instruction,
    mut registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (dest, src, call_arity) = unsafe {
        let Instruction::Call { dest, src, arity } = *ip else {
            unreachable_unchecked()
        };

        (dest, src, arity)
    };

    let src = unsafe { registers.get_value(src) };

    type_check!(src.is_function(), "cannot call, value is not a function",);

    let return_value = {
        let Closure {
            instructions,
            arity: closure_arity,
            size,
            ref captured,
        } = *vm.gc.get_closure(src);

        if call_arity != closure_arity {
            return Err(Box::new(report_error!(
                "the number of arguments must match the number of parameters in a function call"
            )));
        }

        const MIN_REGISTERS: isize = u8::MAX as isize;

        if (registers.0.len() as isize - frame_size as isize) < MIN_REGISTERS {
            return Err(Box::new(report_error!("the call stack ran out of memory")));
        };

        let mut registers = Registers(&mut registers.0[frame_size as usize..]);

        for (i, value) in captured.iter().copied().enumerate() {
            registers.set_value(closure_arity + i as u8, value);
        }

        let index = unsafe { (*instructions).discriminant() };

        unsafe { HANDLERS[index](instructions, registers, vm, size)? }
    };

    registers.set_value(dest, return_value);

    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_return(
    ip: *const Instruction,
    registers: Registers,
    _vm: &mut Vm,
    _frame_size: u8,
) -> Result<Value, Box<Error>> {
    let src = unsafe {
        let Instruction::Return { src } = *ip else {
            unreachable_unchecked()
        };

        src
    };

    let value = unsafe { registers.get_value(src) };

    Ok(value)
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let offset = unsafe {
        let Instruction::Jump { offset } = *ip else {
            unreachable_unchecked()
        };

        offset
    };

    unsafe { dispatch_offset!(ip, registers, vm, frame_size, offset) }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_false(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (src, offset) = unsafe {
        let Instruction::JumpIfFalse { src, offset } = *ip else {
            unreachable_unchecked()
        };

        (src, offset)
    };

    let src = unsafe { registers.get_value(src) };

    type_check!(
        src.is_number(),
        "cannot use this as a condition, value must be a boolean",
    );

    if src.is_truthy() {
        unsafe { dispatch_next!(ip, registers, vm, frame_size) }
    } else {
        unsafe { dispatch_offset!(ip, registers, vm, frame_size, offset) }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_true(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (src, offset) = unsafe {
        let Instruction::JumpIfTrue { src, offset } = *ip else {
            unreachable_unchecked()
        };

        (src, offset)
    };

    let src = unsafe { registers.get_value(src) };

    type_check!(
        src.is_number(),
        "cannot use this as a condition, value must be a boolean",
    );

    if src.is_truthy() {
        unsafe { dispatch_offset!(ip, registers, vm, frame_size, offset) }
    } else {
        unsafe { dispatch_next!(ip, registers, vm, frame_size) }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_rr(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfLess { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };

        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    );

    if src1.as_number() < src2.as_number() {
        unsafe { dispatch_offset!(ip, registers, vm, frame_size, offset) }
    } else {
        unsafe { dispatch_next!(ip, registers, vm, frame_size) }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_ri(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfLessI { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };

        (src1, Value::number(src2.decode()), offset)
    };

    let src1 = unsafe { registers.get_value(src1) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    );

    if src1.as_number() < src2.as_number() {
        unsafe { dispatch_offset!(ip, registers, vm, frame_size, offset) }
    } else {
        unsafe { dispatch_next!(ip, registers, vm, frame_size) }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_equal_rr(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfLessEqual { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };

        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    );

    if src1.as_number() <= src2.as_number() {
        unsafe { dispatch_offset!(ip, registers, vm, frame_size, offset) }
    } else {
        unsafe { dispatch_next!(ip, registers, vm, frame_size) }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_equal_ri(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfLessEqualI { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };

        (src1, Value::number(src2.decode()), offset)
    };

    let src1 = unsafe { registers.get_value(src1) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    );

    if src1.as_number() <= src2.as_number() {
        unsafe { dispatch_offset!(ip, registers, vm, frame_size, offset) }
    } else {
        unsafe { dispatch_next!(ip, registers, vm, frame_size) }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_greater_rr(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfGreater { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };

        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    );

    if src1.as_number() > src2.as_number() {
        unsafe { dispatch_offset!(ip, registers, vm, frame_size, offset) }
    } else {
        unsafe { dispatch_next!(ip, registers, vm, frame_size) }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_greater_ri(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfGreaterI { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };

        (src1, Value::number(src2.decode()), offset)
    };

    let src1 = unsafe { registers.get_value(src1) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    );

    if src1.as_number() > src2.as_number() {
        unsafe { dispatch_offset!(ip, registers, vm, frame_size, offset) }
    } else {
        unsafe { dispatch_next!(ip, registers, vm, frame_size) }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_greater_equal_rr(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfGreaterEqual { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };

        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    );

    if src1.as_number() >= src2.as_number() {
        unsafe { dispatch_offset!(ip, registers, vm, frame_size, offset) }
    } else {
        unsafe { dispatch_next!(ip, registers, vm, frame_size) }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_greater_equal_ri(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfGreaterEqualI { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };

        (src1, Value::number(src2.decode()), offset)
    };

    let src1 = unsafe { registers.get_value(src1) };

    type_check!(
        src1.is_number() && src2.is_number(),
        "cannot compare, both operands must be numbers",
    );

    if src1.as_number() >= src2.as_number() {
        unsafe { dispatch_offset!(ip, registers, vm, frame_size, offset) }
    } else {
        unsafe { dispatch_next!(ip, registers, vm, frame_size) }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_equal_rr(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfEqual { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };

        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    if src1 == src2 {
        unsafe { dispatch_offset!(ip, registers, vm, frame_size, offset) }
    } else {
        unsafe { dispatch_next!(ip, registers, vm, frame_size) }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_equal_ri(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfEqualI { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };

        (src1, Value::number(src2.decode()), offset)
    };

    let src1 = unsafe { registers.get_value(src1) };

    if src1 == src2 {
        unsafe { dispatch_offset!(ip, registers, vm, frame_size, offset) }
    } else {
        unsafe { dispatch_next!(ip, registers, vm, frame_size) }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_not_equal_rr(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfNotEqual { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };

        (src1, src2, offset)
    };

    let src1 = unsafe { registers.get_value(src1) };
    let src2 = unsafe { registers.get_value(src2) };

    if src1 != src2 {
        unsafe { dispatch_offset!(ip, registers, vm, frame_size, offset) }
    } else {
        unsafe { dispatch_next!(ip, registers, vm, frame_size) }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_not_equal_ri(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    let (src1, src2, offset) = unsafe {
        let Instruction::JumpIfNotEqualI { src1, src2, offset } = *ip else {
            unreachable_unchecked()
        };

        (src1, Value::number(src2.decode()), offset)
    };

    let src1 = unsafe { registers.get_value(src1) };

    if src1 != src2 {
        unsafe { dispatch_offset!(ip, registers, vm, frame_size, offset) }
    } else {
        unsafe { dispatch_next!(ip, registers, vm, frame_size) }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_nop(
    ip: *const Instruction,
    registers: Registers,
    vm: &mut Vm,
    frame_size: u8,
) -> Result<Value, Box<Error>> {
    unsafe { dispatch_next!(ip, registers, vm, frame_size) }
}

use std::hint::unreachable_unchecked;

use super::{function::Function, gc::Gc};
use crate::error::kaori_error::KaoriError;
use crate::kaori_error;
use crate::lexer::span::Span;

use crate::{bytecode::instruction::Instruction, runtime::value::Value};

type Handler = extern "rust-preserve-none" fn(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>>;

macro_rules! dispatch_next {
    ($ip:expr, $vm:expr, $registers:expr, $constants:expr, $size:expr) => {{
        let ip: *const Instruction = $ip.add(1);
        let index = (*ip).discriminant();
        become OPCODE_HANDLERS[index](ip, $vm, $registers, $constants, $size);
    }};
}

macro_rules! dispatch_offset {
    ($ip:expr, $vm:expr, $registers:expr, $constants:expr, $offset:expr, $size:expr) => {{
        let ip: *const Instruction = $ip.offset($offset as i16 as isize);
        let index = (*ip).discriminant();
        become OPCODE_HANDLERS[index](ip, $vm, $registers, $constants, $size);
    }};
}

macro_rules! type_check {
    ($cond:expr, $($arg:tt)*) => {
        #[cfg(not(feature = "unchecked"))]
        if std::hint::unlikely(!$cond) {
            return Err(Box::new(kaori_error!(Span::default(), $($arg)*)));
        }
    };
}

const OPCODE_HANDLERS: [Handler; 52] = [
    opcode_add_rr,
    opcode_add_rk,
    opcode_add_kr,
    opcode_subtract_rr,
    opcode_subtract_rk,
    opcode_subtract_kr,
    opcode_multiply_rr,
    opcode_multiply_rk,
    opcode_multiply_kr,
    opcode_divide_rr,
    opcode_divide_rk,
    opcode_divide_kr,
    opcode_modulo_rr,
    opcode_modulo_rk,
    opcode_modulo_kr,
    opcode_equal_rr,
    opcode_equal_rk,
    opcode_equal_kr,
    opcode_not_equal_rr,
    opcode_not_equal_rk,
    opcode_not_equal_kr,
    opcode_less_rr,
    opcode_less_rk,
    opcode_less_kr,
    opcode_less_equal_rr,
    opcode_less_equal_rk,
    opcode_less_equal_kr,
    opcode_greater_rr,
    opcode_greater_rk,
    opcode_greater_kr,
    opcode_greater_equal_rr,
    opcode_greater_equal_rk,
    opcode_greater_equal_kr,
    opcode_not,
    opcode_negate,
    opcode_move_r,
    opcode_move_k,
    opcode_create_dict,
    opcode_set_field_rr,
    opcode_set_field_rk,
    opcode_set_field_kr,
    opcode_set_field_kk,
    opcode_get_field_r,
    opcode_get_field_k,
    opcode_call_k,
    opcode_call_r,
    opcode_return_k,
    opcode_return_r,
    opcode_jump,
    opcode_jump_if_true,
    opcode_jump_if_false,
    opcode_print,
];

pub struct Vm {
    pub registers: Vec<Value>,
    pub gc: Gc,
}

impl Vm {
    pub fn new(gc: Gc) -> Self {
        Self {
            registers: vec![Value::default(); 4096],
            gc,
        }
    }

    pub fn run(&mut self, entry: &Function) -> Result<Value, KaoriError> {
        let Function {
            instructions,
            registers_count,
            constants,
        } = entry;

        let registers = self.registers.as_mut_ptr();
        let constants = constants.as_ptr();
        let ip = instructions.as_ptr();
        let index = unsafe { (*ip).discriminant() };
        OPCODE_HANDLERS[index](ip, self, registers, constants, *registers_count).map_err(|e| *e)
    }
}

#[inline(always)]
unsafe fn get_register_value(index: u8, registers: *mut Value) -> Value {
    unsafe { *registers.add(index as usize) }
}

#[inline(always)]
unsafe fn get_constant_value(index: u8, constants: *const Value) -> Value {
    unsafe { *constants.add(index as usize) }
}

#[inline(always)]
unsafe fn set_value(index: u8, value: Value, registers: *mut Value) {
    unsafe {
        *registers.add(index as usize) = value;
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_add_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::AddRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot add {:?} and {:?}, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::number(lhs.as_number() + rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_add_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::AddRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot add {:?} and {:?}, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::number(lhs.as_number() + rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_add_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::AddKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot add {:?} and {:?}, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::number(lhs.as_number() + rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_subtract_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::SubtractRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot subtract {:?} from {:?}, both operands must be numbers",
            rhs,
            lhs
        );
        set_value(
            dest,
            Value::number(lhs.as_number() - rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_subtract_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::SubtractRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot subtract {:?} from {:?}, both operands must be numbers",
            rhs,
            lhs
        );
        set_value(
            dest,
            Value::number(lhs.as_number() - rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_subtract_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::SubtractKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot subtract {:?} from {:?}, both operands must be numbers",
            rhs,
            lhs
        );
        set_value(
            dest,
            Value::number(lhs.as_number() - rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_multiply_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::MultiplyRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot multiply {:?} and {:?}, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::number(lhs.as_number() * rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_multiply_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::MultiplyRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot multiply {:?} and {:?}, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::number(lhs.as_number() * rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_multiply_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::MultiplyKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot multiply {:?} and {:?}, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::number(lhs.as_number() * rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_divide_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::DivideRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot divide {:?} by {:?}, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::number(lhs.as_number() / rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_divide_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::DivideRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot divide {:?} by {:?}, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::number(lhs.as_number() / rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_divide_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::DivideKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot divide {:?} by {:?}, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::number(lhs.as_number() / rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_modulo_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::ModuloRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compute {:?} modulo {:?}, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::number(lhs.as_number() % rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_modulo_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::ModuloRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compute {:?} modulo {:?}, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::number(lhs.as_number() % rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_modulo_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::ModuloKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compute {:?} modulo {:?}, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::number(lhs.as_number() % rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

// ── comparison ───────────────────────────────────────────────────────────────

#[inline(never)]
extern "rust-preserve-none" fn opcode_equal_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::EqualRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with ==, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() == rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_equal_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::EqualRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with ==, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() == rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_equal_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::EqualKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with ==, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() == rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_not_equal_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::NotEqualRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with !=, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() != rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_not_equal_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::NotEqualRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with !=, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() != rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_not_equal_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::NotEqualKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with !=, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() != rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_less_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::LessRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with <, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() < rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_less_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::LessRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with <, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() < rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_less_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::LessKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with <, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() < rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_less_equal_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::LessEqualRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with <=, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() <= rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_less_equal_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::LessEqualRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with <=, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() <= rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_less_equal_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::LessEqualKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with <=, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() <= rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_greater_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GreaterRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with >, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() > rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_greater_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GreaterRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with >, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() > rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_greater_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GreaterKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with >, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() > rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_greater_equal_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GreaterEqualRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with >=, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() >= rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_greater_equal_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GreaterEqualRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with >=, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() >= rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_greater_equal_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GreaterEqualKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        type_check!(
            lhs.is_number() && rhs.is_number(),
            "cannot compare {:?} and {:?} with >=, both operands must be numbers",
            lhs,
            rhs
        );
        set_value(
            dest,
            Value::boolean(lhs.as_number() >= rhs.as_number()),
            registers,
        );
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_negate(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Negate { dest, src } = *ip else {
            unreachable_unchecked()
        };
        let value = get_register_value(src, registers);
        type_check!(
            value.is_number(),
            "cannot negate {:?}, operand must be a number",
            value
        );
        set_value(dest, Value::number(-value.as_number()), registers);
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_not(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Not { dest, src } = *ip else {
            unreachable_unchecked()
        };
        let value = get_register_value(src, registers);
        type_check!(
            value.is_boolean(),
            "cannot apply ! to {:?}, operand must be a boolean",
            value
        );
        set_value(dest, Value::boolean(!value.as_boolean()), registers);
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_move_r(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::MoveR { dest, src } = *ip else {
            unreachable_unchecked()
        };
        set_value(dest, get_register_value(src, registers), registers);
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_move_k(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::MoveK { dest, src } = *ip else {
            unreachable_unchecked()
        };
        set_value(dest, get_constant_value(src, constants), registers);
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_create_dict(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::CreateDict { dest } = *ip else {
            unreachable_unchecked()
        };
        set_value(dest, vm.gc.allocate_dict(), registers);
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_set_field_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::SetFieldRR { object, key, value } = *ip else {
            unreachable_unchecked()
        };
        let object = get_register_value(object, registers);
        let key = get_register_value(key, registers);
        let val = get_register_value(value, registers);
        type_check!(
            object.is_dict(),
            "cannot set field on {:?}, value is not a dict",
            object
        );
        object.as_dict().insert(key, val);
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_set_field_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::SetFieldRK { object, key, value } = *ip else {
            unreachable_unchecked()
        };
        let object = get_register_value(object, registers);
        let key = get_register_value(key, registers);
        let val = get_constant_value(value, constants);
        type_check!(
            object.is_dict(),
            "cannot set field on {:?}, value is not a dict",
            object
        );
        object.as_dict().insert(key, val);
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_set_field_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::SetFieldKR { object, key, value } = *ip else {
            unreachable_unchecked()
        };
        let object = get_register_value(object, registers);
        let key = get_constant_value(key, constants);
        let val = get_register_value(value, registers);
        type_check!(
            object.is_dict(),
            "cannot set field on {:?}, value is not a dict",
            object
        );
        object.as_dict().insert(key, val);
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_set_field_kk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::SetFieldKK { object, key, value } = *ip else {
            unreachable_unchecked()
        };
        let object = get_register_value(object, registers);
        let key = get_constant_value(key, constants);
        let val = get_constant_value(value, constants);
        type_check!(
            object.is_dict(),
            "cannot set field on {:?}, value is not a dict",
            object
        );
        object.as_dict().insert(key, val);
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_get_field_r(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GetFieldR { dest, object, key } = *ip else {
            unreachable_unchecked()
        };
        let object = get_register_value(object, registers);
        let key = get_register_value(key, registers);
        type_check!(
            object.is_dict(),
            "cannot get field from {:?}, value is not a dict",
            object
        );
        let value = object.as_dict().get(&key).copied().unwrap_or_default();
        set_value(dest, value, registers);
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_get_field_k(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GetFieldK { dest, object, key } = *ip else {
            unreachable_unchecked()
        };
        let object = get_register_value(object, registers);
        let key = get_constant_value(key, constants);
        type_check!(
            object.is_dict(),
            "cannot get field from {:?}, value is not a dict",
            object
        );
        let value = object.as_dict().get(&key).copied().unwrap_or_default();
        set_value(dest, value, registers);
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

// ── call / return ─────────────────────────────────────────────────────────────

#[inline(never)]
extern "rust-preserve-none" fn opcode_call_r(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::CallR { dest, src } = *ip else {
            unreachable_unchecked()
        };
        let callee = get_register_value(src, registers);
        type_check!(
            callee.is_function(),
            "cannot call {:?}, value is not a function",
            callee
        );
        let return_value = {
            let Function {
                ref instructions,
                registers_count,
                ref constants,
            } = *callee.as_function();
            let registers = registers.add(size as usize);
            if std::hint::unlikely(
                registers.add(registers_count as usize)
                    > vm.registers.as_mut_ptr().add(vm.registers.len()),
            ) {
                return Err(Box::new(kaori_error!(Span::default(), "stack overflow")));
            }
            let constants = constants.as_ptr();
            let ip = instructions.as_ptr();
            let index = (*ip).discriminant();
            OPCODE_HANDLERS[index](ip, vm, registers, constants, registers_count)
        }?;
        set_value(dest, return_value, registers);
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_call_k(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::CallK { dest, src } = *ip else {
            unreachable_unchecked()
        };
        let callee = get_constant_value(src, constants);
        type_check!(
            callee.is_function(),
            "cannot call {:?}, value is not a function",
            callee
        );
        let return_value = {
            let Function {
                ref instructions,
                registers_count,
                ref constants,
            } = *callee.as_function();
            if std::hint::unlikely(
                registers.add(registers_count as usize)
                    > vm.registers.as_mut_ptr().add(vm.registers.len()),
            ) {
                return Err(Box::new(kaori_error!(Span::default(), "stack overflow")));
            }
            let registers = registers.add(size as usize);
            let constants = constants.as_ptr();
            let ip = instructions.as_ptr();
            let index = (*ip).discriminant();
            OPCODE_HANDLERS[index](ip, vm, registers, constants, registers_count)
        }?;
        set_value(dest, return_value, registers);
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_return_r(
    ip: *const Instruction,
    _vm: &mut Vm,
    registers: *mut Value,
    _constants: *const Value,
    _size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::ReturnR { src } = *ip else {
            unreachable_unchecked()
        };
        Ok(get_register_value(src, registers))
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_return_k(
    ip: *const Instruction,
    _vm: &mut Vm,
    _registers: *mut Value,
    constants: *const Value,
    _size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::ReturnK { src } = *ip else {
            unreachable_unchecked()
        };
        Ok(get_constant_value(src, constants))
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_jump(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Jump { offset } = *ip else {
            unreachable_unchecked()
        };
        dispatch_offset!(ip, vm, registers, constants, offset, size)
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_jump_if_true(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::JumpIfTrue { src, offset } = *ip else {
            unreachable_unchecked()
        };
        let value = get_register_value(src, registers);
        type_check!(
            value.is_boolean(),
            "cannot use {:?} as a condition, value must be a boolean",
            value
        );
        if value.as_boolean() {
            dispatch_offset!(ip, vm, registers, constants, offset, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_jump_if_false(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::JumpIfFalse { src, offset } = *ip else {
            unreachable_unchecked()
        };
        let value = get_register_value(src, registers);
        type_check!(
            value.is_boolean(),
            "cannot use {:?} as a condition, value must be a boolean",
            value
        );
        if value.as_boolean() {
            dispatch_next!(ip, vm, registers, constants, size)
        } else {
            dispatch_offset!(ip, vm, registers, constants, offset, size)
        }
    }
}

#[inline(never)]
extern "rust-preserve-none" fn opcode_print(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Print { src } = *ip else {
            unreachable_unchecked()
        };
        println!("{:?}", get_register_value(src, registers));
        dispatch_next!(ip, vm, registers, constants, size)
    }
}

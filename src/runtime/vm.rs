use std::hint::unreachable_unchecked;

use super::{function::Function, gc::Gc};
use crate::error::kaori_error::KaoriError;
use crate::kaori_error;
use crate::lexer::span::Span;
use crate::{bytecode::instruction::Instruction, runtime::value::Value};

type Handler = fn(
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
        become HANDLERS[index](ip, $vm, $registers, $constants, $size);
    }};
}

macro_rules! dispatch_next_unchecked {
    ($ip:expr, $vm:expr, $registers:expr, $constants:expr, $size:expr) => {{
        let ip: *const Instruction = $ip.add(1);
        let index = (*ip).discriminant();
        become HANDLERS[index + HANDLERS_UNCHECKED_OFFSET](ip, $vm, $registers, $constants, $size);
    }};
}

macro_rules! dispatch_offset {
    ($ip:expr, $vm:expr, $registers:expr, $constants:expr, $offset:expr, $size:expr) => {{
        let ip: *const Instruction = $ip.offset($offset as i16 as isize);
        let index = (*ip).discriminant();
        become HANDLERS[index](ip, $vm, $registers, $constants, $size);
    }};
}

macro_rules! dispatch_offset_unchecked {
    ($ip:expr, $vm:expr, $registers:expr, $constants:expr, $offset:expr, $size:expr) => {{
        let ip: *const Instruction = $ip.offset($offset as i16 as isize);
        let index = (*ip).discriminant();
        become HANDLERS[index + HANDLERS_UNCHECKED_OFFSET](ip, $vm, $registers, $constants, $size);
    }};
}

macro_rules! type_check {
    ($cond:expr, $($arg:tt)*) => {
        if std::hint::unlikely(!$cond) {
            return Err(Box::new(kaori_error!(Span::default(), $($arg)*)));
        }
    };
}

const REGISTER: u8 = 0;
const CONSTANT: u8 = 1;
const HANDLERS_UNCHECKED_OFFSET: usize = 52;
const HANDLERS: [Handler; 104] = [
    opcode_add::<REGISTER, REGISTER, false>,
    opcode_add::<REGISTER, CONSTANT, false>,
    opcode_add::<CONSTANT, REGISTER, false>,
    opcode_subtract::<REGISTER, REGISTER, false>,
    opcode_subtract::<REGISTER, CONSTANT, false>,
    opcode_subtract::<CONSTANT, REGISTER, false>,
    opcode_multiply::<REGISTER, REGISTER, false>,
    opcode_multiply::<REGISTER, CONSTANT, false>,
    opcode_multiply::<CONSTANT, REGISTER, false>,
    opcode_divide::<REGISTER, REGISTER, false>,
    opcode_divide::<REGISTER, CONSTANT, false>,
    opcode_divide::<CONSTANT, REGISTER, false>,
    opcode_modulo::<REGISTER, REGISTER, false>,
    opcode_modulo::<REGISTER, CONSTANT, false>,
    opcode_modulo::<CONSTANT, REGISTER, false>,
    opcode_equal::<REGISTER, REGISTER, false>,
    opcode_equal::<REGISTER, CONSTANT, false>,
    opcode_equal::<CONSTANT, REGISTER, false>,
    opcode_not_equal::<REGISTER, REGISTER, false>,
    opcode_not_equal::<REGISTER, CONSTANT, false>,
    opcode_not_equal::<CONSTANT, REGISTER, false>,
    opcode_less::<REGISTER, REGISTER, false>,
    opcode_less::<REGISTER, CONSTANT, false>,
    opcode_less::<CONSTANT, REGISTER, false>,
    opcode_less_equal::<REGISTER, REGISTER, false>,
    opcode_less_equal::<REGISTER, CONSTANT, false>,
    opcode_less_equal::<CONSTANT, REGISTER, false>,
    opcode_greater::<REGISTER, REGISTER, false>,
    opcode_greater::<REGISTER, CONSTANT, false>,
    opcode_greater::<CONSTANT, REGISTER, false>,
    opcode_greater_equal::<REGISTER, REGISTER, false>,
    opcode_greater_equal::<REGISTER, CONSTANT, false>,
    opcode_greater_equal::<CONSTANT, REGISTER, false>,
    opcode_not::<false>,
    opcode_negate::<false>,
    opcode_move::<REGISTER, false>,
    opcode_move::<CONSTANT, false>,
    opcode_create_dict::<false>,
    opcode_set_field::<REGISTER, REGISTER, false>,
    opcode_set_field::<REGISTER, CONSTANT, false>,
    opcode_set_field::<CONSTANT, REGISTER, false>,
    opcode_set_field::<CONSTANT, CONSTANT, false>,
    opcode_get_field::<REGISTER, false>,
    opcode_get_field::<CONSTANT, false>,
    opcode_call::<false>,
    opcode_return,
    opcode_jump::<false>,
    opcode_jump_if_true::<false>,
    opcode_jump_if_false::<false>,
    opcode_print::<false>,
    opcode_enter_unchecked_block,
    opcode_exit_unchecked_block,
    // Unchecked handlers
    opcode_add::<REGISTER, REGISTER, true>,
    opcode_add::<REGISTER, CONSTANT, true>,
    opcode_add::<CONSTANT, REGISTER, true>,
    opcode_subtract::<REGISTER, REGISTER, true>,
    opcode_subtract::<REGISTER, CONSTANT, true>,
    opcode_subtract::<CONSTANT, REGISTER, true>,
    opcode_multiply::<REGISTER, REGISTER, true>,
    opcode_multiply::<REGISTER, CONSTANT, true>,
    opcode_multiply::<CONSTANT, REGISTER, true>,
    opcode_divide::<REGISTER, REGISTER, true>,
    opcode_divide::<REGISTER, CONSTANT, true>,
    opcode_divide::<CONSTANT, REGISTER, true>,
    opcode_modulo::<REGISTER, REGISTER, true>,
    opcode_modulo::<REGISTER, CONSTANT, true>,
    opcode_modulo::<CONSTANT, REGISTER, true>,
    opcode_equal::<REGISTER, REGISTER, true>,
    opcode_equal::<REGISTER, CONSTANT, true>,
    opcode_equal::<CONSTANT, REGISTER, true>,
    opcode_not_equal::<REGISTER, REGISTER, true>,
    opcode_not_equal::<REGISTER, CONSTANT, true>,
    opcode_not_equal::<CONSTANT, REGISTER, true>,
    opcode_less::<REGISTER, REGISTER, true>,
    opcode_less::<REGISTER, CONSTANT, true>,
    opcode_less::<CONSTANT, REGISTER, true>,
    opcode_less_equal::<REGISTER, REGISTER, true>,
    opcode_less_equal::<REGISTER, CONSTANT, true>,
    opcode_less_equal::<CONSTANT, REGISTER, true>,
    opcode_greater::<REGISTER, REGISTER, true>,
    opcode_greater::<REGISTER, CONSTANT, true>,
    opcode_greater::<CONSTANT, REGISTER, true>,
    opcode_greater_equal::<REGISTER, REGISTER, true>,
    opcode_greater_equal::<REGISTER, CONSTANT, true>,
    opcode_greater_equal::<CONSTANT, REGISTER, true>,
    opcode_not::<true>,
    opcode_negate::<true>,
    opcode_move::<REGISTER, true>,
    opcode_move::<CONSTANT, true>,
    opcode_create_dict::<true>,
    opcode_set_field::<REGISTER, REGISTER, true>,
    opcode_set_field::<REGISTER, CONSTANT, true>,
    opcode_set_field::<CONSTANT, REGISTER, true>,
    opcode_set_field::<CONSTANT, CONSTANT, true>,
    opcode_get_field::<REGISTER, true>,
    opcode_get_field::<CONSTANT, true>,
    opcode_call::<true>,
    opcode_return,
    opcode_jump::<true>,
    opcode_jump_if_true::<true>,
    opcode_jump_if_false::<true>,
    opcode_print::<true>,
    opcode_enter_unchecked_block,
    opcode_exit_unchecked_block,
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
        HANDLERS[index](ip, self, registers, constants, *registers_count).map_err(|e| *e)
    }
}

#[inline(always)]
unsafe fn get_value<const SRC: u8>(
    index: u8,
    constants: *const Value,
    registers: *mut Value,
) -> Value {
    if SRC == REGISTER {
        unsafe { *registers.add(index as usize) }
    } else {
        unsafe { *constants.add(index as usize) }
    }
}

#[inline(always)]
unsafe fn set_value(dest: u8, value: Value, registers: *mut Value) {
    unsafe {
        *registers.add(dest as usize) = value;
    }
}

#[inline(never)]
fn opcode_enter_unchecked_block(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe { dispatch_next_unchecked!(ip, vm, registers, constants, size) }
}

#[inline(never)]
fn opcode_exit_unchecked_block(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe { dispatch_next!(ip, vm, registers, constants, size) }
}

#[inline(never)]
fn opcode_add<const SRC1: u8, const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::AddRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (REGISTER, CONSTANT) => {
                let Instruction::AddRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (CONSTANT, REGISTER) => {
                let Instruction::AddKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<SRC1>(src1, constants, registers);
        let rhs = get_value::<SRC2>(src2, constants, registers);

        if !UNCHECKED {
            type_check!(
                lhs.is_number() && rhs.is_number(),
                "cannot add {:?} and {:?}, both operands must be numbers",
                lhs,
                rhs
            );
        }

        set_value(
            dest,
            Value::number(lhs.as_number() + rhs.as_number()),
            registers,
        );

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_subtract<const SRC1: u8, const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::SubtractRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (REGISTER, CONSTANT) => {
                let Instruction::SubtractRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (CONSTANT, REGISTER) => {
                let Instruction::SubtractKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<SRC1>(src1, constants, registers);
        let rhs = get_value::<SRC2>(src2, constants, registers);

        if !UNCHECKED {
            type_check!(
                lhs.is_number() && rhs.is_number(),
                "cannot subtract {:?} from {:?}, both operands must be numbers",
                rhs,
                lhs
            );
        }

        set_value(
            dest,
            Value::number(lhs.as_number() - rhs.as_number()),
            registers,
        );

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_multiply<const SRC1: u8, const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::MultiplyRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (REGISTER, CONSTANT) => {
                let Instruction::MultiplyRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (CONSTANT, REGISTER) => {
                let Instruction::MultiplyKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<SRC1>(src1, constants, registers);
        let rhs = get_value::<SRC2>(src2, constants, registers);

        if !UNCHECKED {
            type_check!(
                lhs.is_number() && rhs.is_number(),
                "cannot multiply {:?} and {:?}, both operands must be numbers",
                lhs,
                rhs
            );
        }

        set_value(
            dest,
            Value::number(lhs.as_number() * rhs.as_number()),
            registers,
        );

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_divide<const SRC1: u8, const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::DivideRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (REGISTER, CONSTANT) => {
                let Instruction::DivideRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (CONSTANT, REGISTER) => {
                let Instruction::DivideKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<SRC1>(src1, constants, registers);
        let rhs = get_value::<SRC2>(src2, constants, registers);

        if !UNCHECKED {
            type_check!(
                lhs.is_number() && rhs.is_number(),
                "cannot divide {:?} by {:?}, both operands must be numbers",
                lhs,
                rhs
            );
        }

        set_value(
            dest,
            Value::number(lhs.as_number() / rhs.as_number()),
            registers,
        );

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_modulo<const SRC1: u8, const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::ModuloRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (REGISTER, CONSTANT) => {
                let Instruction::ModuloRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (CONSTANT, REGISTER) => {
                let Instruction::ModuloKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<SRC1>(src1, constants, registers);
        let rhs = get_value::<SRC2>(src2, constants, registers);

        if !UNCHECKED {
            type_check!(
                lhs.is_number() && rhs.is_number(),
                "cannot compute {:?} modulo {:?}, both operands must be numbers",
                lhs,
                rhs
            );
        }

        set_value(
            dest,
            Value::number(lhs.as_number() % rhs.as_number()),
            registers,
        );

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_equal<const SRC1: u8, const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::EqualRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (REGISTER, CONSTANT) => {
                let Instruction::EqualRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (CONSTANT, REGISTER) => {
                let Instruction::EqualKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<SRC1>(src1, constants, registers);
        let rhs = get_value::<SRC2>(src2, constants, registers);

        set_value(dest, Value::boolean(lhs == rhs), registers);

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_not_equal<const SRC1: u8, const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::NotEqualRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (REGISTER, CONSTANT) => {
                let Instruction::NotEqualRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (CONSTANT, REGISTER) => {
                let Instruction::NotEqualKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<SRC1>(src1, constants, registers);
        let rhs = get_value::<SRC2>(src2, constants, registers);

        set_value(dest, Value::boolean(lhs != rhs), registers);

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_less<const SRC1: u8, const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::LessRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (REGISTER, CONSTANT) => {
                let Instruction::LessRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (CONSTANT, REGISTER) => {
                let Instruction::LessKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<SRC1>(src1, constants, registers);
        let rhs = get_value::<SRC2>(src2, constants, registers);

        if !UNCHECKED {
            type_check!(
                lhs.is_number() && rhs.is_number(),
                "cannot compare {:?} and {:?}, both operands must be numbers",
                lhs,
                rhs
            );
        }

        set_value(
            dest,
            Value::boolean(lhs.as_number() < rhs.as_number()),
            registers,
        );

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_less_equal<const SRC1: u8, const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::LessEqualRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (REGISTER, CONSTANT) => {
                let Instruction::LessEqualRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (CONSTANT, REGISTER) => {
                let Instruction::LessEqualKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<SRC1>(src1, constants, registers);
        let rhs = get_value::<SRC2>(src2, constants, registers);

        if !UNCHECKED {
            type_check!(
                lhs.is_number() && rhs.is_number(),
                "cannot compare {:?} and {:?}, both operands must be numbers",
                lhs,
                rhs
            );
        }

        set_value(
            dest,
            Value::boolean(lhs.as_number() <= rhs.as_number()),
            registers,
        );

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_greater<const SRC1: u8, const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::GreaterRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (REGISTER, CONSTANT) => {
                let Instruction::GreaterRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (CONSTANT, REGISTER) => {
                let Instruction::GreaterKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<SRC1>(src1, constants, registers);
        let rhs = get_value::<SRC2>(src2, constants, registers);

        if !UNCHECKED {
            type_check!(
                lhs.is_number() && rhs.is_number(),
                "cannot compare {:?} and {:?}, both operands must be numbers",
                lhs,
                rhs
            );
        }

        set_value(
            dest,
            Value::boolean(lhs.as_number() > rhs.as_number()),
            registers,
        );

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_greater_equal<const SRC1: u8, const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::GreaterEqualRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (REGISTER, CONSTANT) => {
                let Instruction::GreaterEqualRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (CONSTANT, REGISTER) => {
                let Instruction::GreaterEqualKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<SRC1>(src1, constants, registers);
        let rhs = get_value::<SRC2>(src2, constants, registers);

        if !UNCHECKED {
            type_check!(
                lhs.is_number() && rhs.is_number(),
                "cannot compare {:?} and {:?}, both operands must be numbers",
                lhs,
                rhs
            );
        }

        set_value(
            dest,
            Value::boolean(lhs.as_number() >= rhs.as_number()),
            registers,
        );

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_not<const UNCHECKED: bool>(
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
        let value = get_value::<REGISTER>(src, constants, registers);
        if !UNCHECKED {
            type_check!(
                value.is_boolean(),
                "cannot apply ! to {:?}, operand must be a boolean",
                value
            );
        }
        set_value(dest, Value::boolean(!value.as_boolean()), registers);

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_negate<const UNCHECKED: bool>(
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
        let value = get_value::<REGISTER>(src, constants, registers);
        if !UNCHECKED {
            type_check!(
                value.is_number(),
                "cannot negate {:?}, operand must be a number",
                value
            );
        }
        set_value(dest, Value::number(-value.as_number()), registers);

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_move<const SRC: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src) = match SRC {
            REGISTER => {
                let Instruction::MoveR { dest, src } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src)
            }
            CONSTANT => {
                let Instruction::MoveK { dest, src } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src)
            }
            _ => unreachable_unchecked(),
        };

        set_value(dest, get_value::<SRC>(src, constants, registers), registers);

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_create_dict<const UNCHECKED: bool>(
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

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_set_field<const KEY: u8, const VALUE: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (object, key, value) = match (KEY, VALUE) {
            (REGISTER, REGISTER) => {
                let Instruction::SetFieldRR { object, key, value } = *ip else {
                    unreachable_unchecked()
                };
                (object, key, value)
            }
            (REGISTER, CONSTANT) => {
                let Instruction::SetFieldRK { object, key, value } = *ip else {
                    unreachable_unchecked()
                };
                (object, key, value)
            }
            (CONSTANT, REGISTER) => {
                let Instruction::SetFieldKR { object, key, value } = *ip else {
                    unreachable_unchecked()
                };
                (object, key, value)
            }
            (CONSTANT, CONSTANT) => {
                let Instruction::SetFieldKK { object, key, value } = *ip else {
                    unreachable_unchecked()
                };
                (object, key, value)
            }
            _ => unreachable_unchecked(),
        };

        let object = get_value::<REGISTER>(object, constants, registers);
        let key = get_value::<KEY>(key, constants, registers);
        let val = get_value::<VALUE>(value, constants, registers);

        if !UNCHECKED {
            type_check!(
                object.is_dict(),
                "cannot set field on {:?}, value is not a dict",
                object
            );
        }

        object.as_dict().insert(key, val);

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_get_field<const KEY: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, object, key) = match KEY {
            REGISTER => {
                let Instruction::GetFieldR { dest, object, key } = *ip else {
                    unreachable_unchecked()
                };
                (dest, object, key)
            }
            CONSTANT => {
                let Instruction::GetFieldK { dest, object, key } = *ip else {
                    unreachable_unchecked()
                };
                (dest, object, key)
            }
            _ => unreachable_unchecked(),
        };

        let object = get_value::<REGISTER>(object, constants, registers);
        let key = get_value::<KEY>(key, constants, registers);

        if !UNCHECKED {
            type_check!(
                object.is_dict(),
                "cannot get field from {:?}, value is not a dict",
                object
            );
        }

        let value = object.as_dict().get(&key).copied().unwrap_or_default();
        set_value(dest, value, registers);

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_call<const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Call { dest, src } = *ip else {
            unreachable_unchecked()
        };
        let callee = get_value::<REGISTER>(src, constants, registers);

        if !UNCHECKED {
            type_check!(
                callee.is_function(),
                "cannot call {:?}, value is not a function",
                callee
            );
        }

        let return_value = {
            let Function {
                ref instructions,
                registers_count,
                ref constants,
            } = *callee.as_function();

            let callee_registers = registers.add(size as usize);

            if std::hint::unlikely(
                callee_registers.add(registers_count as usize)
                    > vm.registers.as_mut_ptr().add(vm.registers.len()),
            ) {
                return Err(Box::new(kaori_error!(Span::default(), "stack overflow")));
            }

            let constants = constants.as_ptr();
            let ip = instructions.as_ptr();
            let index = (*ip).discriminant();

            HANDLERS[index](ip, vm, callee_registers, constants, registers_count)
        }?;

        set_value(dest, return_value, registers);

        dispatch_next!(ip, vm, registers, constants, size)
    }
}

#[inline(never)]
fn opcode_return(
    ip: *const Instruction,
    _vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    _size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Return { src } = *ip else {
            unreachable_unchecked()
        };
        Ok(get_value::<REGISTER>(src, constants, registers))
    }
}

#[inline(never)]
fn opcode_jump<const UNCHECKED: bool>(
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

        if UNCHECKED {
            dispatch_offset_unchecked!(ip, vm, registers, constants, offset, size)
        } else {
            dispatch_offset!(ip, vm, registers, constants, offset, size)
        }
    }
}

#[inline(never)]
fn opcode_jump_if_true<const UNCHECKED: bool>(
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
        let value = get_value::<REGISTER>(src, constants, registers);

        if !UNCHECKED {
            type_check!(
                value.is_boolean(),
                "cannot use {:?} as a condition, value must be a boolean",
                value
            );
        }

        if value.as_boolean() {
            if UNCHECKED {
                dispatch_offset_unchecked!(ip, vm, registers, constants, offset, size)
            } else {
                dispatch_offset!(ip, vm, registers, constants, offset, size)
            }
        } else {
            if UNCHECKED {
                dispatch_next_unchecked!(ip, vm, registers, constants, size)
            } else {
                dispatch_next!(ip, vm, registers, constants, size)
            }
        }
    }
}

#[inline(never)]
fn opcode_jump_if_false<const UNCHECKED: bool>(
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
        let value = get_value::<REGISTER>(src, constants, registers);

        if !UNCHECKED {
            type_check!(
                value.is_boolean(),
                "cannot use {:?} as a condition, value must be a boolean",
                value
            );
        }

        if value.as_boolean() {
            if UNCHECKED {
                dispatch_next_unchecked!(ip, vm, registers, constants, size)
            } else {
                dispatch_next!(ip, vm, registers, constants, size)
            }
        } else {
            if UNCHECKED {
                dispatch_offset_unchecked!(ip, vm, registers, constants, offset, size)
            } else {
                dispatch_offset!(ip, vm, registers, constants, offset, size)
            }
        }
    }
}

#[inline(never)]
fn opcode_print<const UNCHECKED: bool>(
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
        println!("{:?}", get_value::<REGISTER>(src, constants, registers));

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

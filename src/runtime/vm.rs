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
        let ip: *const Instruction = $ip.offset($offset as isize);
        let index = (*ip).discriminant();
        become HANDLERS[index](ip, $vm, $registers, $constants, $size);
    }};
}

macro_rules! dispatch_offset_unchecked {
    ($ip:expr, $vm:expr, $registers:expr, $constants:expr, $offset:expr, $size:expr) => {{
        let ip: *const Instruction = $ip.offset($offset as isize);
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
const IMMEDIATE: u8 = 1;
const HANDLERS_UNCHECKED_OFFSET: usize = HANDLERS.len() / 2;

const HANDLERS: [Handler; 108] = [
    // CHECKED HANDLERS
    opcode_add::<REGISTER, false>,
    opcode_add::<IMMEDIATE, false>,
    opcode_subtract::<REGISTER, REGISTER, false>,
    opcode_subtract::<REGISTER, IMMEDIATE, false>,
    opcode_subtract::<IMMEDIATE, REGISTER, false>,
    opcode_multiply::<REGISTER, false>,
    opcode_multiply::<IMMEDIATE, false>,
    opcode_divide::<REGISTER, REGISTER, false>,
    opcode_divide::<REGISTER, IMMEDIATE, false>,
    opcode_divide::<IMMEDIATE, REGISTER, false>,
    opcode_modulo::<REGISTER, REGISTER, false>,
    opcode_modulo::<REGISTER, IMMEDIATE, false>,
    opcode_modulo::<IMMEDIATE, REGISTER, false>,
    opcode_equal::<REGISTER, false>,
    opcode_equal::<IMMEDIATE, false>,
    opcode_not_equal::<REGISTER, false>,
    opcode_not_equal::<IMMEDIATE, false>,
    opcode_less::<REGISTER, false>,
    opcode_less::<IMMEDIATE, false>,
    opcode_less_equal::<REGISTER, false>,
    opcode_less_equal::<IMMEDIATE, false>,
    opcode_greater::<REGISTER, false>,
    opcode_greater::<IMMEDIATE, false>,
    opcode_greater_equal::<REGISTER, false>,
    opcode_greater_equal::<IMMEDIATE, false>,
    opcode_not::<false>,
    opcode_negate::<false>,
    opcode_move::<false>,
    opcode_load_k::<false>,
    opcode_load_imm::<false>,
    opcode_create_dict::<false>,
    opcode_set_field::<REGISTER, false>,
    opcode_set_field::<IMMEDIATE, false>,
    opcode_get_field::<false>,
    opcode_call::<false>,
    opcode_return,
    opcode_jump::<false>,
    opcode_jump_if_true::<false>,
    opcode_jump_if_false::<false>,
    opcode_jump_if_less::<REGISTER, false>,
    opcode_jump_if_less::<IMMEDIATE, false>,
    opcode_jump_if_less_equal::<REGISTER, false>,
    opcode_jump_if_less_equal::<IMMEDIATE, false>,
    opcode_jump_if_greater::<REGISTER, false>,
    opcode_jump_if_greater::<IMMEDIATE, false>,
    opcode_jump_if_greater_equal::<REGISTER, false>,
    opcode_jump_if_greater_equal::<IMMEDIATE, false>,
    opcode_jump_if_equal::<REGISTER, false>,
    opcode_jump_if_equal::<IMMEDIATE, false>,
    opcode_jump_if_not_equal::<REGISTER, false>,
    opcode_jump_if_not_equal::<IMMEDIATE, false>,
    opcode_print::<false>,
    opcode_enter_unchecked_block,
    opcode_exit_unchecked_block,
    // UNCHECKED HANDLERS
    opcode_add::<REGISTER, true>,
    opcode_add::<IMMEDIATE, true>,
    opcode_subtract::<REGISTER, REGISTER, true>,
    opcode_subtract::<REGISTER, IMMEDIATE, true>,
    opcode_subtract::<IMMEDIATE, REGISTER, true>,
    opcode_multiply::<REGISTER, true>,
    opcode_multiply::<IMMEDIATE, true>,
    opcode_divide::<REGISTER, REGISTER, true>,
    opcode_divide::<REGISTER, IMMEDIATE, true>,
    opcode_divide::<IMMEDIATE, REGISTER, true>,
    opcode_modulo::<REGISTER, REGISTER, true>,
    opcode_modulo::<REGISTER, IMMEDIATE, true>,
    opcode_modulo::<IMMEDIATE, REGISTER, true>,
    opcode_equal::<REGISTER, true>,
    opcode_equal::<IMMEDIATE, true>,
    opcode_not_equal::<REGISTER, true>,
    opcode_not_equal::<IMMEDIATE, true>,
    opcode_less::<REGISTER, true>,
    opcode_less::<IMMEDIATE, true>,
    opcode_less_equal::<REGISTER, true>,
    opcode_less_equal::<IMMEDIATE, true>,
    opcode_greater::<REGISTER, true>,
    opcode_greater::<IMMEDIATE, true>,
    opcode_greater_equal::<REGISTER, true>,
    opcode_greater_equal::<IMMEDIATE, true>,
    opcode_not::<true>,
    opcode_negate::<true>,
    opcode_move::<true>,
    opcode_load_k::<true>,
    opcode_load_imm::<true>,
    opcode_create_dict::<true>,
    opcode_set_field::<REGISTER, true>,
    opcode_set_field::<IMMEDIATE, true>,
    opcode_get_field::<true>,
    opcode_call::<true>,
    opcode_return,
    opcode_jump::<true>,
    opcode_jump_if_true::<true>,
    opcode_jump_if_false::<true>,
    opcode_jump_if_less::<REGISTER, true>,
    opcode_jump_if_less::<IMMEDIATE, true>,
    opcode_jump_if_less_equal::<REGISTER, true>,
    opcode_jump_if_less_equal::<IMMEDIATE, true>,
    opcode_jump_if_greater::<REGISTER, true>,
    opcode_jump_if_greater::<IMMEDIATE, true>,
    opcode_jump_if_greater_equal::<REGISTER, true>,
    opcode_jump_if_greater_equal::<IMMEDIATE, true>,
    opcode_jump_if_equal::<REGISTER, true>,
    opcode_jump_if_equal::<IMMEDIATE, true>,
    opcode_jump_if_not_equal::<REGISTER, true>,
    opcode_jump_if_not_equal::<IMMEDIATE, true>,
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
    imm: u16,
    constants: *const Value,
    registers: *mut Value,
) -> Value {
    unsafe {
        match SRC {
            REGISTER => *registers.add(index as usize),
            IMMEDIATE => Value::number(decode_immediate(imm)),
            _ => unreachable_unchecked(),
        }
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
fn opcode_add<const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2, imm) = match SRC2 {
            REGISTER => {
                let Instruction::Add { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2, 0u16)
            }
            IMMEDIATE => {
                let Instruction::AddI { dest, src1, imm } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, 0u8, imm)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = *registers.add(src1 as usize);
        let rhs = get_value::<SRC2>(src2, imm, constants, registers);

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
        let (dest, src1, src2, imm1, imm2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::Subtract { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2, 0u16, 0u16)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::SubtractRI { dest, src1, imm } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, 0u8, 0u16, imm)
            }
            (IMMEDIATE, REGISTER) => {
                let Instruction::SubtractIR { dest, imm, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, 0u8, src2, imm, 0u16)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<SRC1>(src1, imm1, constants, registers);
        let rhs = get_value::<SRC2>(src2, imm2, constants, registers);

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
fn opcode_multiply<const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2, imm) = match SRC2 {
            REGISTER => {
                let Instruction::Multiply { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2, 0u16)
            }
            IMMEDIATE => {
                let Instruction::MultiplyI { dest, src1, imm } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, 0u8, imm)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = *registers.add(src1 as usize);
        let rhs = get_value::<SRC2>(src2, imm, constants, registers);

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
        let (dest, src1, src2, imm1, imm2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::Divide { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2, 0u16, 0u16)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::DivideRI { dest, src1, imm } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, 0u8, 0u16, imm)
            }
            (IMMEDIATE, REGISTER) => {
                let Instruction::DivideIR { dest, imm, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, 0u8, src2, imm, 0u16)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<SRC1>(src1, imm1, constants, registers);
        let rhs = get_value::<SRC2>(src2, imm2, constants, registers);

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
        let (dest, src1, src2, imm1, imm2) = match (SRC1, SRC2) {
            (REGISTER, REGISTER) => {
                let Instruction::Modulo { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2, 0u16, 0u16)
            }
            (REGISTER, IMMEDIATE) => {
                let Instruction::ModuloRI { dest, src1, imm } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, 0u8, 0u16, imm)
            }
            (IMMEDIATE, REGISTER) => {
                let Instruction::ModuloIR { dest, imm, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, 0u8, src2, imm, 0u16)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<SRC1>(src1, imm1, constants, registers);
        let rhs = get_value::<SRC2>(src2, imm2, constants, registers);

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
fn opcode_equal<const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2, imm) = match SRC2 {
            REGISTER => {
                let Instruction::Equal { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2, 0u16)
            }
            IMMEDIATE => {
                let Instruction::EqualI { dest, src1, imm } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, 0u8, imm)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = *registers.add(src1 as usize);
        let rhs = get_value::<SRC2>(src2, imm, constants, registers);

        set_value(dest, Value::boolean(lhs == rhs), registers);

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_not_equal<const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2, imm) = match SRC2 {
            REGISTER => {
                let Instruction::NotEqual { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2, 0u16)
            }
            IMMEDIATE => {
                let Instruction::NotEqualI { dest, src1, imm } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, 0u8, imm)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = *registers.add(src1 as usize);
        let rhs = get_value::<SRC2>(src2, imm, constants, registers);

        set_value(dest, Value::boolean(lhs != rhs), registers);

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_less<const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2, imm) = match SRC2 {
            REGISTER => {
                let Instruction::Less { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2, 0u16)
            }
            IMMEDIATE => {
                let Instruction::LessI { dest, src1, imm } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, 0u8, imm)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = *registers.add(src1 as usize);
        let rhs = get_value::<SRC2>(src2, imm, constants, registers);

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
fn opcode_less_equal<const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2, imm) = match SRC2 {
            REGISTER => {
                let Instruction::LessEqual { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2, 0u16)
            }
            IMMEDIATE => {
                let Instruction::LessEqualI { dest, src1, imm } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, 0u8, imm)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = *registers.add(src1 as usize);
        let rhs = get_value::<SRC2>(src2, imm, constants, registers);

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
fn opcode_greater<const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2, imm) = match SRC2 {
            REGISTER => {
                let Instruction::Greater { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2, 0u16)
            }
            IMMEDIATE => {
                let Instruction::GreaterI { dest, src1, imm } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, 0u8, imm)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = *registers.add(src1 as usize);
        let rhs = get_value::<SRC2>(src2, imm, constants, registers);

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
fn opcode_greater_equal<const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2, imm) = match SRC2 {
            REGISTER => {
                let Instruction::GreaterEqual { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2, 0u16)
            }
            IMMEDIATE => {
                let Instruction::GreaterEqualI { dest, src1, imm } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, 0u8, imm)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = *registers.add(src1 as usize);
        let rhs = get_value::<SRC2>(src2, imm, constants, registers);

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
        let value = get_value::<REGISTER>(src, 0, constants, registers);

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
        let value = get_value::<REGISTER>(src, 0, constants, registers);

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
fn opcode_move<const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Move { dest, src } = *ip else {
            unreachable_unchecked()
        };
        set_value(
            dest,
            get_value::<REGISTER>(src, 0, constants, registers),
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
fn opcode_load_k<const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::LoadK { dest, src } = *ip else {
            unreachable_unchecked()
        };
        set_value(dest, *constants.add(src as usize), registers);

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_load_imm<const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::LoadImm { dest, imm } = *ip else {
            unreachable_unchecked()
        };
        set_value(
            dest,
            get_value::<IMMEDIATE>(0, imm, constants, registers),
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
fn opcode_set_field<const VALUE: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (object, key, src, imm) = match VALUE {
            REGISTER => {
                let Instruction::SetField { object, key, value } = *ip else {
                    unreachable_unchecked()
                };
                (object, key, value, 0u16)
            }
            IMMEDIATE => {
                let Instruction::SetFieldI { object, key, imm } = *ip else {
                    unreachable_unchecked()
                };
                (object, key, 0u8, imm)
            }
            _ => unreachable_unchecked(),
        };

        let object = get_value::<REGISTER>(object, 0, constants, registers);
        let key = get_value::<REGISTER>(key, 0, constants, registers);
        let value = get_value::<VALUE>(src, imm, constants, registers);

        type_check!(
            object.is_dict(),
            "cannot set field on {:?}, value is not a dict",
            object
        );

        object.as_dict().insert(key, value);

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_get_field<const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GetField { dest, object, key } = *ip else {
            unreachable_unchecked()
        };

        let object = get_value::<REGISTER>(object, 0, constants, registers);
        let key = get_value::<REGISTER>(key, 0, constants, registers);

        type_check!(
            object.is_dict(),
            "cannot get field from {:?}, value is not a dict",
            object
        );

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
        let callee = get_value::<REGISTER>(src, 0, constants, registers);

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
        Ok(get_value::<REGISTER>(src, 0, constants, registers))
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
        let value = get_value::<REGISTER>(src, 0, constants, registers);

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
        } else if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
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
        let value = get_value::<REGISTER>(src, 0, constants, registers);

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
        } else if UNCHECKED {
            dispatch_offset_unchecked!(ip, vm, registers, constants, offset, size)
        } else {
            dispatch_offset!(ip, vm, registers, constants, offset, size)
        }
    }
}

#[inline(never)]
fn opcode_jump_if_less<const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (src1, src2, imm, offset) = match SRC2 {
            REGISTER => {
                let Instruction::JumpIfLess { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                (src1, src2, 0u16, offset)
            }
            IMMEDIATE => {
                let Instruction::JumpIfLessI { src1, imm, offset } = *ip else {
                    unreachable_unchecked()
                };
                (src1, 0u8, imm, offset)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<REGISTER>(src1, 0, constants, registers);
        let rhs = get_value::<SRC2>(src2, imm, constants, registers);

        if !UNCHECKED {
            type_check!(
                lhs.is_number() && rhs.is_number(),
                "cannot compare {:?} and {:?}, both operands must be numbers",
                lhs,
                rhs
            );
        }

        if lhs.as_number() < rhs.as_number() {
            if UNCHECKED {
                dispatch_offset_unchecked!(ip, vm, registers, constants, offset, size)
            } else {
                dispatch_offset!(ip, vm, registers, constants, offset, size)
            }
        } else if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_jump_if_less_equal<const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (src1, src2, imm, offset) = match SRC2 {
            REGISTER => {
                let Instruction::JumpIfLessEqual { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                (src1, src2, 0u16, offset)
            }
            IMMEDIATE => {
                let Instruction::JumpIfLessEqualI { src1, imm, offset } = *ip else {
                    unreachable_unchecked()
                };
                (src1, 0u8, imm, offset)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<REGISTER>(src1, 0, constants, registers);
        let rhs = get_value::<SRC2>(src2, imm, constants, registers);

        if !UNCHECKED {
            type_check!(
                lhs.is_number() && rhs.is_number(),
                "cannot compare {:?} and {:?}, both operands must be numbers",
                lhs,
                rhs
            );
        }

        if lhs.as_number() <= rhs.as_number() {
            if UNCHECKED {
                dispatch_offset_unchecked!(ip, vm, registers, constants, offset, size)
            } else {
                dispatch_offset!(ip, vm, registers, constants, offset, size)
            }
        } else if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_jump_if_greater<const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (src1, src2, imm, offset) = match SRC2 {
            REGISTER => {
                let Instruction::JumpIfGreater { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                (src1, src2, 0u16, offset)
            }
            IMMEDIATE => {
                let Instruction::JumpIfGreaterI { src1, imm, offset } = *ip else {
                    unreachable_unchecked()
                };
                (src1, 0u8, imm, offset)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<REGISTER>(src1, 0, constants, registers);
        let rhs = get_value::<SRC2>(src2, imm, constants, registers);

        if !UNCHECKED {
            type_check!(
                lhs.is_number() && rhs.is_number(),
                "cannot compare {:?} and {:?}, both operands must be numbers",
                lhs,
                rhs
            );
        }

        if lhs.as_number() > rhs.as_number() {
            if UNCHECKED {
                dispatch_offset_unchecked!(ip, vm, registers, constants, offset, size)
            } else {
                dispatch_offset!(ip, vm, registers, constants, offset, size)
            }
        } else if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_jump_if_greater_equal<const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (src1, src2, imm, offset) = match SRC2 {
            REGISTER => {
                let Instruction::JumpIfGreaterEqual { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                (src1, src2, 0u16, offset)
            }
            IMMEDIATE => {
                let Instruction::JumpIfGreaterEqualI { src1, imm, offset } = *ip else {
                    unreachable_unchecked()
                };
                (src1, 0u8, imm, offset)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<REGISTER>(src1, 0, constants, registers);
        let rhs = get_value::<SRC2>(src2, imm, constants, registers);

        if !UNCHECKED {
            type_check!(
                lhs.is_number() && rhs.is_number(),
                "cannot compare {:?} and {:?}, both operands must be numbers",
                lhs,
                rhs
            );
        }

        if lhs.as_number() >= rhs.as_number() {
            if UNCHECKED {
                dispatch_offset_unchecked!(ip, vm, registers, constants, offset, size)
            } else {
                dispatch_offset!(ip, vm, registers, constants, offset, size)
            }
        } else if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_jump_if_equal<const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (src1, src2, imm, offset) = match SRC2 {
            REGISTER => {
                let Instruction::JumpIfEqual { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                (src1, src2, 0u16, offset)
            }
            IMMEDIATE => {
                let Instruction::JumpIfEqualI { src1, imm, offset } = *ip else {
                    unreachable_unchecked()
                };
                (src1, 0u8, imm, offset)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<REGISTER>(src1, 0, constants, registers);
        let rhs = get_value::<SRC2>(src2, imm, constants, registers);

        if lhs == rhs {
            if UNCHECKED {
                dispatch_offset_unchecked!(ip, vm, registers, constants, offset, size)
            } else {
                dispatch_offset!(ip, vm, registers, constants, offset, size)
            }
        } else if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_jump_if_not_equal<const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (src1, src2, imm, offset) = match SRC2 {
            REGISTER => {
                let Instruction::JumpIfNotEqual { src1, src2, offset } = *ip else {
                    unreachable_unchecked()
                };
                (src1, src2, 0u16, offset)
            }
            IMMEDIATE => {
                let Instruction::JumpIfNotEqualI { src1, imm, offset } = *ip else {
                    unreachable_unchecked()
                };
                (src1, 0u8, imm, offset)
            }
            _ => unreachable_unchecked(),
        };

        let lhs = get_value::<REGISTER>(src1, 0, constants, registers);
        let rhs = get_value::<SRC2>(src2, imm, constants, registers);

        if lhs != rhs {
            if UNCHECKED {
                dispatch_offset_unchecked!(ip, vm, registers, constants, offset, size)
            } else {
                dispatch_offset!(ip, vm, registers, constants, offset, size)
            }
        } else if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
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
        println!("{:?}", get_value::<REGISTER>(src, 0, constants, registers));

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

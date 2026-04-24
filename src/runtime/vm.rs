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
    ($unchecked:expr, $cond:expr, $($arg:tt)*) => {
        if !$unchecked && std::hint::unlikely(!$cond) {
            return Err(Box::new(kaori_error!(Span::default(), $($arg)*)));
        }
    };
}

const REGISTER: u8 = 0;
const IMMEDIATE: u8 = 1;
const CONSTANT: u8 = 2;
const HANDLERS_UNCHECKED_OFFSET: usize = HANDLERS.len() / 2;

const HANDLERS: [Handler; 110] = [
    // CHECKED HANDLERS (false)

    // --- Arithmetic ---
    opcode_add::<REGISTER, REGISTER, false>,  // Add
    opcode_add::<REGISTER, IMMEDIATE, false>, // AddI
    opcode_subtract::<REGISTER, REGISTER, false>, // Subtract
    opcode_subtract::<REGISTER, IMMEDIATE, false>, // SubtractRI
    opcode_subtract::<IMMEDIATE, REGISTER, false>, // SubtractIR
    opcode_multiply::<REGISTER, REGISTER, false>, // Multiply
    opcode_multiply::<REGISTER, IMMEDIATE, false>, // MultiplyI
    opcode_divide::<REGISTER, REGISTER, false>, // Divide
    opcode_divide::<REGISTER, IMMEDIATE, false>, // DivideRI
    opcode_divide::<IMMEDIATE, REGISTER, false>, // DivideIR
    opcode_modulo::<REGISTER, REGISTER, false>, // Modulo
    opcode_modulo::<REGISTER, IMMEDIATE, false>, // ModuloRI
    opcode_modulo::<IMMEDIATE, REGISTER, false>, // ModuloIR
    // --- Comparisons ---
    opcode_equal::<REGISTER, REGISTER, false>,  // Equal
    opcode_equal::<REGISTER, IMMEDIATE, false>, // EqualI
    opcode_not_equal::<REGISTER, REGISTER, false>,
    opcode_not_equal::<REGISTER, IMMEDIATE, false>,
    opcode_less::<REGISTER, REGISTER, false>,
    opcode_less::<REGISTER, IMMEDIATE, false>,
    opcode_less_equal::<REGISTER, REGISTER, false>,
    opcode_less_equal::<REGISTER, IMMEDIATE, false>,
    opcode_greater::<REGISTER, REGISTER, false>,
    opcode_greater::<REGISTER, IMMEDIATE, false>,
    opcode_greater_equal::<REGISTER, REGISTER, false>,
    opcode_greater_equal::<REGISTER, IMMEDIATE, false>,
    // --- Unary ---
    opcode_not::<false>,
    opcode_negate::<false>,
    // --- Move / Load ---
    opcode_move::<false>,
    opcode_load_k::<false>,
    opcode_load_imm::<false>,
    // --- Objects ---
    opcode_create_dict::<false>,
    opcode_set_field::<REGISTER, false>,  // SetField
    opcode_set_field::<IMMEDIATE, false>, // SetFieldI
    opcode_get_field::<false>,
    // --- Calls ---
    opcode_call::<REGISTER, false>, // Call
    opcode_call::<CONSTANT, false>, // CallK
    opcode_return,
    // --- Control Flow ---
    opcode_jump::<false>,
    opcode_jump_if_false::<false>,
    opcode_jump_if_true::<false>,
    opcode_jump_if_less::<REGISTER, REGISTER, false>,
    opcode_jump_if_less::<REGISTER, IMMEDIATE, false>,
    opcode_jump_if_less_equal::<REGISTER, REGISTER, false>,
    opcode_jump_if_less_equal::<REGISTER, IMMEDIATE, false>,
    opcode_jump_if_greater::<REGISTER, REGISTER, false>,
    opcode_jump_if_greater::<REGISTER, IMMEDIATE, false>,
    opcode_jump_if_greater_equal::<REGISTER, REGISTER, false>,
    opcode_jump_if_greater_equal::<REGISTER, IMMEDIATE, false>,
    opcode_jump_if_equal::<REGISTER, REGISTER, false>,
    opcode_jump_if_equal::<REGISTER, IMMEDIATE, false>,
    opcode_jump_if_not_equal::<REGISTER, REGISTER, false>,
    opcode_jump_if_not_equal::<REGISTER, IMMEDIATE, false>,
    // --- Misc ---
    opcode_print::<false>,
    opcode_enter_unchecked_block,
    opcode_exit_unchecked_block,
    // =============================
    // UNCHECKED HANDLERS (true)
    // =============================
    opcode_add::<REGISTER, REGISTER, true>,
    opcode_add::<REGISTER, IMMEDIATE, true>,
    opcode_subtract::<REGISTER, REGISTER, true>,
    opcode_subtract::<REGISTER, IMMEDIATE, true>,
    opcode_subtract::<IMMEDIATE, REGISTER, true>,
    opcode_multiply::<REGISTER, REGISTER, true>,
    opcode_multiply::<REGISTER, IMMEDIATE, true>,
    opcode_divide::<REGISTER, REGISTER, true>,
    opcode_divide::<REGISTER, IMMEDIATE, true>,
    opcode_divide::<IMMEDIATE, REGISTER, true>,
    opcode_modulo::<REGISTER, REGISTER, true>,
    opcode_modulo::<REGISTER, IMMEDIATE, true>,
    opcode_modulo::<IMMEDIATE, REGISTER, true>,
    opcode_equal::<REGISTER, REGISTER, true>,
    opcode_equal::<REGISTER, IMMEDIATE, true>,
    opcode_not_equal::<REGISTER, REGISTER, true>,
    opcode_not_equal::<REGISTER, IMMEDIATE, true>,
    opcode_less::<REGISTER, REGISTER, true>,
    opcode_less::<REGISTER, IMMEDIATE, true>,
    opcode_less_equal::<REGISTER, REGISTER, true>,
    opcode_less_equal::<REGISTER, IMMEDIATE, true>,
    opcode_greater::<REGISTER, REGISTER, true>,
    opcode_greater::<REGISTER, IMMEDIATE, true>,
    opcode_greater_equal::<REGISTER, REGISTER, true>,
    opcode_greater_equal::<REGISTER, IMMEDIATE, true>,
    opcode_not::<true>,
    opcode_negate::<true>,
    opcode_move::<true>,
    opcode_load_k::<true>,
    opcode_load_imm::<true>,
    opcode_create_dict::<true>,
    opcode_set_field::<REGISTER, true>,
    opcode_set_field::<IMMEDIATE, true>,
    opcode_get_field::<true>,
    opcode_call::<REGISTER, true>,
    opcode_call::<CONSTANT, true>,
    opcode_return,
    opcode_jump::<true>,
    opcode_jump_if_false::<true>,
    opcode_jump_if_true::<true>,
    opcode_jump_if_less::<REGISTER, REGISTER, true>,
    opcode_jump_if_less::<REGISTER, IMMEDIATE, true>,
    opcode_jump_if_less_equal::<REGISTER, REGISTER, true>,
    opcode_jump_if_less_equal::<REGISTER, IMMEDIATE, true>,
    opcode_jump_if_greater::<REGISTER, REGISTER, true>,
    opcode_jump_if_greater::<REGISTER, IMMEDIATE, true>,
    opcode_jump_if_greater_equal::<REGISTER, REGISTER, true>,
    opcode_jump_if_greater_equal::<REGISTER, IMMEDIATE, true>,
    opcode_jump_if_equal::<REGISTER, REGISTER, true>,
    opcode_jump_if_equal::<REGISTER, IMMEDIATE, true>,
    opcode_jump_if_not_equal::<REGISTER, REGISTER, true>,
    opcode_jump_if_not_equal::<REGISTER, IMMEDIATE, true>,
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
            UNCHECKED,
            src1.is_number() && src2.is_number(),
            "cannot add {:?} and {:?}, both operands must be numbers",
            src1,
            src2
        );

        set_value(
            dest,
            Value::number(src1.as_number() + src2.as_number()),
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
            UNCHECKED,
            src1.is_number() && src2.is_number(),
            "cannot subtract {:?} from {:?}, both operands must be numbers",
            src1,
            src2
        );

        set_value(
            dest,
            Value::number(src1.as_number() - src2.as_number()),
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
            UNCHECKED,
            src1.is_number() && src2.is_number(),
            "cannot multiply {:?} and {:?}, both operands must be numbers",
            src1,
            src2
        );

        set_value(
            dest,
            Value::number(src1.as_number() * src2.as_number()),
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
            UNCHECKED,
            src1.is_number() && src2.is_number(),
            "cannot divide {:?} by {:?}, both operands must be numbers",
            src1,
            src2
        );

        set_value(
            dest,
            Value::number(src1.as_number() / src2.as_number()),
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
            UNCHECKED,
            src1.is_number() && src2.is_number(),
            "cannot compute {:?} modulo {:?}, both operands must be numbers",
            src1,
            src2
        );

        set_value(
            dest,
            Value::number(src1.as_number() % src2.as_number()),
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
            UNCHECKED,
            src1.is_number() && src2.is_number(),
            "cannot compare {:?} and {:?}, both operands must be numbers",
            src1,
            src2
        );

        set_value(
            dest,
            Value::number((src1.as_number() < src2.as_number()) as u8 as f64),
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
            UNCHECKED,
            src1.is_number() && src2.is_number(),
            "cannot compare {:?} and {:?}, both operands must be numbers",
            src1,
            src2
        );

        set_value(
            dest,
            Value::number((src1.as_number() <= src2.as_number()) as u8 as f64),
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
            UNCHECKED,
            src1.is_number() && src2.is_number(),
            "cannot compare {:?} and {:?}, both operands must be numbers",
            src1,
            src2
        );

        set_value(
            dest,
            Value::number((src1.as_number() > src2.as_number()) as u8 as f64),
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
            UNCHECKED,
            src1.is_number() && src2.is_number(),
            "cannot compare {:?} and {:?}, both operands must be numbers",
            src1,
            src2
        );

        set_value(
            dest,
            Value::number((src1.as_number() >= src2.as_number()) as u8 as f64),
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
        let src = *registers.add(src as usize);

        type_check!(
            UNCHECKED,
            src.is_number(),
            "cannot apply not to {:?}, operand must be a boolean",
            src
        );

        set_value(
            dest,
            Value::number((src.as_number() == 0.0) as u8 as f64),
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
        let src = *registers.add(src as usize);

        type_check!(
            UNCHECKED,
            src.is_number(),
            "cannot negate {:?}, operand must be a number",
            src
        );

        set_value(dest, Value::number(-src.as_number()), registers);

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

        let src = *registers.add(src as usize);
        set_value(dest, src, registers);

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
        let Instruction::LoadImm { dest, src } = *ip else {
            unreachable_unchecked()
        };
        set_value(dest, Value::number(src.decode()), registers);

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

        type_check!(
            false,
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
        let object = *registers.add(object as usize);
        let key = *registers.add(key as usize);

        type_check!(
            false,
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
fn opcode_call<const SRC: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src) = match SRC {
            REGISTER => {
                let Instruction::Call { dest, src } = *ip else {
                    unreachable_unchecked()
                };

                (dest, *registers.add(src as usize))
            }
            CONSTANT => {
                let Instruction::CallK { dest, src } = *ip else {
                    unreachable_unchecked()
                };

                (dest, *constants.add(src as usize))
            }
            _ => unreachable_unchecked(),
        };

        type_check!(
            false,
            src.is_function(),
            "cannot call {:?}, value is not a function",
            src
        );

        let return_value = {
            let Function {
                ref instructions,
                registers_count,
                ref constants,
            } = *src.as_function();

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

            HANDLERS[index](ip, vm, registers, constants, registers_count)
        }?;

        set_value(dest, return_value, registers);

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

#[inline(never)]
fn opcode_return(
    ip: *const Instruction,
    _vm: &mut Vm,
    registers: *mut Value,
    _constants: *const Value,
    _size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Return { src } = *ip else {
            unreachable_unchecked()
        };

        Ok(*registers.add(src as usize))
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

        let src = *registers.add(src as usize);

        type_check!(
            UNCHECKED,
            src.is_number(),
            "cannot use {:?} as a condition, value must be a boolean",
            src
        );

        if src.is_truthy() {
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

        let src = *registers.add(src as usize);

        type_check!(
            UNCHECKED,
            src.is_number(),
            "cannot use {:?} as a condition, value must be a boolean",
            src
        );

        if src.is_truthy() {
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
fn opcode_jump_if_less<const SRC1: u8, const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
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
            UNCHECKED,
            src1.is_number() && src2.is_number(),
            "cannot compare {:?} and {:?}, both operands must be numbers",
            src1,
            src2
        );

        if src1.as_number() < src2.as_number() {
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
fn opcode_jump_if_less_equal<const SRC1: u8, const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
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
            UNCHECKED,
            src1.is_number() && src2.is_number(),
            "cannot compare {:?} and {:?}, both operands must be numbers",
            src1,
            src2
        );

        if src1.as_number() <= src2.as_number() {
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
fn opcode_jump_if_greater<const SRC1: u8, const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
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
            UNCHECKED,
            src1.is_number() && src2.is_number(),
            "cannot compare {:?} and {:?}, both operands must be numbers",
            src1,
            src2
        );

        if src1.as_number() > src2.as_number() {
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
fn opcode_jump_if_greater_equal<const SRC1: u8, const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
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
            UNCHECKED,
            src1.is_number() && src2.is_number(),
            "cannot compare {:?} and {:?}, both operands must be numbers",
            src1,
            src2
        );

        if src1.as_number() >= src2.as_number() {
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
fn opcode_jump_if_equal<const SRC1: u8, const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
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
fn opcode_jump_if_not_equal<const SRC1: u8, const SRC2: u8, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
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
        let src = *registers.add(src as usize);
        println!("{:?}", src);

        if UNCHECKED {
            dispatch_next_unchecked!(ip, vm, registers, constants, size)
        } else {
            dispatch_next!(ip, vm, registers, constants, size)
        }
    }
}

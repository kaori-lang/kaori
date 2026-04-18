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
        become HANDLERS_CHECKED[index](ip, $vm, $registers, $constants, $size);
    }};
}

macro_rules! dispatch_next_unchecked {
    ($ip:expr, $vm:expr, $registers:expr, $constants:expr, $size:expr) => {{
        let ip: *const Instruction = $ip.add(1);
        let index = (*ip).discriminant();
        become HANDLERS_UNCHECKED[index](ip, $vm, $registers, $constants, $size);
    }};
}

macro_rules! dispatch_offset {
    ($ip:expr, $vm:expr, $registers:expr, $constants:expr, $offset:expr, $size:expr) => {{
        let ip: *const Instruction = $ip.offset($offset as i16 as isize);
        let index = (*ip).discriminant();
        become HANDLERS_CHECKED[index](ip, $vm, $registers, $constants, $size);
    }};
}

macro_rules! dispatch_offset_unchecked {
    ($ip:expr, $vm:expr, $registers:expr, $constants:expr, $offset:expr, $size:expr) => {{
        let ip: *const Instruction = $ip.offset($offset as i16 as isize);
        let index = (*ip).discriminant();
        become HANDLERS_UNCHECKED[index](ip, $vm, $registers, $constants, $size);
    }};
}

macro_rules! type_check {
    ($cond:expr, $($arg:tt)*) => {
        if std::hint::unlikely(!$cond) {
            return Err(Box::new(kaori_error!(Span::default(), $($arg)*)));
        }
    };
}

type Operand = u8;

const REGISTER: u8 = 0;
const CONSTANT: u8 = 1;

const ADD: u8 = 0;
const SUBTRACT: u8 = 1;
const MULTIPLY: u8 = 2;
const DIVIDE: u8 = 3;
const MODULO: u8 = 4;
const EQUAL: u8 = 5;
const NOT_EQUAL: u8 = 6;
const LESS: u8 = 7;
const LESS_EQUAL: u8 = 8;
const GREATER: u8 = 9;
const GREATER_EQUAL: u8 = 10;

const HANDLERS_CHECKED: [Handler; 52] = [
    opcode_binary::<ADD, REGISTER, REGISTER, false>,
    opcode_binary::<ADD, REGISTER, CONSTANT, false>,
    opcode_binary::<ADD, CONSTANT, REGISTER, false>,
    opcode_binary::<SUBTRACT, REGISTER, REGISTER, false>,
    opcode_binary::<SUBTRACT, REGISTER, CONSTANT, false>,
    opcode_binary::<SUBTRACT, CONSTANT, REGISTER, false>,
    opcode_binary::<MULTIPLY, REGISTER, REGISTER, false>,
    opcode_binary::<MULTIPLY, REGISTER, CONSTANT, false>,
    opcode_binary::<MULTIPLY, CONSTANT, REGISTER, false>,
    opcode_binary::<DIVIDE, REGISTER, REGISTER, false>,
    opcode_binary::<DIVIDE, REGISTER, CONSTANT, false>,
    opcode_binary::<DIVIDE, CONSTANT, REGISTER, false>,
    opcode_binary::<MODULO, REGISTER, REGISTER, false>,
    opcode_binary::<MODULO, REGISTER, CONSTANT, false>,
    opcode_binary::<MODULO, CONSTANT, REGISTER, false>,
    opcode_binary::<EQUAL, REGISTER, REGISTER, false>,
    opcode_binary::<EQUAL, REGISTER, CONSTANT, false>,
    opcode_binary::<EQUAL, CONSTANT, REGISTER, false>,
    opcode_binary::<NOT_EQUAL, REGISTER, REGISTER, false>,
    opcode_binary::<NOT_EQUAL, REGISTER, CONSTANT, false>,
    opcode_binary::<NOT_EQUAL, CONSTANT, REGISTER, false>,
    opcode_binary::<LESS, REGISTER, REGISTER, false>,
    opcode_binary::<LESS, REGISTER, CONSTANT, false>,
    opcode_binary::<LESS, CONSTANT, REGISTER, false>,
    opcode_binary::<LESS_EQUAL, REGISTER, REGISTER, false>,
    opcode_binary::<LESS_EQUAL, REGISTER, CONSTANT, false>,
    opcode_binary::<LESS_EQUAL, CONSTANT, REGISTER, false>,
    opcode_binary::<GREATER, REGISTER, REGISTER, false>,
    opcode_binary::<GREATER, REGISTER, CONSTANT, false>,
    opcode_binary::<GREATER, CONSTANT, REGISTER, false>,
    opcode_binary::<GREATER_EQUAL, REGISTER, REGISTER, false>,
    opcode_binary::<GREATER_EQUAL, REGISTER, CONSTANT, false>,
    opcode_binary::<GREATER_EQUAL, CONSTANT, REGISTER, false>,
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
];

const HANDLERS_UNCHECKED: [Handler; 52] = [
    opcode_binary::<ADD, REGISTER, REGISTER, true>,
    opcode_binary::<ADD, REGISTER, CONSTANT, true>,
    opcode_binary::<ADD, CONSTANT, REGISTER, true>,
    opcode_binary::<SUBTRACT, REGISTER, REGISTER, true>,
    opcode_binary::<SUBTRACT, REGISTER, CONSTANT, true>,
    opcode_binary::<SUBTRACT, CONSTANT, REGISTER, true>,
    opcode_binary::<MULTIPLY, REGISTER, REGISTER, true>,
    opcode_binary::<MULTIPLY, REGISTER, CONSTANT, true>,
    opcode_binary::<MULTIPLY, CONSTANT, REGISTER, true>,
    opcode_binary::<DIVIDE, REGISTER, REGISTER, true>,
    opcode_binary::<DIVIDE, REGISTER, CONSTANT, true>,
    opcode_binary::<DIVIDE, CONSTANT, REGISTER, true>,
    opcode_binary::<MODULO, REGISTER, REGISTER, true>,
    opcode_binary::<MODULO, REGISTER, CONSTANT, true>,
    opcode_binary::<MODULO, CONSTANT, REGISTER, true>,
    opcode_binary::<EQUAL, REGISTER, REGISTER, true>,
    opcode_binary::<EQUAL, REGISTER, CONSTANT, true>,
    opcode_binary::<EQUAL, CONSTANT, REGISTER, true>,
    opcode_binary::<NOT_EQUAL, REGISTER, REGISTER, true>,
    opcode_binary::<NOT_EQUAL, REGISTER, CONSTANT, true>,
    opcode_binary::<NOT_EQUAL, CONSTANT, REGISTER, true>,
    opcode_binary::<LESS, REGISTER, REGISTER, true>,
    opcode_binary::<LESS, REGISTER, CONSTANT, true>,
    opcode_binary::<LESS, CONSTANT, REGISTER, true>,
    opcode_binary::<LESS_EQUAL, REGISTER, REGISTER, true>,
    opcode_binary::<LESS_EQUAL, REGISTER, CONSTANT, true>,
    opcode_binary::<LESS_EQUAL, CONSTANT, REGISTER, true>,
    opcode_binary::<GREATER, REGISTER, REGISTER, true>,
    opcode_binary::<GREATER, REGISTER, CONSTANT, true>,
    opcode_binary::<GREATER, CONSTANT, REGISTER, true>,
    opcode_binary::<GREATER_EQUAL, REGISTER, REGISTER, true>,
    opcode_binary::<GREATER_EQUAL, REGISTER, CONSTANT, true>,
    opcode_binary::<GREATER_EQUAL, CONSTANT, REGISTER, true>,
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
        HANDLERS_CHECKED[index](ip, self, registers, constants, *registers_count).map_err(|e| *e)
    }
}

#[inline(always)]
unsafe fn get_value<const SRC: Operand>(
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
fn opcode_binary<const OP: u8, const SRC1: Operand, const SRC2: Operand, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (dest, src1, src2) = match (OP, SRC1, SRC2) {
            (ADD, REGISTER, REGISTER) => {
                let Instruction::AddRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (ADD, REGISTER, CONSTANT) => {
                let Instruction::AddRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (ADD, CONSTANT, REGISTER) => {
                let Instruction::AddKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (SUBTRACT, REGISTER, REGISTER) => {
                let Instruction::SubtractRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (SUBTRACT, REGISTER, CONSTANT) => {
                let Instruction::SubtractRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (SUBTRACT, CONSTANT, REGISTER) => {
                let Instruction::SubtractKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (MULTIPLY, REGISTER, REGISTER) => {
                let Instruction::MultiplyRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (MULTIPLY, REGISTER, CONSTANT) => {
                let Instruction::MultiplyRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (MULTIPLY, CONSTANT, REGISTER) => {
                let Instruction::MultiplyKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (DIVIDE, REGISTER, REGISTER) => {
                let Instruction::DivideRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (DIVIDE, REGISTER, CONSTANT) => {
                let Instruction::DivideRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (DIVIDE, CONSTANT, REGISTER) => {
                let Instruction::DivideKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (MODULO, REGISTER, REGISTER) => {
                let Instruction::ModuloRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (MODULO, REGISTER, CONSTANT) => {
                let Instruction::ModuloRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (MODULO, CONSTANT, REGISTER) => {
                let Instruction::ModuloKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (EQUAL, REGISTER, REGISTER) => {
                let Instruction::EqualRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (EQUAL, REGISTER, CONSTANT) => {
                let Instruction::EqualRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (EQUAL, CONSTANT, REGISTER) => {
                let Instruction::EqualKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (NOT_EQUAL, REGISTER, REGISTER) => {
                let Instruction::NotEqualRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (NOT_EQUAL, REGISTER, CONSTANT) => {
                let Instruction::NotEqualRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (NOT_EQUAL, CONSTANT, REGISTER) => {
                let Instruction::NotEqualKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (LESS, REGISTER, REGISTER) => {
                let Instruction::LessRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (LESS, REGISTER, CONSTANT) => {
                let Instruction::LessRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (LESS, CONSTANT, REGISTER) => {
                let Instruction::LessKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (LESS_EQUAL, REGISTER, REGISTER) => {
                let Instruction::LessEqualRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (LESS_EQUAL, REGISTER, CONSTANT) => {
                let Instruction::LessEqualRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (LESS_EQUAL, CONSTANT, REGISTER) => {
                let Instruction::LessEqualKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (GREATER, REGISTER, REGISTER) => {
                let Instruction::GreaterRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (GREATER, REGISTER, CONSTANT) => {
                let Instruction::GreaterRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (GREATER, CONSTANT, REGISTER) => {
                let Instruction::GreaterKR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (GREATER_EQUAL, REGISTER, REGISTER) => {
                let Instruction::GreaterEqualRR { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (GREATER_EQUAL, REGISTER, CONSTANT) => {
                let Instruction::GreaterEqualRK { dest, src1, src2 } = *ip else {
                    unreachable_unchecked()
                };
                (dest, src1, src2)
            }
            (GREATER_EQUAL, CONSTANT, REGISTER) => {
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
            match OP {
                ADD => type_check!(
                    lhs.is_number() && rhs.is_number(),
                    "cannot add {:?} and {:?}, both operands must be numbers",
                    lhs,
                    rhs
                ),
                SUBTRACT => type_check!(
                    lhs.is_number() && rhs.is_number(),
                    "cannot subtract {:?} from {:?}, both operands must be numbers",
                    rhs,
                    lhs
                ),
                MULTIPLY => type_check!(
                    lhs.is_number() && rhs.is_number(),
                    "cannot multiply {:?} and {:?}, both operands must be numbers",
                    lhs,
                    rhs
                ),
                DIVIDE => type_check!(
                    lhs.is_number() && rhs.is_number(),
                    "cannot divide {:?} by {:?}, both operands must be numbers",
                    lhs,
                    rhs
                ),
                MODULO => type_check!(
                    lhs.is_number() && rhs.is_number(),
                    "cannot compute {:?} modulo {:?}, both operands must be numbers",
                    lhs,
                    rhs
                ),
                LESS | LESS_EQUAL | GREATER | GREATER_EQUAL => type_check!(
                    lhs.is_number() && rhs.is_number(),
                    "cannot compare {:?} and {:?}, both operands must be numbers",
                    lhs,
                    rhs
                ),
                EQUAL | NOT_EQUAL => {}
                _ => unreachable_unchecked(),
            }
        }

        let value = match OP {
            ADD => Value::number(lhs.as_number() + rhs.as_number()),
            SUBTRACT => Value::number(lhs.as_number() - rhs.as_number()),
            MULTIPLY => Value::number(lhs.as_number() * rhs.as_number()),
            DIVIDE => Value::number(lhs.as_number() / rhs.as_number()),
            MODULO => Value::number(lhs.as_number() % rhs.as_number()),
            EQUAL => Value::boolean(lhs == rhs),
            NOT_EQUAL => Value::boolean(lhs != rhs),
            LESS => Value::boolean(lhs.as_number() < rhs.as_number()),
            LESS_EQUAL => Value::boolean(lhs.as_number() <= rhs.as_number()),
            GREATER => Value::boolean(lhs.as_number() > rhs.as_number()),
            GREATER_EQUAL => Value::boolean(lhs.as_number() >= rhs.as_number()),
            _ => unreachable_unchecked(),
        };

        set_value(dest, value, registers);

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
fn opcode_move<const SRC: Operand, const UNCHECKED: bool>(
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
fn opcode_set_field<const KEY: Operand, const VAL: Operand, const UNCHECKED: bool>(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
    size: u8,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let (object, key, value) = match (KEY, VAL) {
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
        let val = get_value::<VAL>(value, constants, registers);

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
fn opcode_get_field<const KEY: Operand, const UNCHECKED: bool>(
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

            HANDLERS_CHECKED[index](ip, vm, callee_registers, constants, registers_count)
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

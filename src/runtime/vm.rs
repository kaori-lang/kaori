use std::hint::unreachable_unchecked;

use super::{function::Function, gc::Gc};
use crate::error::kaori_error::KaoriError;
use crate::kaori_error;
use crate::lexer::span::Span;
use crate::runtime::value::{TYPE_BOOLEAN, TYPE_DICT, TYPE_FUNCTION, TYPE_NUMBER};
use crate::{bytecode::instruction::Instruction, runtime::value::Value};

type Handler = fn(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>>;

macro_rules! dispatch_next {
    ($ip:expr, $vm:expr, $registers:expr, $constants:expr) => {{
        let ip: *const Instruction = $ip.add(1);
        let index = (*ip).discriminant();
        become OPCODE_HANDLERS[index](ip, $vm, $registers, $constants);
    }};
}

macro_rules! dispatch_offset {
    ($ip:expr, $vm:expr, $registers:expr, $constants:expr, $offset:expr) => {{
        let ip: *const Instruction = $ip.offset($offset as i16 as isize);
        let index = (*ip).discriminant();
        become OPCODE_HANDLERS[index](ip, $vm, $registers, $constants);
    }};
}

macro_rules! type_error {
    ($($arg:tt)*) => {{
        return Err(Box::new(kaori_error!(Span::default(), $($arg)*)));
    }};
}

const OPCODE_HANDLERS: [Handler; 54] = [
    opcode_add_rr,           // 0  AddRR
    opcode_add_rk,           // 1  AddRK
    opcode_add_kr,           // 2  AddKR
    opcode_subtract_rr,      // 3  SubtractRR
    opcode_subtract_rk,      // 4  SubtractRK
    opcode_subtract_kr,      // 5  SubtractKR
    opcode_multiply_rr,      // 6  MultiplyRR
    opcode_multiply_rk,      // 7  MultiplyRK
    opcode_multiply_kr,      // 8  MultiplyKR
    opcode_divide_rr,        // 9  DivideRR
    opcode_divide_rk,        // 10 DivideRK
    opcode_divide_kr,        // 11 DivideKR
    opcode_modulo_rr,        // 12 ModuloRR
    opcode_modulo_rk,        // 13 ModuloRK
    opcode_modulo_kr,        // 14 ModuloKR
    opcode_power_rr,         // 15 PowerRR
    opcode_power_rk,         // 16 PowerRK
    opcode_power_kr,         // 17 PowerKR
    opcode_equal_rr,         // 18 EqualRR
    opcode_equal_rk,         // 19 EqualRK
    opcode_equal_kr,         // 20 EqualKR
    opcode_not_equal_rr,     // 21 NotEqualRR
    opcode_not_equal_rk,     // 22 NotEqualRK
    opcode_not_equal_kr,     // 23 NotEqualKR
    opcode_greater_rr,       // 24 GreaterRR
    opcode_greater_rk,       // 25 GreaterRK
    opcode_greater_kr,       // 26 GreaterKR
    opcode_greater_equal_rr, // 27 GreaterEqualRR
    opcode_greater_equal_rk, // 28 GreaterEqualRK
    opcode_greater_equal_kr, // 29 GreaterEqualKR
    opcode_not_k,            // 30 NotK
    opcode_not_r,            // 31 NotR
    opcode_negate_k,         // 32 NegateK
    opcode_negate_r,         // 33 NegateR
    opcode_move_r,           // 34 MoveR
    opcode_move_k,           // 35 MoveK
    opcode_create_dict,      // 36 CreateDict
    opcode_set_field_rr,     // 37 SetFieldRR
    opcode_set_field_rk,     // 38 SetFieldRK
    opcode_set_field_kr,     // 39 SetFieldKR
    opcode_set_field_kk,     // 40 SetFieldKK
    opcode_get_field_r,      // 41 GetFieldR
    opcode_get_field_k,      // 42 GetFieldK
    opcode_call_k,           // 43 CallK
    opcode_call_r,           // 44 CallR
    opcode_return_k,         // 45 ReturnK
    opcode_return_r,         // 46 ReturnR
    opcode_jump,             // 47 Jump
    opcode_jump_if_true_k,   // 48 JumpIfTrueK
    opcode_jump_if_true_r,   // 49 JumpIfTrueR
    opcode_jump_if_false_k,  // 50 JumpIfFalseK
    opcode_jump_if_false_r,  // 51 JumpIfFalseR
    opcode_print_k,          // 52 PrintK
    opcode_print_r,          // 53 PrintR
];

pub struct Vm {
    pub registers: Vec<Value>,
    pub frames: Vec<(*mut Value, usize)>,
    pub gc: Gc,
}

impl Vm {
    pub fn new(gc: Gc) -> Self {
        Self {
            registers: vec![Value::default(); 4096],
            frames: Vec::new(),
            gc,
        }
    }

    pub fn run(&mut self, entry: &Function) -> Result<Value, KaoriError> {
        let Function {
            instructions,
            registers_count,
            constants,
        } = entry;
        let registers = self.push_frame(*registers_count as usize);
        let constants = constants.as_ptr();
        let ip = instructions.as_ptr();
        let index = unsafe { (*ip).discriminant() };

        OPCODE_HANDLERS[index](ip, self, registers, constants).map_err(|e| *e)
    }

    pub fn push_frame(&mut self, size: usize) -> *mut Value {
        let offset = self
            .frames
            .last()
            .map(|(ptr, len)| unsafe {
                ptr.add(*len).offset_from(self.registers.as_ptr()) as usize
            })
            .unwrap_or(0);

        let ptr = unsafe { self.registers.as_mut_ptr().add(offset) };
        self.frames.push((ptr, size));
        ptr
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop();
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
fn opcode_add_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::AddRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number() + rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot add {:?} and {:?}, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_add_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::AddRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number() + rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot add {:?} and {:?}, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_add_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::AddKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number() + rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot add {:?} and {:?}, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_subtract_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::SubtractRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number() - rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot subtract {:?} from {:?}, both operands must be numbers",
                rhs,
                lhs
            )
        }
    }
}

#[inline(never)]
fn opcode_subtract_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::SubtractRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number() - rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot subtract {:?} from {:?}, both operands must be numbers",
                rhs,
                lhs
            )
        }
    }
}

#[inline(never)]
fn opcode_subtract_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::SubtractKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number() - rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot subtract {:?} from {:?}, both operands must be numbers",
                rhs,
                lhs
            )
        }
    }
}

#[inline(never)]
fn opcode_multiply_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::MultiplyRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number() * rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot multiply {:?} and {:?}, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_multiply_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::MultiplyRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number() * rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot multiply {:?} and {:?}, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_multiply_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::MultiplyKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number() * rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot multiply {:?} and {:?}, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_divide_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::DivideRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number() / rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot divide {:?} by {:?}, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_divide_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::DivideRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number() / rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot divide {:?} by {:?}, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_divide_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::DivideKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number() / rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot divide {:?} by {:?}, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_modulo_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::ModuloRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number() % rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compute {:?} modulo {:?}, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_modulo_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::ModuloRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number() % rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compute {:?} modulo {:?}, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_modulo_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::ModuloKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number() % rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compute {:?} modulo {:?}, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_power_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::PowerRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number().powf(rhs.as_number())),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot raise {:?} to the power of {:?}, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_power_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::PowerRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number().powf(rhs.as_number())),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot raise {:?} to the power of {:?}, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_power_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::PowerKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::number(lhs.as_number().powf(rhs.as_number())),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot raise {:?} to the power of {:?}, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_equal_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::EqualRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::boolean(lhs.as_number() == rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with ==, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_equal_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::EqualRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::boolean(lhs.as_number() == rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with ==, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_equal_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::EqualKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::boolean(lhs.as_number() == rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with ==, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_not_equal_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::NotEqualRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::boolean(lhs.as_number() != rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with !=, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_not_equal_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::NotEqualRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::boolean(lhs.as_number() != rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with !=, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_not_equal_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::NotEqualKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::boolean(lhs.as_number() != rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with !=, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_greater_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GreaterRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);

        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::boolean(lhs.as_number() > rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with >, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_greater_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GreaterRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::boolean(lhs.as_number() > rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with >, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_greater_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GreaterKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::boolean(lhs.as_number() > rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with >, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_greater_equal_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GreaterEqualRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::boolean(lhs.as_number() >= rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with >=, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_greater_equal_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GreaterEqualRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::boolean(lhs.as_number() >= rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with >=, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_greater_equal_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GreaterEqualKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);
        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
            set_value(
                dest,
                Value::boolean(lhs.as_number() >= rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with >=, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_negate_r(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::NegateR { dest, src } = *ip else {
            unreachable_unchecked()
        };
        let value = get_register_value(src, registers);
        if value.tag() == TYPE_NUMBER {
            set_value(dest, Value::number(-value.as_number()), registers);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot negate {:?}, operand must be a number", value)
        }
    }
}

#[inline(never)]
fn opcode_negate_k(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::NegateK { dest, src } = *ip else {
            unreachable_unchecked()
        };
        let value = get_constant_value(src, constants);
        if value.tag() == TYPE_NUMBER {
            set_value(dest, Value::number(-value.as_number()), registers);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot negate {:?}, operand must be a number", value)
        }
    }
}

#[inline(never)]
fn opcode_not_r(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::NotR { dest, src } = *ip else {
            unreachable_unchecked()
        };
        let value = get_register_value(src, registers);
        if value.tag() == TYPE_BOOLEAN {
            set_value(dest, Value::boolean(!value.as_boolean()), registers);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot apply ! to {:?}, operand must be a boolean", value)
        }
    }
}

#[inline(never)]
fn opcode_not_k(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::NotK { dest, src } = *ip else {
            unreachable_unchecked()
        };
        let value = get_constant_value(src, constants);
        if value.tag() == TYPE_BOOLEAN {
            set_value(dest, Value::boolean(!value.as_boolean()), registers);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot apply ! to {:?}, operand must be a boolean", value)
        }
    }
}

#[inline(never)]
fn opcode_move_r(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::MoveR { dest, src } = *ip else {
            unreachable_unchecked()
        };
        set_value(dest, get_register_value(src, registers), registers);
        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_move_k(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::MoveK { dest, src } = *ip else {
            unreachable_unchecked()
        };
        set_value(dest, get_constant_value(src, constants), registers);
        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_create_dict(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::CreateDict { dest } = *ip else {
            unreachable_unchecked()
        };
        set_value(dest, vm.gc.allocate_dict(), registers);
        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_set_field_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::SetFieldRR { object, key, value } = *ip else {
            unreachable_unchecked()
        };
        let obj = get_register_value(object, registers);
        let key = get_register_value(key, registers);
        let val = get_register_value(value, registers);
        if obj.tag() == TYPE_DICT {
            obj.as_dict().insert(key, val);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot set field on {:?}, value is not a dict", obj)
        }
    }
}

#[inline(never)]
fn opcode_set_field_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::SetFieldRK { object, key, value } = *ip else {
            unreachable_unchecked()
        };
        let obj = get_register_value(object, registers);
        let key = get_register_value(key, registers);
        let val = get_constant_value(value, constants);
        if obj.tag() == TYPE_DICT {
            obj.as_dict().insert(key, val);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot set field on {:?}, value is not a dict", obj)
        }
    }
}

#[inline(never)]
fn opcode_set_field_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::SetFieldKR { object, key, value } = *ip else {
            unreachable_unchecked()
        };
        let obj = get_register_value(object, registers);
        let key = get_constant_value(key, constants);
        let val = get_register_value(value, registers);
        if obj.tag() == TYPE_DICT {
            obj.as_dict().insert(key, val);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot set field on {:?}, value is not a dict", obj)
        }
    }
}

#[inline(never)]
fn opcode_set_field_kk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::SetFieldKK { object, key, value } = *ip else {
            unreachable_unchecked()
        };
        let obj = get_register_value(object, registers);
        let key = get_constant_value(key, constants);
        let val = get_constant_value(value, constants);
        if obj.tag() == TYPE_DICT {
            obj.as_dict().insert(key, val);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot set field on {:?}, value is not a dict", obj)
        }
    }
}

#[inline(never)]
fn opcode_get_field_r(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GetFieldR { dest, object, key } = *ip else {
            unreachable_unchecked()
        };
        let obj = get_register_value(object, registers);
        let key = get_register_value(key, registers);
        if obj.tag() == TYPE_DICT {
            let value = obj.as_dict().get(&key).copied().unwrap_or_default();
            set_value(dest, value, registers);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot get field from {:?}, value is not a dict", obj)
        }
    }
}

#[inline(never)]
fn opcode_get_field_k(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GetFieldK { dest, object, key } = *ip else {
            unreachable_unchecked()
        };
        let obj = get_register_value(object, registers);
        let key = get_constant_value(key, constants);
        if obj.tag() == TYPE_DICT {
            let value = obj.as_dict().get(&key).copied().unwrap_or_default();
            set_value(dest, value, registers);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot get field from {:?}, value is not a dict", obj)
        }
    }
}

#[inline(never)]
fn opcode_call_r(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::CallR { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let callee = get_register_value(src, registers);

        if callee.tag() == TYPE_FUNCTION {
            let return_value = {
                let Function {
                    ref instructions,
                    registers_count,
                    ref constants,
                } = *callee.as_function();

                let registers = vm.push_frame(registers_count as usize);
                let constants = constants.as_ptr();
                let ip = instructions.as_ptr();
                let index = (*ip).discriminant();
                OPCODE_HANDLERS[index](ip, vm, registers, constants)
            }?;
            set_value(dest, return_value, registers);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot call {:?}, value is not a function", callee)
        }
    }
}

#[inline(never)]
fn opcode_call_k(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::CallK { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let callee = get_constant_value(src, constants);
        if callee.tag() == TYPE_FUNCTION {
            let return_value = {
                let Function {
                    ref instructions,
                    registers_count,
                    ref constants,
                } = *callee.as_function();
                let registers = vm.push_frame(registers_count as usize);
                let constants = constants.as_ptr();
                let ip = instructions.as_ptr();
                let index = (*ip).discriminant();
                OPCODE_HANDLERS[index](ip, vm, registers, constants)
            }?;
            set_value(dest, return_value, registers);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot call {:?}, value is not a function", callee)
        }
    }
}

#[inline(never)]
fn opcode_return_r(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    _constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::ReturnR { src } = *ip else {
            unreachable_unchecked()
        };
        let value = get_register_value(src, registers);
        vm.pop_frame();
        Ok(value)
    }
}

#[inline(never)]
fn opcode_return_k(
    ip: *const Instruction,
    vm: &mut Vm,
    _registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::ReturnK { src } = *ip else {
            unreachable_unchecked()
        };
        let value = get_constant_value(src, constants);
        vm.pop_frame();
        Ok(value)
    }
}

#[inline(never)]
fn opcode_jump(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Jump { offset } = *ip else {
            unreachable_unchecked()
        };
        dispatch_offset!(ip, vm, registers, constants, offset)
    }
}

#[inline(never)]
fn opcode_jump_if_true_r(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::JumpIfTrueR { src, offset } = *ip else {
            unreachable_unchecked()
        };
        let value = get_register_value(src, registers);
        if value.tag() == TYPE_BOOLEAN {
            if value.as_boolean() {
                dispatch_offset!(ip, vm, registers, constants, offset)
            } else {
                dispatch_next!(ip, vm, registers, constants)
            }
        } else {
            type_error!(
                "cannot use {:?} as a condition, value must be a boolean",
                value
            )
        }
    }
}

#[inline(never)]
fn opcode_jump_if_true_k(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::JumpIfTrueK { src, offset } = *ip else {
            unreachable_unchecked()
        };
        let value = get_constant_value(src, constants);
        if value.tag() == TYPE_BOOLEAN {
            if value.as_boolean() {
                dispatch_offset!(ip, vm, registers, constants, offset)
            } else {
                dispatch_next!(ip, vm, registers, constants)
            }
        } else {
            type_error!(
                "cannot use {:?} as a condition, value must be a boolean",
                value
            )
        }
    }
}

#[inline(never)]
fn opcode_jump_if_false_r(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::JumpIfFalseR { src, offset } = *ip else {
            unreachable_unchecked()
        };
        let value = get_register_value(src, registers);
        if value.tag() == TYPE_BOOLEAN {
            if value.as_boolean() {
                dispatch_next!(ip, vm, registers, constants)
            } else {
                dispatch_offset!(ip, vm, registers, constants, offset)
            }
        } else {
            type_error!(
                "cannot use {:?} as a condition, value must be a boolean",
                value
            )
        }
    }
}

#[inline(never)]
fn opcode_jump_if_false_k(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::JumpIfFalseK { src, offset } = *ip else {
            unreachable_unchecked()
        };
        let value = get_constant_value(src, constants);
        if value.tag() == TYPE_BOOLEAN {
            if value.as_boolean() {
                dispatch_next!(ip, vm, registers, constants)
            } else {
                dispatch_offset!(ip, vm, registers, constants, offset)
            }
        } else {
            type_error!(
                "cannot use {:?} as a condition, value must be a boolean",
                value
            )
        }
    }
}

#[inline(never)]
fn opcode_print_r(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::PrintR { src } = *ip else {
            unreachable_unchecked()
        };
        println!("{:?}", get_register_value(src, registers));
        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_print_k(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::PrintK { src } = *ip else {
            unreachable_unchecked()
        };
        println!("{:?}", get_constant_value(src, constants));
        dispatch_next!(ip, vm, registers, constants)
    }
}

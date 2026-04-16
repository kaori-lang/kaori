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

const OPCODE_HANDLERS: [Handler; 60] = [
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
    opcode_power_rr,
    opcode_power_rk,
    opcode_power_kr,
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
    opcode_not_k,
    opcode_not_r,
    opcode_negate_k,
    opcode_negate_r,
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
    opcode_jump_if_true_k,
    opcode_jump_if_true_r,
    opcode_jump_if_false_k,
    opcode_jump_if_false_r,
    opcode_print_k,
    opcode_print_r,
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
        let ptr = match self.frames.last() {
            Some(&(ptr, len)) => unsafe { ptr.add(len) },
            None => self.registers.as_mut_ptr(),
        };

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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
fn opcode_less_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::LessRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };
        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);

        if lhs.is_number() && rhs.is_number() {
            set_value(
                dest,
                Value::boolean(lhs.as_number() < rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with <, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_less_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::LessRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);

        if lhs.is_number() && rhs.is_number() {
            set_value(
                dest,
                Value::boolean(lhs.as_number() < rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with <, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_less_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::LessKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);

        if lhs.is_number() && rhs.is_number() {
            set_value(
                dest,
                Value::boolean(lhs.as_number() < rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with <, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_less_equal_rr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::LessEqualRR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_register_value(src1, registers);
        let rhs = get_register_value(src2, registers);

        if lhs.is_number() && rhs.is_number() {
            set_value(
                dest,
                Value::boolean(lhs.as_number() <= rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with <=, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_less_equal_rk(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::LessEqualRK { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_register_value(src1, registers);
        let rhs = get_constant_value(src2, constants);

        if lhs.is_number() && rhs.is_number() {
            set_value(
                dest,
                Value::boolean(lhs.as_number() <= rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with <=, both operands must be numbers",
                lhs,
                rhs
            )
        }
    }
}

#[inline(never)]
fn opcode_less_equal_kr(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::LessEqualKR { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_constant_value(src1, constants);
        let rhs = get_register_value(src2, registers);

        if lhs.is_number() && rhs.is_number() {
            set_value(
                dest,
                Value::boolean(lhs.as_number() <= rhs.as_number()),
                registers,
            );
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!(
                "cannot compare {:?} and {:?} with <=, both operands must be numbers",
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

        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if lhs.is_number() && rhs.is_number() {
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
        if value.is_number() {
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
        if value.is_number() {
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
        if value.is_boolean() {
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
        if value.is_boolean() {
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
        let object = get_register_value(object, registers);
        let key = get_register_value(key, registers);
        let val = get_register_value(value, registers);
        if object.is_dict() {
            object.as_dict().insert(key, val);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot set field on {:?}, value is not a dict", object)
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
        let object = get_register_value(object, registers);
        let key = get_register_value(key, registers);
        let val = get_constant_value(value, constants);
        if object.is_dict() {
            object.as_dict().insert(key, val);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot set field on {:?}, value is not a dict", object)
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
        let object = get_register_value(object, registers);
        let key = get_constant_value(key, constants);
        let val = get_register_value(value, registers);
        if object.is_dict() {
            object.as_dict().insert(key, val);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot set field on {:?}, value is not a dict", object)
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
        let object = get_register_value(object, registers);
        let key = get_constant_value(key, constants);
        let val = get_constant_value(value, constants);
        if object.is_dict() {
            object.as_dict().insert(key, val);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot set field on {:?}, value is not a dict", object)
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
        let object = get_register_value(object, registers);
        let key = get_register_value(key, registers);
        if object.is_dict() {
            let value = object.as_dict().get(&key).copied().unwrap_or_default();
            set_value(dest, value, registers);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot get field from {:?}, value is not a dict", object)
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
        let object = get_register_value(object, registers);
        let key = get_constant_value(key, constants);
        if object.is_dict() {
            let value = object.as_dict().get(&key).copied().unwrap_or_default();
            set_value(dest, value, registers);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot get field from {:?}, value is not a dict", object)
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

        if callee.is_function() {
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
        if callee.is_function() {
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
        if value.is_boolean() {
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
        if value.is_boolean() {
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
        if value.is_boolean() {
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
        if value.is_boolean() {
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

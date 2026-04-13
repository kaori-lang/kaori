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
        //core::hint::cold_path();
        return Err(Box::new(kaori_error!(Span::default(), $($arg)*)));
    }};
}

const OPCODE_HANDLERS: [Handler; 24] = [
    opcode_add,
    opcode_subtract,
    opcode_multiply,
    opcode_divide,
    opcode_modulo,
    opcode_power,
    opcode_equal,
    opcode_not_equal,
    opcode_greater,
    opcode_greater_equal,
    opcode_less,
    opcode_less_equal,
    opcode_negate,
    opcode_not,
    opcode_move,
    opcode_create_dict,
    opcode_set_field,
    opcode_get_field,
    opcode_call,
    opcode_return,
    opcode_jump,
    opcode_jump_if_true,
    opcode_jump_if_false,
    opcode_print,
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
fn get_value(index: i16, registers: *mut Value, constants: *const Value) -> Value {
    unsafe {
        if index < 0 {
            *constants.add(-(index + 1) as usize)
        } else {
            *registers.add(index as usize)
        }
    }
}

#[inline(always)]
fn set_value(index: u16, value: Value, registers: *mut Value) {
    unsafe {
        *registers.add(index as usize) = value;
    }
}

#[inline(never)]
fn opcode_move(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Move { dest, src } = *ip else {
            unreachable_unchecked()
        };

        set_value(dest, get_value(src, registers, constants), registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_add(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Add { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants);
        let rhs = get_value(src2, registers, constants);

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
fn opcode_subtract(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Subtract { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants);
        let rhs = get_value(src2, registers, constants);

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
fn opcode_multiply(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Multiply { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants);
        let rhs = get_value(src2, registers, constants);

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
fn opcode_divide(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Divide { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants);
        let rhs = get_value(src2, registers, constants);

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
fn opcode_modulo(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Modulo { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants);
        let rhs = get_value(src2, registers, constants);

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
fn opcode_power(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Power { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants);
        let rhs = get_value(src2, registers, constants);

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
fn opcode_equal(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Equal { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants);
        let rhs = get_value(src2, registers, constants);

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
fn opcode_not_equal(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::NotEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants);
        let rhs = get_value(src2, registers, constants);

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
fn opcode_greater(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Greater { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants);
        let rhs = get_value(src2, registers, constants);

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
fn opcode_greater_equal(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GreaterEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants);
        let rhs = get_value(src2, registers, constants);

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
fn opcode_less(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Less { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants);
        let rhs = get_value(src2, registers, constants);

        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
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
fn opcode_less_equal(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::LessEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants);
        let rhs = get_value(src2, registers, constants);

        if lhs.tag() == TYPE_NUMBER && rhs.tag() == TYPE_NUMBER {
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
fn opcode_negate(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Negate { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let value = get_value(src, registers, constants);

        if value.tag() == TYPE_NUMBER {
            set_value(dest, Value::number(-value.as_number()), registers);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot negate {:?}, operand must be a number", value)
        }
    }
}

#[inline(never)]
fn opcode_not(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Not { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let value = get_value(src, registers, constants);

        if value.tag() == TYPE_BOOLEAN {
            set_value(dest, Value::boolean(!value.as_boolean()), registers);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot apply ! to {:?}, operand must be a boolean", value)
        }
    }
}

#[inline(never)]
fn opcode_call(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Call { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let callee = get_value(src, registers, constants);

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
fn opcode_return(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Return { src } = *ip else {
            unreachable_unchecked()
        };

        let value = get_value(src, registers, constants);
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
fn opcode_jump_if_true(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::JumpIfTrue { src, offset } = *ip else {
            unreachable_unchecked()
        };

        let value = get_value(src, registers, constants);

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
fn opcode_jump_if_false(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::JumpIfFalse { src, offset } = *ip else {
            unreachable_unchecked()
        };

        let value = get_value(src, registers, constants);

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
fn opcode_print(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::Print { src } = *ip else {
            unreachable_unchecked()
        };

        println!("{:?}", get_value(src, registers, constants));

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
fn opcode_set_field(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::SetField { object, key, value } = *ip else {
            unreachable_unchecked()
        };

        let key = get_value(key, registers, constants);
        let value = get_value(value, registers, constants);
        let object = get_value(object as i16, registers, constants);

        if object.tag() == TYPE_DICT {
            object.as_dict().insert(key, value);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot set field on {:?}, value is not a dict", object)
        }
    }
}

#[inline(never)]
fn opcode_get_field(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Result<Value, Box<KaoriError>> {
    unsafe {
        let Instruction::GetField { dest, object, key } = *ip else {
            unreachable_unchecked()
        };

        let object = get_value(object, registers, constants);
        let key = get_value(key, registers, constants);

        if object.tag() == TYPE_DICT {
            let value = object.as_dict().get(&key).copied().unwrap_or_default();
            set_value(dest, value, registers);
            dispatch_next!(ip, vm, registers, constants)
        } else {
            type_error!("cannot get field from {:?}, value is not a dict", object)
        }
    }
}

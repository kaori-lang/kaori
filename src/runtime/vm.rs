use std::hint::unreachable_unchecked;

use crate::{bytecode::instruction::Instruction, runtime::value::Value};

use super::{function::Function, gc::Gc};

type Handler = fn(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value;

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

    pub fn run(&mut self, entry: &Function) {
        let Function {
            instructions,
            registers_count,
            constants,
        } = entry;

        let registers = self.push_frame(*registers_count as usize);
        let constants = constants.as_ptr();
        let ip = instructions.as_ptr();
        let index = unsafe { (*ip).discriminant() };

        OPCODE_HANDLERS[index](ip, self, registers, constants);

        self.pop_frame();
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
) -> Value {
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
) -> Value {
    unsafe {
        let Instruction::Add { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();
        set_value(dest, Value::number(lhs + rhs), registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_subtract(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::Subtract { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();
        set_value(dest, Value::number(lhs - rhs), registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_multiply(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::Multiply { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();
        set_value(dest, Value::number(lhs * rhs), registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_divide(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::Divide { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();
        set_value(dest, Value::number(lhs / rhs), registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_modulo(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::Modulo { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();
        set_value(dest, Value::number(lhs % rhs), registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_power(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::Power { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();
        set_value(dest, Value::number(lhs.powf(rhs)), registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_equal(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::Equal { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();
        set_value(dest, Value::boolean(lhs == rhs), registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_not_equal(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::NotEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();
        set_value(dest, Value::boolean(lhs != rhs), registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_greater(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::Greater { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();
        set_value(dest, Value::boolean(lhs > rhs), registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_greater_equal(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::GreaterEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();
        set_value(dest, Value::boolean(lhs >= rhs), registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_less(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::Less { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();
        set_value(dest, Value::boolean(lhs < rhs), registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_less_equal(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::LessEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked()
        };

        let lhs = get_value(src1, registers, constants).expect_number();
        let rhs = get_value(src2, registers, constants).expect_number();
        set_value(dest, Value::boolean(lhs <= rhs), registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_negate(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::Negate { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let value = get_value(src, registers, constants).expect_number();
        set_value(dest, Value::number(-value), registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_not(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::Not { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let value = get_value(src, registers, constants).expect_boolean();
        set_value(dest, Value::boolean(!value), registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_call(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::Call { dest, src } = *ip else {
            unreachable_unchecked()
        };

        let return_value = {
            let Function {
                ref instructions,
                registers_count,
                ref constants,
            } = *get_value(src, registers, constants).expect_function();

            let registers = vm.push_frame(registers_count as usize);
            let constants = constants.as_ptr();
            let ip = instructions.as_ptr();

            let index = (*ip).discriminant();

            OPCODE_HANDLERS[index](ip, vm, registers, constants)
        };

        set_value(dest, return_value, registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_return(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::Return { src } = *ip else {
            unreachable_unchecked()
        };

        let value = get_value(src, registers, constants);

        vm.pop_frame();

        value
    }
}

#[inline(never)]
fn opcode_jump(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
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
) -> Value {
    unsafe {
        let Instruction::JumpIfTrue { src, offset } = *ip else {
            unreachable_unchecked()
        };

        match get_value(src, registers, constants).expect_boolean() {
            true => dispatch_offset!(ip, vm, registers, constants, offset),
            false => dispatch_next!(ip, vm, registers, constants),
        }
    }
}

#[inline(never)]
fn opcode_jump_if_false(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::JumpIfFalse { src, offset } = *ip else {
            unreachable_unchecked()
        };

        match get_value(src, registers, constants).expect_boolean() {
            true => dispatch_next!(ip, vm, registers, constants),
            false => dispatch_offset!(ip, vm, registers, constants, offset),
        }
    }
}

#[inline(never)]
fn opcode_print(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::Print { src } = *ip else {
            unreachable_unchecked()
        };

        let value = get_value(src, registers, constants);

        println!("{:?}", value);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_create_dict(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::CreateDict { dest } = *ip else {
            unreachable_unchecked()
        };

        let value = vm.gc.allocate_dict();
        set_value(dest, value, registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_set_field(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::SetField { object, key, value } = *ip else {
            unreachable_unchecked()
        };

        let key = get_value(key, registers, constants);
        let value = get_value(value, registers, constants);
        let object = get_value(object as i16, registers, constants);

        (*object.as_dict()).insert(key, value);

        dispatch_next!(ip, vm, registers, constants)
    }
}

#[inline(never)]
fn opcode_get_field(
    ip: *const Instruction,
    vm: &mut Vm,
    registers: *mut Value,
    constants: *const Value,
) -> Value {
    unsafe {
        let Instruction::GetField { dest, object, key } = *ip else {
            unreachable_unchecked()
        };

        let object = get_value(object, registers, constants);
        let key = get_value(key, registers, constants);

        let value = (*object.as_dict()).get(&key).copied().unwrap_or_default();

        set_value(dest, value, registers);

        dispatch_next!(ip, vm, registers, constants)
    }
}

use std::hint::unreachable_unchecked;

use super::gc::Gc;
use crate::bytecode::Function;
use crate::diagnostics::error::Error;

use crate::program::{CONSTANT_POOL, FUNCTIONS};
use crate::report_error;
use crate::runtime::gc::Closure;
use crate::{bytecode::instruction::Instruction, runtime::value::Value};

type Handler = unsafe extern "rust-preserve-none" fn(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>>;

macro_rules! dispatch_next {
    ($index:expr, $instructions:expr, $registers:expr, $vm:expr) => {{
        let index = $index + 1;
        let instruction = $instructions[index];
        let discriminant = instruction.discriminant();

        become HANDLERS[discriminant](index, $instructions, $registers, $vm);
    }};
}

macro_rules! dispatch_offset {
    ($index:expr, $instructions:expr, $registers:expr, $vm:expr, $offset:expr) => {{
        let index = ($index as isize + $offset as isize) as usize;
        let instruction = $instructions[index];
        let discriminant = instruction.discriminant();

        become HANDLERS[discriminant](index, $instructions, $registers, $vm);
    }};
}

macro_rules! type_check {
    ($cond:expr, $($arg:tt)*) => {{
        if std::hint::unlikely(!$cond) {
            return Err(Box::new(report_error!($($arg)*)));
        }
    }};
}

struct Frame {
    return_index: usize,
    return_instructions: &'static [Instruction],
    return_registers: Registers,
    return_dest: u8,
    size: u8,
}

pub struct Vm {
    registers: [Value; 4096],
    stack: Vec<Frame>,
    gc: Gc,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            registers: [Value::default(); 4096],
            stack: Vec::new(),
            gc: Gc::default(),
        }
    }

    pub fn run(&mut self) -> Result<Value, Error> {
        let Function {
            ref instructions,
            registers_count,
            ..
        } = FUNCTIONS.get().unwrap()[0];

        let registers = Registers(self.registers.as_mut_ptr());

        self.stack.push(Frame {
            return_index: 0,
            return_instructions: &[],
            return_registers: Registers(std::ptr::null_mut()),
            return_dest: 0,
            size: registers_count,
        });

        let index = 0;
        let discriminant = instructions[index].discriminant();
        unsafe { HANDLERS[discriminant](index, instructions, registers, self).map_err(|e| *e)? };

        let result = self.registers[0];
        Ok(result)
    }
}

const HANDLERS: [Handler; 55] = [
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

#[derive(Clone, Copy)]
struct Registers(pub *mut Value);

impl Registers {
    unsafe fn set_value(&mut self, dest: u8, value: Value) {
        unsafe {
            *self.0.add(dest as usize) = value;
        }
    }

    unsafe fn get_value(&self, src: u8) -> Value {
        unsafe { *self.0.add(src as usize) }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_add_rr(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::Add { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = registers.get_value(src2);
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot add, both operands must be numbers",
        );
        registers.set_value(dest, Value::number(src1.as_number() + src2.as_number()));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_add_ri(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::AddI { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = Value::number(src2.decode());
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot add, both operands must be numbers",
        );
        registers.set_value(dest, Value::number(src1.as_number() + src2.as_number()));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_subtract_rr(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::Subtract { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = registers.get_value(src2);
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot subtract, both operands must be numbers",
        );
        registers.set_value(dest, Value::number(src1.as_number() - src2.as_number()));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_subtract_ri(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::SubtractRI { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = Value::number(src2.decode());
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot subtract, both operands must be numbers",
        );
        registers.set_value(dest, Value::number(src1.as_number() - src2.as_number()));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_subtract_ir(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::SubtractIR { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = Value::number(src1.decode());
        let src2 = registers.get_value(src2);
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot subtract, both operands must be numbers",
        );
        registers.set_value(dest, Value::number(src1.as_number() - src2.as_number()));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_multiply_rr(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::Multiply { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = registers.get_value(src2);
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot multiply, both operands must be numbers",
        );
        registers.set_value(dest, Value::number(src1.as_number() * src2.as_number()));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_multiply_ri(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::MultiplyI { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = Value::number(src2.decode());
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot multiply, both operands must be numbers",
        );
        registers.set_value(dest, Value::number(src1.as_number() * src2.as_number()));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_divide_rr(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::Divide { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = registers.get_value(src2);
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot divide, both operands must be numbers",
        );
        registers.set_value(dest, Value::number(src1.as_number() / src2.as_number()));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_divide_ri(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::DivideRI { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = Value::number(src2.decode());
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot divide, both operands must be numbers",
        );
        registers.set_value(dest, Value::number(src1.as_number() / src2.as_number()));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_divide_ir(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::DivideIR { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = Value::number(src1.decode());
        let src2 = registers.get_value(src2);
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot divide, both operands must be numbers",
        );
        registers.set_value(dest, Value::number(src1.as_number() / src2.as_number()));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_modulo_rr(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::Modulo { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = registers.get_value(src2);
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compute modulo, both operands must be numbers",
        );
        registers.set_value(dest, Value::number(src1.as_number() % src2.as_number()));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_modulo_ri(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::ModuloRI { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = Value::number(src2.decode());
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compute modulo, both operands must be numbers",
        );
        registers.set_value(dest, Value::number(src1.as_number() % src2.as_number()));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_modulo_ir(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::ModuloIR { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = Value::number(src1.decode());
        let src2 = registers.get_value(src2);
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compute modulo, both operands must be numbers",
        );
        registers.set_value(dest, Value::number(src1.as_number() % src2.as_number()));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_equal_rr(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::Equal { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = registers.get_value(src2);
        registers.set_value(dest, Value::number((src1 == src2) as u8 as f64));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_equal_ri(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::EqualI { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = Value::number(src2.decode());
        registers.set_value(dest, Value::number((src1 == src2) as u8 as f64));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_not_equal_rr(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::NotEqual { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = registers.get_value(src2);
        registers.set_value(dest, Value::number((src1 != src2) as u8 as f64));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_not_equal_ri(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::NotEqualI { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = Value::number(src2.decode());
        registers.set_value(dest, Value::number((src1 != src2) as u8 as f64));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_rr(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::Less { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = registers.get_value(src2);
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );
        registers.set_value(
            dest,
            Value::number((src1.as_number() < src2.as_number()) as u8 as f64),
        );
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_ri(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::LessI { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = Value::number(src2.decode());
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );
        registers.set_value(
            dest,
            Value::number((src1.as_number() < src2.as_number()) as u8 as f64),
        );
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_equal_rr(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::LessEqual { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = registers.get_value(src2);
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );
        registers.set_value(
            dest,
            Value::number((src1.as_number() <= src2.as_number()) as u8 as f64),
        );
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_less_equal_ri(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::LessEqualI { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = Value::number(src2.decode());
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );
        registers.set_value(
            dest,
            Value::number((src1.as_number() <= src2.as_number()) as u8 as f64),
        );
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_greater_rr(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::Greater { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = registers.get_value(src2);
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );
        registers.set_value(
            dest,
            Value::number((src1.as_number() > src2.as_number()) as u8 as f64),
        );
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_greater_ri(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::GreaterI { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = Value::number(src2.decode());
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );
        registers.set_value(
            dest,
            Value::number((src1.as_number() > src2.as_number()) as u8 as f64),
        );
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_greater_equal_rr(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::GreaterEqual { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = registers.get_value(src2);
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );
        registers.set_value(
            dest,
            Value::number((src1.as_number() >= src2.as_number()) as u8 as f64),
        );
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_greater_equal_ri(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::GreaterEqualI { dest, src1, src2 } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = Value::number(src2.decode());
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );
        registers.set_value(
            dest,
            Value::number((src1.as_number() >= src2.as_number()) as u8 as f64),
        );
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_not(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::Not { dest, src } = instructions[index] else {
            unreachable_unchecked()
        };
        let src = registers.get_value(src);
        type_check!(
            src.is_number(),
            "cannot apply not, operand must be a boolean",
        );
        registers.set_value(dest, Value::number((src.as_number() == 0.0) as u8 as f64));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_negate(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::Negate { dest, src } = instructions[index] else {
            unreachable_unchecked()
        };
        let src = registers.get_value(src);
        type_check!(src.is_number(), "cannot negate, operand must be a number",);
        registers.set_value(dest, Value::number(-src.as_number()));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_move(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::Move { dest, src } = instructions[index] else {
            unreachable_unchecked()
        };
        let src = registers.get_value(src);
        registers.set_value(dest, src);
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_move_arg(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::MoveArg { dest, src } = instructions[index] else {
            unreachable_unchecked()
        };
        let src = registers.get_value(src);
        registers.set_value(dest, src);
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_load_k(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::LoadK { dest, src } = instructions[index] else {
            unreachable_unchecked()
        };
        let constant = CONSTANT_POOL.get().unwrap_unchecked()[src as usize];
        registers.set_value(dest, constant);
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_load_imm(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::LoadImm { dest, src } = instructions[index] else {
            unreachable_unchecked()
        };
        registers.set_value(dest, Value::number(src.decode()));
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_create_dict(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::CreateDict { dest } = instructions[index] else {
            unreachable_unchecked()
        };
        let value = vm.gc.allocate_dict();
        registers.set_value(dest, value);
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_set_field_r(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::SetField { object, key, value } = instructions[index] else {
            unreachable_unchecked()
        };
        let object = registers.get_value(object);
        let key = registers.get_value(key);
        let value = registers.get_value(value);
        type_check!(object.is_dict(), "cannot set field, value is not a dict",);
        vm.gc.get_mut_dict(object).insert(key, value);
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_set_field_i(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::SetFieldI { object, key, src } = instructions[index] else {
            unreachable_unchecked()
        };
        let object = registers.get_value(object);
        let key = registers.get_value(key);
        let value = Value::number(src.decode());
        type_check!(object.is_dict(), "cannot set field, value is not a dict",);
        vm.gc.get_mut_dict(object).insert(key, value);
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_get_field(
    index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::GetField { dest, object, key } = instructions[index] else {
            unreachable_unchecked()
        };
        let object = registers.get_value(object);
        let key = registers.get_value(key);
        type_check!(object.is_dict(), "cannot get field, value is not a dict",);
        let value = vm
            .gc
            .get_dict(object)
            .get(&key)
            .copied()
            .unwrap_or_default();
        registers.set_value(dest, value);
        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_create_closure(
    mut index: usize,
    instructions: &[Instruction],
    mut registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::CreateClosure {
            dest,
            captures: captures_count,
            src,
        } = instructions[index]
        else {
            unreachable_unchecked()
        };

        let Function {
            ref instructions,
            registers_count,
            parameters,
        } = FUNCTIONS.get().unwrap_unchecked()[src as usize];

        let closure = Closure {
            instructions,
            parameters,
            size: registers_count,
            captured: vec![Value::default(); captures_count as usize].into_boxed_slice(),
        };

        let closure = vm.gc.allocate_closure(closure);
        registers.set_value(dest, closure);

        let mut captured_values = Vec::with_capacity(captures_count as usize);

        for _ in 0..captures_count {
            index += 1;
            let Instruction::CaptureValue { src } = instructions[index] else {
                unreachable_unchecked()
            };
            let capture = registers.get_value(src);
            captured_values.push(capture);
        }

        vm.gc.get_mut_closure(closure).captured = captured_values.into_boxed_slice();

        dispatch_next!(index, instructions, registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_call(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::Call { dest, src } = instructions[index] else {
            unreachable_unchecked()
        };

        let src = registers.get_value(src);
        type_check!(src.is_function(), "cannot call, value is not a function",);

        let Closure {
            instructions: fn_instructions,
            parameters,
            size,
            ref captured,
        } = *vm.gc.get_closure(src);

        let current_frame_size = vm.stack.last().unwrap_unchecked().size;
        let mut callee_registers = Registers(registers.0.add(current_frame_size as usize));

        for (i, value) in captured.iter().copied().enumerate() {
            callee_registers.set_value(parameters + i as u8, value);
        }

        let fn_instructions: &[Instruction] =
            std::slice::from_raw_parts(fn_instructions, /* len */ usize::MAX);

        let frame = Frame {
            return_index: index + 1,
            return_instructions: instructions,
            return_registers: registers,
            return_dest: dest,
            size,
        };

        vm.stack.push(frame);

        let callee_index = 0;
        let discriminant = fn_instructions[callee_index].discriminant();
        become HANDLERS[discriminant](callee_index, fn_instructions, callee_registers, vm)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_return(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::Return { src } = instructions[index] else {
            unreachable_unchecked()
        };

        let return_value = registers.get_value(src);

        let Frame {
            return_index,
            return_instructions,
            return_registers: mut registers,
            return_dest: dest,
            ..
        } = vm.stack.pop().unwrap_unchecked();

        if return_instructions.is_empty() {
            *vm.registers.as_mut_ptr() = return_value;
            Ok(())
        } else {
            registers.set_value(dest, return_value);
            let discriminant = return_instructions[return_index].discriminant();
            become HANDLERS[discriminant](return_index, return_instructions, registers, vm)
        }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::Jump { offset } = instructions[index] else {
            unreachable_unchecked()
        };
        dispatch_offset!(index, instructions, registers, vm, offset)
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_false(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::JumpIfFalse { src, offset } = instructions[index] else {
            unreachable_unchecked()
        };
        let src = registers.get_value(src);
        type_check!(
            src.is_number(),
            "cannot use this as a condition, value must be a boolean",
        );
        if src.is_truthy() {
            dispatch_next!(index, instructions, registers, vm)
        } else {
            dispatch_offset!(index, instructions, registers, vm, offset)
        }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_true(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::JumpIfTrue { src, offset } = instructions[index] else {
            unreachable_unchecked()
        };
        let src = registers.get_value(src);
        type_check!(
            src.is_number(),
            "cannot use this as a condition, value must be a boolean",
        );
        if src.is_truthy() {
            dispatch_offset!(index, instructions, registers, vm, offset)
        } else {
            dispatch_next!(index, instructions, registers, vm)
        }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_rr(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::JumpIfLess { src1, src2, offset } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = registers.get_value(src2);
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );
        if src1.as_number() < src2.as_number() {
            dispatch_offset!(index, instructions, registers, vm, offset)
        } else {
            dispatch_next!(index, instructions, registers, vm)
        }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_ri(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::JumpIfLessI { src1, src2, offset } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = Value::number(src2.decode());
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );
        if src1.as_number() < src2.as_number() {
            dispatch_offset!(index, instructions, registers, vm, offset)
        } else {
            dispatch_next!(index, instructions, registers, vm)
        }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_equal_rr(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::JumpIfLessEqual { src1, src2, offset } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = registers.get_value(src2);
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );
        if src1.as_number() <= src2.as_number() {
            dispatch_offset!(index, instructions, registers, vm, offset)
        } else {
            dispatch_next!(index, instructions, registers, vm)
        }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_less_equal_ri(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::JumpIfLessEqualI { src1, src2, offset } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = Value::number(src2.decode());
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );
        if src1.as_number() <= src2.as_number() {
            dispatch_offset!(index, instructions, registers, vm, offset)
        } else {
            dispatch_next!(index, instructions, registers, vm)
        }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_greater_rr(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::JumpIfGreater { src1, src2, offset } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = registers.get_value(src2);
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );
        if src1.as_number() > src2.as_number() {
            dispatch_offset!(index, instructions, registers, vm, offset)
        } else {
            dispatch_next!(index, instructions, registers, vm)
        }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_greater_ri(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::JumpIfGreaterI { src1, src2, offset } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = Value::number(src2.decode());
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );
        if src1.as_number() > src2.as_number() {
            dispatch_offset!(index, instructions, registers, vm, offset)
        } else {
            dispatch_next!(index, instructions, registers, vm)
        }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_greater_equal_rr(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::JumpIfGreaterEqual { src1, src2, offset } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = registers.get_value(src2);
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );
        if src1.as_number() >= src2.as_number() {
            dispatch_offset!(index, instructions, registers, vm, offset)
        } else {
            dispatch_next!(index, instructions, registers, vm)
        }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_greater_equal_ri(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::JumpIfGreaterEqualI { src1, src2, offset } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = Value::number(src2.decode());
        type_check!(
            src1.is_number() && src2.is_number(),
            "cannot compare, both operands must be numbers",
        );
        if src1.as_number() >= src2.as_number() {
            dispatch_offset!(index, instructions, registers, vm, offset)
        } else {
            dispatch_next!(index, instructions, registers, vm)
        }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_equal_rr(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::JumpIfEqual { src1, src2, offset } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = registers.get_value(src2);
        if src1 == src2 {
            dispatch_offset!(index, instructions, registers, vm, offset)
        } else {
            dispatch_next!(index, instructions, registers, vm)
        }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_equal_ri(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::JumpIfEqualI { src1, src2, offset } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = Value::number(src2.decode());
        if src1 == src2 {
            dispatch_offset!(index, instructions, registers, vm, offset)
        } else {
            dispatch_next!(index, instructions, registers, vm)
        }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_not_equal_rr(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::JumpIfNotEqual { src1, src2, offset } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = registers.get_value(src2);
        if src1 != src2 {
            dispatch_offset!(index, instructions, registers, vm, offset)
        } else {
            dispatch_next!(index, instructions, registers, vm)
        }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_jump_if_not_equal_ri(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe {
        let Instruction::JumpIfNotEqualI { src1, src2, offset } = instructions[index] else {
            unreachable_unchecked()
        };
        let src1 = registers.get_value(src1);
        let src2 = Value::number(src2.decode());
        if src1 != src2 {
            dispatch_offset!(index, instructions, registers, vm, offset)
        } else {
            dispatch_next!(index, instructions, registers, vm)
        }
    }
}

#[inline(never)]
unsafe extern "rust-preserve-none" fn opcode_nop(
    index: usize,
    instructions: &[Instruction],
    registers: Registers,
    vm: &mut Vm,
) -> Result<(), Box<Error>> {
    unsafe { dispatch_next!(index, instructions, registers, vm) }
}

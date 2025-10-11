use std::hint::unreachable_unchecked;

use crate::bytecode::{instruction::Instruction, value::Value};

use super::call_stack::CallStack;

type InstructionHandler = fn(&mut VMContext, ip: *const Instruction);
pub struct VMContext {
    pub call_stack: CallStack,
    pub constants: Vec<Value>,
    pub registers: Vec<Value>,
    pub instruction_dispatch: [InstructionHandler; 20],
}

impl VMContext {
    pub fn new(return_address: *const Instruction, constants: Vec<Value>) -> Self {
        Self {
            call_stack: CallStack::new(return_address),
            constants,
            registers: vec![Value::default(); 1024],
            instruction_dispatch: [
                instruction_add,              // 0
                instruction_subtract,         // 1
                instruction_multiply,         // 2
                instruction_divide,           // 3
                instruction_modulo,           // 4
                instruction_equal,            // 5
                instruction_not_equal,        // 6
                instruction_greater,          // 7
                instruction_greater_equal,    // 8
                instruction_less,             // 9
                instruction_less_equal,       // 10
                instruction_negate,           // 11
                instruction_not,              // 12
                instruction_move,             // 13
                instruction_call,             // 14
                instruction_return,           // 15
                instruction_jump,             // 16
                instruction_conditional_jump, // 17
                instruction_print,            // 18
                instruction_halt,             // 19
            ],
        }
    }

    #[inline(always)]
    fn get_value(&self, register: i16) -> &Value {
        if register < 0 {
            &self.constants[-register as usize]
        } else {
            let base_address = self.call_stack.function_frames.last().unwrap().base_address;

            &self.registers[base_address + register as usize]
        }
    }

    #[inline(always)]
    fn set_value(&mut self, register: i16, value: Value) {
        let base_address = self.call_stack.function_frames.last().unwrap().base_address;

        self.registers[base_address + register as usize] = value;
    }
}

pub fn run_vm(instructions: Vec<Instruction>, constants: Vec<Value>) {
    let halt_ip = unsafe { instructions.as_ptr().add(instructions.len() - 1) };

    let mut ctx = VMContext::new(halt_ip, constants);

    let ip = instructions.as_ptr();
    let op_code = instructions[0].discriminant();

    ctx.instruction_dispatch[op_code](&mut ctx, ip);
}

#[inline(never)]
fn instruction_move(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Move { dest, src } = *ip else {
            unreachable_unchecked();
        };

        let value = ctx.get_value(src);

        ctx.set_value(dest, *value);

        let ip = ip.add(1);
        let op_code = (*ip).discriminant();

        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}

#[inline(never)]
fn instruction_add(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Add { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };

        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::number(lhs + rhs));

        let ip = ip.add(1);
        let op_code = (*ip).discriminant();

        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}

#[inline(never)]
fn instruction_subtract(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Subtract { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::number(lhs - rhs));

        let ip = ip.add(1);
        let op_code = (*ip).discriminant();

        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}

#[inline(never)]
fn instruction_multiply(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Multiply { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::number(lhs * rhs));

        let ip = ip.add(1);
        let op_code = (*ip).discriminant();

        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}

#[inline(never)]
fn instruction_divide(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Divide { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::number(lhs / rhs));

        let ip = ip.add(1);
        let op_code = (*ip).discriminant();

        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}

#[inline(never)]
fn instruction_modulo(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Modulo { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::number(lhs % rhs));

        let ip = ip.add(1);
        let op_code = (*ip).discriminant();

        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}

#[inline(never)]
fn instruction_equal(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Equal { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::boolean(lhs == rhs));

        let ip = ip.add(1);
        let op_code = (*ip).discriminant();

        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}

#[inline(never)]
fn instruction_not_equal(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::NotEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::boolean(lhs != rhs));

        let ip = ip.add(1);
        let op_code = (*ip).discriminant();

        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}

#[inline(never)]
fn instruction_greater(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Greater { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::boolean(lhs > rhs));

        let ip = ip.add(1);
        let op_code = (*ip).discriminant();

        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}

#[inline(never)]
fn instruction_greater_equal(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::GreaterEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::boolean(lhs >= rhs));

        let ip = ip.add(1);
        let op_code = (*ip).discriminant();

        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}

#[inline(never)]
fn instruction_less(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Less { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::boolean(lhs < rhs));

        let ip = ip.add(1);
        let op_code = (*ip).discriminant();

        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}

#[inline(never)]
fn instruction_less_equal(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::LessEqual { dest, src1, src2 } = *ip else {
            unreachable_unchecked();
        };
        let lhs = ctx.get_value(src1).as_number();
        let rhs = ctx.get_value(src2).as_number();
        ctx.set_value(dest, Value::boolean(lhs <= rhs));

        let ip = ip.add(1);
        let op_code = (*ip).discriminant();

        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}

#[inline(never)]
fn instruction_negate(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Negate { dest, src } = *ip else {
            unreachable_unchecked();
        };
        let value = ctx.get_value(src).as_number();
        ctx.set_value(dest, Value::number(-value));

        let ip = ip.add(1);
        let op_code = (*ip).discriminant();

        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}

#[inline(never)]
fn instruction_not(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Not { dest, src } = *ip else {
            unreachable_unchecked();
        };
        let value = ctx.get_value(src).as_boolean();
        ctx.set_value(dest, Value::boolean(!value));

        let ip = ip.add(1);
        let op_code = (*ip).discriminant();

        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}
#[inline(never)]
fn instruction_jump(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Jump { offset } = *ip else {
            unreachable_unchecked();
        };

        let ip = ip.offset(offset as isize);
        let op_code = (*ip).discriminant();
        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}

#[inline(never)]
fn instruction_conditional_jump(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::ConditionalJump {
            src,
            true_offset,
            false_offset,
        } = *ip
        else {
            unreachable_unchecked();
        };

        let value = ctx.get_value(src);
        let ip = if value.as_boolean() {
            ip.offset(true_offset as isize)
        } else {
            ip.offset(false_offset as isize)
        };

        let op_code = (*ip).discriminant();
        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}

#[inline(never)]
fn instruction_call(ctx: &mut VMContext, ip: *const Instruction) {}

#[inline(never)]
fn instruction_return(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Return { src } = *ip else {
            unreachable_unchecked();
        };

        let value = *ctx.get_value(src);

        let frame = ctx.call_stack.pop_frame();

        let dest = frame.return_register;

        ctx.set_value(dest, value);

        let ip = frame.return_address;
        let op_code = (*ip).discriminant();

        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}

#[inline(never)]
fn instruction_print(ctx: &mut VMContext, ip: *const Instruction) {
    unsafe {
        let Instruction::Print { src } = *ip else {
            unreachable_unchecked();
        };

        let value = ctx.get_value(src);
        println!("{}", value.as_number()); // adapt if you have multiple value types

        let ip = ip.add(1);
        let op_code = (*ip).discriminant();

        become ctx.instruction_dispatch[op_code](ctx, ip);
    }
}

#[inline(never)]
fn instruction_halt(_ctx: &mut VMContext, _ip: *const Instruction) {
    println!("Program finished!");
}

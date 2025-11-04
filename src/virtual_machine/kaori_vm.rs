use crate::bytecode::{function::Function, value::Value};

pub struct FunctionFrame {
    pub size: u8,
    pub registers_ptr: *mut Value,
    pub constants_ptr: *const Value,
    pub return_address: *const u16,
    pub return_register: u16,
}

impl FunctionFrame {
    pub fn new(
        size: u8,
        registers_ptr: *mut Value,
        constants_ptr: *const Value,
        return_address: *const u16,
        return_register: u16,
    ) -> Self {
        Self {
            size,
            registers_ptr,
            constants_ptr,
            return_address,
            return_register,
        }
    }
}

type InstructionHandler = fn(&mut VMContext, bp: *const u16);

const OPCODE_HANDLERS: [InstructionHandler; 63] = [
    opcode_add_rr,           // 0
    opcode_add_rk,           // 1
    opcode_add_kr,           // 2
    opcode_add_kk,           // 3
    opcode_subtract_rr,      // 4
    opcode_subtract_rk,      // 5
    opcode_subtract_kr,      // 6
    opcode_subtract_kk,      // 7
    opcode_multiply_rr,      // 8
    opcode_multiply_rk,      // 9
    opcode_multiply_kr,      // 10
    opcode_multiply_kk,      // 11
    opcode_divide_rr,        // 12
    opcode_divide_rk,        // 13
    opcode_divide_kr,        // 14
    opcode_divide_kk,        // 15
    opcode_modulo_rr,        // 16
    opcode_modulo_rk,        // 17
    opcode_modulo_kr,        // 18
    opcode_modulo_kk,        // 19
    opcode_equal_rr,         // 20
    opcode_equal_rk,         // 21
    opcode_equal_kr,         // 22
    opcode_equal_kk,         // 23
    opcode_not_equal_rr,     // 24
    opcode_not_equal_rk,     // 25
    opcode_not_equal_kr,     // 26
    opcode_not_equal_kk,     // 27
    opcode_greater_rr,       // 28
    opcode_greater_rk,       // 29
    opcode_greater_kr,       // 30
    opcode_greater_kk,       // 31
    opcode_greater_equal_rr, // 32
    opcode_greater_equal_rk, // 33
    opcode_greater_equal_kr, // 34
    opcode_greater_equal_kk, // 35
    opcode_less_rr,          // 36
    opcode_less_rk,          // 37
    opcode_less_kr,          // 38
    opcode_less_kk,          // 39
    opcode_less_equal_rr,    // 40
    opcode_less_equal_rk,    // 41
    opcode_less_equal_kr,    // 42
    opcode_less_equal_kk,    // 43
    opcode_negate_r,         // 44
    opcode_negate_k,         // 45
    opcode_not_r,            // 46
    opcode_not_k,            // 47
    opcode_move_r,           // 48
    opcode_move_k,           // 49
    opcode_call_r,           // 50
    opcode_call_k,           // 51
    opcode_return_r,         // 52
    opcode_return_k,         // 53
    opcode_return_void,      // 54
    opcode_jump,             // 55
    opcode_jump_if_true_r,   // 56
    opcode_jump_if_true_k,   // 57
    opcode_jump_if_false_r,  // 58
    opcode_jump_if_false_k,  // 59
    opcode_print_r,          // 60
    opcode_print_k,          // 61
    opcode_halt,             // 62
];

pub struct VMContext<'a> {
    pub functions: &'a [Function],
    pub call_stack: Vec<FunctionFrame>,
    pub registers: Vec<Value>,
    pub frame_size: u8,
    pub registers_ptr: *mut Value,
    pub constants_ptr: *const Value,
}

macro_rules! dispatch {
    ($ctx:expr, $bp: expr) => {
        let _: &mut VMContext = $ctx;
        let _: *const u16 = $bp;
        let op_code: u16 = *$bp;

        become OPCODE_HANDLERS[op_code as usize]($ctx, $bp)
    };
}

macro_rules! dispatch_to {
    ($ctx:expr, $bp:expr, $offset: expr) => {
        let _: &mut VMContext = $ctx;
        let _: *const u16 = $bp;
        let _: i16 = $offset;

        let bp = $bp.offset($offset as isize);
        dispatch!($ctx, bp);
    };
}

impl<'a> VMContext<'a> {
    pub fn new(
        functions: &'a [Function],
        registers: Vec<Value>,
        frame_size: u8,
        registers_ptr: *mut Value,
        constants_ptr: *const Value,
        main_frame: FunctionFrame,
    ) -> Self {
        Self {
            functions,
            call_stack: vec![main_frame],
            registers,
            frame_size,
            registers_ptr,
            constants_ptr,
        }
    }

    #[inline(always)]
    fn get_constant_value(&self, index: u16) -> Value {
        unsafe { *self.constants_ptr.add(index as usize) }
    }

    #[inline(always)]
    fn get_register_value(&self, index: u16) -> Value {
        unsafe { *self.registers_ptr.add(index as usize) }
    }

    #[inline(always)]
    fn set_value(&mut self, index: u16, value: Value) {
        unsafe {
            *self.registers_ptr.add(index as usize) = value;
        }
    }

    #[inline(always)]
    fn pop_frame(&mut self) -> FunctionFrame {
        let frame = unsafe { self.call_stack.pop().unwrap_unchecked() };

        if let Some(frame) = self.call_stack.last() {
            self.registers_ptr = frame.registers_ptr;
            self.constants_ptr = frame.constants_ptr;
        }

        frame
    }

    #[inline(always)]
    fn push_frame(
        &mut self,
        return_register: u16,
        return_address: *const u16,
        frame_size: u8,
        constants_ptr: *const Value,
    ) {
        let size = self.call_stack.last().unwrap().size;

        let registers_ptr = unsafe { self.registers_ptr.add(size as usize) };

        let frame = FunctionFrame::new(
            frame_size,
            registers_ptr,
            constants_ptr,
            return_address,
            return_register,
        );

        self.registers_ptr = registers_ptr;
        self.constants_ptr = constants_ptr;

        self.call_stack.push(frame);
    }
}

pub fn run_kaori_vm(bytes: Vec<u16>, functions: Vec<Function>) {
    let mut registers = vec![Value::default(); 1024];
    let Function {
        bp,
        frame_size,
        ref constants,
    } = functions[0];

    let registers_ptr = registers.as_mut_ptr();
    let constants_ptr = (*constants).as_ptr();

    let return_address = unsafe { bytes.as_ptr().add(bytes.len() - 1) };
    let main_frame =
        FunctionFrame::new(frame_size, registers_ptr, constants_ptr, return_address, 0);

    let mut ctx = VMContext::new(
        &functions,
        registers,
        frame_size,
        registers_ptr,
        constants_ptr,
        main_frame,
    );

    let op_code = unsafe { *bp };
    OPCODE_HANDLERS[op_code as usize](&mut ctx, bp)
}

#[inline(never)]
fn opcode_move_r(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src = *bp.add(2);

        let value = ctx.get_register_value(src);
        ctx.set_value(dest, value);

        let bp = bp.add(3);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_move_k(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src = *bp.add(2);

        let value = ctx.get_constant_value(src);
        ctx.set_value(dest, value);

        let bp = bp.add(3);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_add_rr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs + rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_add_rk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs + rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_add_kr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs + rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_add_kk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs + rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_subtract_rr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs - rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_subtract_rk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs - rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_subtract_kr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs - rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_subtract_kk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs - rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_multiply_rr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs * rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_multiply_rk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs * rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_multiply_kr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs * rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_multiply_kk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs * rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_divide_rr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs / rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_divide_rk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs / rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_divide_kr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs / rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_divide_kk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs / rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_modulo_rr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs % rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_modulo_rk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs % rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_modulo_kr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs % rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_modulo_kk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::number(lhs % rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_equal_rr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs == rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_equal_rk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs == rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_equal_kr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs == rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_equal_kk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs == rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_not_equal_rr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs != rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_not_equal_rk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs != rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_not_equal_kr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs != rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_not_equal_kk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs != rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_greater_rr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs > rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_greater_rk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs > rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_greater_kr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs > rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_greater_kk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs > rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_greater_equal_rr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs >= rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_greater_equal_rk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs >= rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_greater_equal_kr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs >= rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_greater_equal_kk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs >= rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_less_rr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs < rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_less_rk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs < rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_less_kr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs < rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_less_kk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs < rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_less_equal_rr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs <= rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_less_equal_rk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_register_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs <= rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_less_equal_kr(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_register_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs <= rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_less_equal_kk(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src1 = *bp.add(2);
        let src2 = *bp.add(3);

        let lhs = ctx.get_constant_value(src1).as_number();
        let rhs = ctx.get_constant_value(src2).as_number();

        ctx.set_value(dest, Value::boolean(lhs <= rhs));

        let bp = bp.add(4);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_negate_r(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src = *bp.add(2);

        let value = ctx.get_register_value(src).as_number();
        ctx.set_value(dest, Value::number(-value));

        let bp = bp.add(3);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_negate_k(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src = *bp.add(2);

        let value = ctx.get_constant_value(src).as_number();
        ctx.set_value(dest, Value::number(-value));

        let bp = bp.add(3);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_not_r(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src = *bp.add(2);

        let value = ctx.get_register_value(src).as_boolean();
        ctx.set_value(dest, Value::boolean(!value));

        let bp = bp.add(3);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_not_k(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src = *bp.add(2);

        let value = ctx.get_constant_value(src).as_boolean();
        ctx.set_value(dest, Value::boolean(!value));

        let bp = bp.add(3);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_call_r(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src = *bp.add(2);

        let return_address = bp.add(3);

        let function_index = ctx.get_register_value(src).as_function();

        let Function {
            bp,
            frame_size,
            ref constants,
        } = ctx.functions[function_index];
        let constants_ptr = (*constants).as_ptr();

        ctx.push_frame(dest, return_address, frame_size, constants_ptr);

        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_call_k(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let dest = *bp.add(1);
        let src = *bp.add(2);

        let return_address = bp.add(3);

        let function_index = ctx.get_constant_value(src).as_function();

        let Function {
            bp,
            frame_size,
            ref constants,
        } = ctx.functions[function_index];
        let constants_ptr = (*constants).as_ptr();

        ctx.push_frame(dest, return_address, frame_size, constants_ptr);

        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_return_r(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let src = *bp.add(1);
        let value = ctx.get_register_value(src);

        let FunctionFrame {
            return_address: bp,
            return_register: dest,
            ..
        } = ctx.pop_frame();

        ctx.set_value(dest, value);

        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_return_k(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let src = *bp.add(1);
        let value = ctx.get_constant_value(src);

        let FunctionFrame {
            return_address: bp,
            return_register: dest,
            ..
        } = ctx.pop_frame();

        ctx.set_value(dest, value);

        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_return_void(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let frame = ctx.pop_frame();
        let bp = frame.return_address;

        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_jump(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let offset = *bp.add(1) as i16;

        dispatch_to!(ctx, bp, offset);
    }
}

#[inline(never)]
fn opcode_jump_if_true_r(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let src = *bp.add(1);
        let offset = *bp.add(2) as i16;

        match ctx.get_register_value(src).as_boolean() {
            true => {
                dispatch_to!(ctx, bp, offset);
            }
            false => {
                let bp = bp.add(3);
                dispatch!(ctx, bp);
            }
        }
    }
}

#[inline(never)]
fn opcode_jump_if_true_k(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let src = *bp.add(1);
        let offset = *bp.add(2) as i16;

        match ctx.get_constant_value(src).as_boolean() {
            true => {
                dispatch_to!(ctx, bp, offset);
            }
            false => {
                let bp = bp.add(3);
                dispatch!(ctx, bp);
            }
        }
    }
}

#[inline(never)]
fn opcode_jump_if_false_r(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let src = *bp.add(1);
        let offset = *bp.add(2) as i16;

        match ctx.get_register_value(src).as_boolean() {
            true => {
                let bp = bp.add(3);
                dispatch!(ctx, bp);
            }
            false => {
                dispatch_to!(ctx, bp, offset);
            }
        }
    }
}

#[inline(never)]
fn opcode_jump_if_false_k(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let src = *bp.add(1);
        let offset = *bp.add(2) as i16;

        match ctx.get_constant_value(src).as_boolean() {
            true => {
                let bp = bp.add(3);
                dispatch!(ctx, bp);
            }
            false => {
                dispatch_to!(ctx, bp, offset);
            }
        }
    }
}

#[inline(never)]
fn opcode_print_r(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let src = *bp.add(1);
        let value = ctx.get_register_value(src).as_number();

        println!("{}", value);

        let bp = bp.add(2);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_print_k(ctx: &mut VMContext, bp: *const u16) {
    unsafe {
        let src = *bp.add(1);
        let value = ctx.get_constant_value(src).as_number();

        println!("{}", value);

        let bp = bp.add(2);
        dispatch!(ctx, bp);
    }
}

#[inline(never)]
fn opcode_halt(_ctx: &mut VMContext, _bp: *const u16) {}

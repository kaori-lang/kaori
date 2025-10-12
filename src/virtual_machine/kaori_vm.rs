use std::mem::MaybeUninit;
use crate::bytecode::{instruction::Instruction, value::Value};

struct CallFrame {
    return_address: *const Instruction,
    register_ptr: *mut Value,
    return_slot: *mut Value
}

type InstructionHandler = unsafe fn(
    call_stack: *mut CallFrame,
    constants_ptr: *const Value,
    register_ptr: *mut Value,
    ip: *const Instruction
);

struct Context {
    call_stack: *mut CallFrame,
    constants_ptr: *const Value,
    register_ptr: *mut Value,
}

#[inline(always)]
unsafe fn register_index(index: i16) -> usize {
    unsafe {
        core::hint::assert_unchecked(index >= 0);
        index.cast_unsigned() as usize
    }
}

macro_rules! push_frame {
    ($ctx:ident, ret: $ret_address: expr; current_size: $size: expr; slot: $slot: expr $(;)?) => {{
        let slot = raw_register!($ctx, $slot);
        let frame = CallFrame {
            return_address: $ret_address,
            register_ptr: $ctx.register_ptr,
            return_slot: slot,
        };
        *$ctx.call_stack = frame;
        $ctx.call_stack = $ctx.call_stack.add(1);
        $ctx.register_ptr = $ctx.register_ptr.add($size);
    }};
}

macro_rules! pop_frame {
    ($ctx:ident) => {{
        $ctx.call_stack = $ctx.call_stack.sub(1);
        std::ptr::read($ctx.call_stack)
    }};
}


macro_rules! raw_register {
    ($ctx:ident, $var: expr) => { $ctx.register_ptr.add(register_index($var)) };
}

macro_rules! get {
    (@raw $ctx:ident, $var: ident) => {
        *match $var {
            index @ 0.. => raw_register!($ctx, index),
            const_index @ ..0 => $ctx.constants_ptr.add(const_index.unsigned_abs() as usize)
        }
    };
    ($ctx: ident, $var: ident) => {
        (get!(@raw $ctx, $var).as_number())
    };
}

macro_rules! set {
    (@raw $ctx:ident, $var: ident = $value: expr) => {{
        *raw_register!($ctx, $var) = $value;
    }};
    ($ctx:ident, $var: ident = $value: expr) => {
        set!(@raw $ctx, $var = Value::number($value))
    };
}


macro_rules! call_to {
    ($(@become $({$become_binder: tt})?)? $ctx:ident, $new_ip:expr) => {{
        let ip = $new_ip;
        let opcode = (*ip).discriminant();
        $(become $({$become_binder})?)? INSTRUCTION_DISPATCH[opcode](
            $ctx.call_stack,
            $ctx.constants_ptr,
            $ctx.register_ptr,
            ip
        );
    }};
}

macro_rules! dispatch_to {
    ($ctx:ident, $new_ip:expr) => { call_to!(@become $ctx, $new_ip) };
}

macro_rules! dispatch_next {
    ($ctx:ident, $ip: expr) => { dispatch_to!($ctx, $ip.add(1)) };
}

macro_rules! declare_instructions {
    (
        ctx: $ctx: ident,
        ip: $ip: ident,
        $($instruction: ident $({ $($field: ident),* })? => $body: block)*
    ) => {
        const fn build_dispatch_table() -> [InstructionHandler; Instruction::COUNT] {
            // Start with None for all entries
            let mut table: [Option<InstructionHandler>; Instruction::COUNT] = [None; Instruction::COUNT];

            // Map each handler to its discriminant
            $({
                let index = (Instruction::$instruction $({ $($field: 0),* })?).discriminant();
                unsafe fn instruction(
                    call_stack: *mut CallFrame,
                    constants_ptr: *const Value,
                    register_ptr: *mut Value,
                    $ip: *const Instruction
                ) {
                    #[allow(unused_mut)]
                    let mut $ctx = Context {
                        call_stack,
                        constants_ptr,
                        register_ptr,
                    };

                    unsafe {
                        let Instruction::$instruction $({ $($field),* })? = *$ip else {
                            core::hint::unreachable_unchecked()
                        };

                        $body
                    }
                }
                table[index] = Some(instruction);
            })*

            // Verify all entries are set and unwrap to final table
            let mut final_table: [InstructionHandler; Instruction::COUNT] = [const { |_, _, _, _| unreachable!() }; Instruction::COUNT];
            let mut i = 0;
            while i < Instruction::COUNT {
                final_table[i] = match table[i] {
                    Some(handler) => handler,
                    None => panic!("Missing handler for instruction"), // Compile-time panic if any missing
                };
                i += 1;
            }

            final_table
        }
    };
}

declare_instructions! {
    ctx: ctx,
    ip: ip,
    Add { dest, src1, src2 } => {
        set!(ctx, dest = get!(ctx, src1) + get!(ctx, src2));
        dispatch_next!(ctx, ip)
    }

    Subtract { dest, src1, src2 } => {
        set!(ctx, dest = get!(ctx, src1) - get!(ctx, src2));
        dispatch_next!(ctx, ip)
    }

    Multiply { dest, src1, src2 } => {
        set!(ctx, dest = get!(ctx, src1) * get!(ctx, src2));
        dispatch_next!(ctx, ip)
    }

    Divide { dest, src1, src2 } => {
        set!(ctx, dest = get!(ctx, src1) / get!(ctx, src2));
        dispatch_next!(ctx, ip)
    }

    Modulo { dest, src1, src2 } => {
        set!(ctx, dest = get!(ctx, src1) % get!(ctx, src2));
        dispatch_next!(ctx, ip)
    }

    Equal { dest, src1, src2 } => {
        set!(@raw ctx, dest = Value::boolean(get!(ctx, src1) == get!(ctx, src2)));
        dispatch_next!(ctx, ip)
    }

    NotEqual { dest, src1, src2 } => {
        set!(@raw ctx, dest = Value::boolean(get!(ctx, src1) != get!(ctx, src2)));
        dispatch_next!(ctx, ip)
    }

    Greater { dest, src1, src2 } => {
        set!(@raw ctx, dest = Value::boolean(get!(ctx, src1) > get!(ctx, src2)));
        dispatch_next!(ctx, ip)
    }

    GreaterEqual { dest, src1, src2 } => {
        set!(@raw ctx, dest = Value::boolean(get!(ctx, src1) >= get!(ctx, src2)));
        dispatch_next!(ctx, ip)
    }

    Less { dest, src1, src2 } => {
        set!(@raw ctx, dest = Value::boolean(get!(ctx, src1) < get!(ctx, src2)));
        dispatch_next!(ctx, ip)
    }

    LessEqual { dest, src1, src2 } => {
        set!(@raw ctx, dest = Value::boolean(get!(ctx, src1) <= get!(ctx, src2)));
        dispatch_next!(ctx, ip)
    }

    Negate { dest, src } => {
        set!(ctx, dest = -get!(ctx, src));
        dispatch_next!(ctx, ip)
    }

    Not { dest, src } => {
        set!(@raw ctx, dest = Value::boolean(!get!(@raw ctx, src).as_boolean()));
        dispatch_next!(ctx, ip)
    }

    Move { dest, src } => {
        set!(@raw ctx, dest = get!(@raw ctx, src));
        dispatch_next!(ctx, ip)
    }

    Call { dest, src, caller_size } => {
        push_frame!(
            ctx,
            ret: ip.add(1);
            current_size: usize::from(caller_size);
            slot: dest;
        );
        let new_ip = get!(@raw ctx, src).as_instruction();
        dispatch_to!(ctx, new_ip)
    }

    Return { src } => {
        let ret_val = get!(@raw ctx, src);
        let frame = pop_frame!(ctx);
        *frame.return_slot = ret_val;
        ctx.register_ptr = frame.register_ptr;
        dispatch_to!(ctx, frame.return_address)
    }

    Jump { offset } => {
        let new_ip = ip.offset(isize::from(offset));
        dispatch_to!(ctx, new_ip)
    }

    ConditionalJump { src, true_offset, false_offset } => {
        let offset = match get!(@raw ctx, src).as_boolean() {
            true => true_offset,
            false => false_offset,
        };
        let new_ip = ip.offset(isize::from(offset));
        dispatch_to!(ctx, new_ip)
    }

    Print { src } => {
        println!("{}", get!(ctx, src));
        dispatch_next!(ctx, ip)
    }

    Halt => {
        let _ = (ip, ctx);
        /*No dispatch, function returns */
    }
}

static INSTRUCTION_DISPATCH: [InstructionHandler; Instruction::COUNT] = build_dispatch_table();

pub unsafe fn run_vm(instructions: &[Instruction], constants: &[Value]) {
    static HALT: Instruction = Instruction::Halt;

    let mut call_stack = [const { MaybeUninit::<CallFrame>::uninit() }; 512];
    let mut registers = [const { MaybeUninit::<Value>::uninit() }; 1024];

    let mut ctx = Context {
        call_stack: call_stack.as_mut_ptr().cast::<CallFrame>(),
        constants_ptr: constants.as_ptr(),
        register_ptr: registers.as_mut_ptr().cast::<Value>(),
    };


    unsafe {
        // Push initial frame
        // it contains one slot for the return value of main
        push_frame!(ctx, ret: &raw const HALT; current_size: 1; slot: 0);
        call_to!(ctx, instructions.as_ptr())
    }
}

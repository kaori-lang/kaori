use crate::runtime::{
    instruction::Instruction,
    operands::{Const, Reg},
};

use std::hint::unreachable_unchecked;

#[inline(always)]
pub unsafe fn add(ip: *const Instruction) -> (Reg, Reg, Reg) {
    let Instruction::Add { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("add called on non-Add instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn add_k(ip: *const Instruction) -> (Reg, Reg, Const) {
    let Instruction::AddK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("add_k called on non-AddK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn subtract(ip: *const Instruction) -> (Reg, Reg, Reg) {
    let Instruction::Subtract { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("subtract called on non-Subtract instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn subtract_rk(ip: *const Instruction) -> (Reg, Reg, Const) {
    let Instruction::SubtractRK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("subtract_rk called on non-SubtractRK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn subtract_kr(ip: *const Instruction) -> (Reg, Const, Reg) {
    let Instruction::SubtractKR { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("subtract_kr called on non-SubtractKR instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn multiply(ip: *const Instruction) -> (Reg, Reg, Reg) {
    let Instruction::Multiply { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("multiply called on non-Multiply instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn multiply_k(ip: *const Instruction) -> (Reg, Reg, Const) {
    let Instruction::MultiplyK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("multiply_k called on non-MultiplyK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn divide(ip: *const Instruction) -> (Reg, Reg, Reg) {
    let Instruction::Divide { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("divide called on non-Divide instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn divide_rk(ip: *const Instruction) -> (Reg, Reg, Const) {
    let Instruction::DivideRK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("divide_rk called on non-DivideRK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn divide_kr(ip: *const Instruction) -> (Reg, Const, Reg) {
    let Instruction::DivideKR { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("divide_kr called on non-DivideKR instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn modulo(ip: *const Instruction) -> (Reg, Reg, Reg) {
    let Instruction::Modulo { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("modulo called on non-Modulo instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn modulo_rk(ip: *const Instruction) -> (Reg, Reg, Const) {
    let Instruction::ModuloRK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("modulo_rk called on non-ModuloRK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn modulo_kr(ip: *const Instruction) -> (Reg, Const, Reg) {
    let Instruction::ModuloKR { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("modulo_kr called on non-ModuloKR instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn equal(ip: *const Instruction) -> (Reg, Reg, Reg) {
    let Instruction::Equal { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("equal called on non-Equal instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn equal_k(ip: *const Instruction) -> (Reg, Reg, Const) {
    let Instruction::EqualK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("equal_k called on non-EqualK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn not_equal(ip: *const Instruction) -> (Reg, Reg, Reg) {
    let Instruction::NotEqual { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("not_equal called on non-NotEqual instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn not_equal_k(ip: *const Instruction) -> (Reg, Reg, Const) {
    let Instruction::NotEqualK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("not_equal_k called on non-NotEqualK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn less(ip: *const Instruction) -> (Reg, Reg, Reg) {
    let Instruction::Less { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("less called on non-Less instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn less_rk(ip: *const Instruction) -> (Reg, Reg, Const) {
    let Instruction::LessRK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("less_rk called on non-LessRK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn less_kr(ip: *const Instruction) -> (Reg, Const, Reg) {
    let Instruction::LessKR { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("less_kr called on non-LessKR instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn less_equal(ip: *const Instruction) -> (Reg, Reg, Reg) {
    let Instruction::LessEqual { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("less_equal called on non-LessEqual instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn less_equal_rk(ip: *const Instruction) -> (Reg, Reg, Const) {
    let Instruction::LessEqualRK { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("less_equal_rk called on non-LessEqualRK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn less_equal_kr(ip: *const Instruction) -> (Reg, Const, Reg) {
    let Instruction::LessEqualKR { dest, src1, src2 } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("less_equal_kr called on non-LessEqualKR instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src1, src2)
}

#[inline(always)]
pub unsafe fn not(ip: *const Instruction) -> (Reg, Reg) {
    let Instruction::Not { dest, src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("not called on non-Not instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src)
}

#[inline(always)]
pub unsafe fn negate(ip: *const Instruction) -> (Reg, Reg) {
    let Instruction::Negate { dest, src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("negate called on non-Negate instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src)
}

#[inline(always)]
pub unsafe fn move_(ip: *const Instruction) -> (Reg, Reg) {
    let Instruction::Move { dest, src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("move called on non-Move instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src)
}

#[inline(always)]
pub unsafe fn load_const(ip: *const Instruction) -> (Reg, Const) {
    let Instruction::LoadConst { dest, src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("load_const called on non-LoadConst instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src)
}

#[inline(always)]
pub unsafe fn create_map(ip: *const Instruction) -> Reg {
    let Instruction::CreateMap { dest } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("create_map called on non-CreateMap instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    dest
}

#[inline(always)]
pub unsafe fn set_field(ip: *const Instruction) -> (Reg, Reg, Reg) {
    let Instruction::SetField { object, key, value } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("set_field called on non-SetField instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (object, key, value)
}

#[inline(always)]
pub unsafe fn get_field(ip: *const Instruction) -> (Reg, Reg, Reg) {
    let Instruction::GetField { dest, object, key } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("get_field called on non-GetField instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, object, key)
}

#[inline(always)]
pub unsafe fn create_closure(ip: *const Instruction) -> (Reg, u32, u8) {
    let Instruction::CreateClosure {
        dest,
        src,
        captures,
    } = (unsafe { *ip })
    else {
        if cfg!(debug_assertions) {
            unreachable!("create_closure called on non-CreateClosure instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src, captures)
}

#[inline(always)]
pub unsafe fn create_ref(ip: *const Instruction) -> (Reg, Reg) {
    let Instruction::CreateRef { dest, src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("create_ref called on non-CreateRef instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src)
}

#[inline(always)]
pub unsafe fn deref_set(ip: *const Instruction) -> (Reg, Reg) {
    let Instruction::DerefSet { dest, src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("deref_set called on non-DerefSet instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src)
}

#[inline(always)]
pub unsafe fn deref(ip: *const Instruction) -> (Reg, Reg) {
    let Instruction::Deref { dest, src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("deref called on non-Deref instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src)
}

#[inline(always)]
pub unsafe fn call(ip: *const Instruction) -> (Reg, Reg) {
    let Instruction::Call { dest, src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("call called on non-Call instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (dest, src)
}

#[inline(always)]
pub unsafe fn return_(ip: *const Instruction) -> Reg {
    let Instruction::Return { src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("return called on non-Return instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    src
}

#[inline(always)]
pub unsafe fn jump(ip: *const Instruction) -> i32 {
    let Instruction::Jump { offset } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("jump called on non-Jump instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    offset
}

#[inline(always)]
pub unsafe fn jump_if_false(ip: *const Instruction) -> (Reg, i32) {
    let Instruction::JumpIfFalse { src, offset } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("jump_if_false called on non-JumpIfFalse instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (src, offset)
}

#[inline(always)]
pub unsafe fn jump_if_true(ip: *const Instruction) -> (Reg, i32) {
    let Instruction::JumpIfTrue { src, offset } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("jump_if_true called on non-JumpIfTrue instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (src, offset)
}

#[inline(always)]
pub unsafe fn jump_if_less(ip: *const Instruction) -> (Reg, Reg, i32) {
    let Instruction::JumpIfLess { src1, src2, offset } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("jump_if_less called on non-JumpIfLess instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (src1, src2, offset)
}

#[inline(always)]
pub unsafe fn jump_if_less_rk(ip: *const Instruction) -> (Reg, Const, i32) {
    let Instruction::JumpIfLessRK { src1, src2, offset } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("jump_if_less_rk called on non-JumpIfLessRK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (src1, src2, offset)
}

#[inline(always)]
pub unsafe fn jump_if_less_kr(ip: *const Instruction) -> (Const, Reg, i32) {
    let Instruction::JumpIfLessKR { src1, src2, offset } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("jump_if_less_kr called on non-JumpIfLessKR instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (src1, src2, offset)
}

#[inline(always)]
pub unsafe fn jump_if_less_equal(ip: *const Instruction) -> (Reg, Reg, i32) {
    let Instruction::JumpIfLessEqual { src1, src2, offset } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("jump_if_less_equal called on non-JumpIfLessEqual instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (src1, src2, offset)
}

#[inline(always)]
pub unsafe fn jump_if_less_equal_rk(ip: *const Instruction) -> (Reg, Const, i32) {
    let Instruction::JumpIfLessEqualRK { src1, src2, offset } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("jump_if_less_equal_rk called on non-JumpIfLessEqualRK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (src1, src2, offset)
}

#[inline(always)]
pub unsafe fn jump_if_less_equal_kr(ip: *const Instruction) -> (Const, Reg, i32) {
    let Instruction::JumpIfLessEqualKR { src1, src2, offset } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("jump_if_less_equal_kr called on non-JumpIfLessEqualKR instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (src1, src2, offset)
}

#[inline(always)]
pub unsafe fn jump_if_equal(ip: *const Instruction) -> (Reg, Reg, i32) {
    let Instruction::JumpIfEqual { src1, src2, offset } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("jump_if_equal called on non-JumpIfEqual instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (src1, src2, offset)
}

#[inline(always)]
pub unsafe fn jump_if_equal_k(ip: *const Instruction) -> (Reg, Const, i32) {
    let Instruction::JumpIfEqualK { src1, src2, offset } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("jump_if_equal_k called on non-JumpIfEqualK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (src1, src2, offset)
}

#[inline(always)]
pub unsafe fn jump_if_not_equal(ip: *const Instruction) -> (Reg, Reg, i32) {
    let Instruction::JumpIfNotEqual { src1, src2, offset } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("jump_if_not_equal called on non-JumpIfNotEqual instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (src1, src2, offset)
}

#[inline(always)]
pub unsafe fn jump_if_not_equal_k(ip: *const Instruction) -> (Reg, Const, i32) {
    let Instruction::JumpIfNotEqualK { src1, src2, offset } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("jump_if_not_equal_k called on non-JumpIfNotEqualK instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    (src1, src2, offset)
}

#[inline(always)]
pub unsafe fn capture_value(ip: *const Instruction) -> Reg {
    let Instruction::CaptureValue { src } = (unsafe { *ip }) else {
        if cfg!(debug_assertions) {
            unreachable!("capture_value called on non-CaptureValue instruction");
        } else {
            unsafe { unreachable_unchecked() }
        }
    };
    src
}

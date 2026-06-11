use crate::runtime::{
    instruction::Instruction,
    operands::{Const, Reg},
};

use std::hint::unreachable_unchecked;

#[allow(clippy::missing_safety_doc)]
impl Instruction {
    #[inline(always)]
    pub unsafe fn decode_add(ip: *const Self) -> (Reg, Reg, Reg) {
        let Instruction::Add { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_add called on non-Add instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_add_k(ip: *const Self) -> (Reg, Reg, Const) {
        let Instruction::AddK { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_add_k called on non-AddK instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_subtract(ip: *const Self) -> (Reg, Reg, Reg) {
        let Instruction::Subtract { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_subtract called on non-Subtract instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_subtract_rk(ip: *const Self) -> (Reg, Reg, Const) {
        let Instruction::SubtractRK { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_subtract_rk called on non-SubtractRK instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_subtract_kr(ip: *const Self) -> (Reg, Const, Reg) {
        let Instruction::SubtractKR { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_subtract_kr called on non-SubtractKR instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_multiply(ip: *const Self) -> (Reg, Reg, Reg) {
        let Instruction::Multiply { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_multiply called on non-Multiply instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_multiply_k(ip: *const Self) -> (Reg, Reg, Const) {
        let Instruction::MultiplyK { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_multiply_k called on non-MultiplyK instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_divide(ip: *const Self) -> (Reg, Reg, Reg) {
        let Instruction::Divide { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_divide called on non-Divide instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_divide_rk(ip: *const Self) -> (Reg, Reg, Const) {
        let Instruction::DivideRK { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_divide_rk called on non-DivideRK instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_divide_kr(ip: *const Self) -> (Reg, Const, Reg) {
        let Instruction::DivideKR { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_divide_kr called on non-DivideKR instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_modulo(ip: *const Self) -> (Reg, Reg, Reg) {
        let Instruction::Modulo { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_modulo called on non-Modulo instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_modulo_rk(ip: *const Self) -> (Reg, Reg, Const) {
        let Instruction::ModuloRK { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_modulo_rk called on non-ModuloRK instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_modulo_kr(ip: *const Self) -> (Reg, Const, Reg) {
        let Instruction::ModuloKR { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_modulo_kr called on non-ModuloKR instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_equal(ip: *const Self) -> (Reg, Reg, Reg) {
        let Instruction::Equal { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_equal called on non-Equal instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_equal_k(ip: *const Self) -> (Reg, Reg, Const) {
        let Instruction::EqualK { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_equal_k called on non-EqualK instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_not_equal(ip: *const Self) -> (Reg, Reg, Reg) {
        let Instruction::NotEqual { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_not_equal called on non-NotEqual instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_not_equal_k(ip: *const Self) -> (Reg, Reg, Const) {
        let Instruction::NotEqualK { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_not_equal_k called on non-NotEqualK instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_less(ip: *const Self) -> (Reg, Reg, Reg) {
        let Instruction::Less { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_less called on non-Less instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_less_k(ip: *const Self) -> (Reg, Reg, Const) {
        let Instruction::LessK { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_less_k called on non-LessK instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_less_equal(ip: *const Self) -> (Reg, Reg, Reg) {
        let Instruction::LessEqual { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_less_equal called on non-LessEqual instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_less_equal_k(ip: *const Self) -> (Reg, Reg, Const) {
        let Instruction::LessEqualK { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_less_equal_k called on non-LessEqualK instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_greater(ip: *const Self) -> (Reg, Reg, Reg) {
        let Instruction::Greater { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_greater called on non-Greater instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_greater_k(ip: *const Self) -> (Reg, Reg, Const) {
        let Instruction::GreaterK { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_greater_k called on non-GreaterK instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_greater_equal(ip: *const Self) -> (Reg, Reg, Reg) {
        let Instruction::GreaterEqual { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_greater_equal called on non-GreaterEqual instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_greater_equal_k(ip: *const Self) -> (Reg, Reg, Const) {
        let Instruction::GreaterEqualK { dest, src1, src2 } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_greater_equal_k called on non-GreaterEqualK instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src1, src2)
    }

    #[inline(always)]
    pub unsafe fn decode_not(ip: *const Self) -> (Reg, Reg) {
        let Instruction::Not { dest, src } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_not called on non-Not instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src)
    }

    #[inline(always)]
    pub unsafe fn decode_negate(ip: *const Self) -> (Reg, Reg) {
        let Instruction::Negate { dest, src } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_negate called on non-Negate instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src)
    }

    #[inline(always)]
    pub unsafe fn decode_move(ip: *const Self) -> (Reg, Reg) {
        let Instruction::Move { dest, src } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_move called on non-Move instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src)
    }

    #[inline(always)]
    pub unsafe fn decode_load_const(ip: *const Self) -> (Reg, Const) {
        let Instruction::LoadConst { dest, src } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_load_const called on non-LoadConst instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src)
    }

    #[inline(always)]
    pub unsafe fn decode_create_map(ip: *const Self) -> Reg {
        let Instruction::CreateMap { dest } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_create_map called on non-CreateMap instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        dest
    }

    #[inline(always)]
    pub unsafe fn decode_set_field(ip: *const Self) -> (Reg, Reg, Reg) {
        let Instruction::SetField { object, key, value } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_set_field called on non-SetField instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (object, key, value)
    }

    #[inline(always)]
    pub unsafe fn decode_get_field(ip: *const Self) -> (Reg, Reg, Reg) {
        let Instruction::GetField { dest, object, key } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_get_field called on non-GetField instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, object, key)
    }

    #[inline(always)]
    pub unsafe fn decode_create_closure(ip: *const Self) -> (Reg, u32, u8) {
        let Instruction::CreateClosure {
            dest,
            src,
            captures,
        } = (unsafe { *ip })
        else {
            if cfg!(debug_assertions) {
                unreachable!("decode_create_closure called on non-CreateClosure instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src, captures)
    }

    #[inline(always)]
    pub unsafe fn decode_create_ref(ip: *const Self) -> (Reg, Reg) {
        let Instruction::CreateRef { dest, src } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_create_ref called on non-CreateRef instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src)
    }

    #[inline(always)]
    pub unsafe fn decode_deref_set(ip: *const Self) -> (Reg, Reg) {
        let Instruction::DerefSet { dest, src } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_deref_set called on non-DerefSet instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src)
    }

    #[inline(always)]
    pub unsafe fn decode_deref(ip: *const Self) -> (Reg, Reg) {
        let Instruction::Deref { dest, src } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_deref called on non-Deref instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src)
    }

    #[inline(always)]
    pub unsafe fn decode_call(ip: *const Self) -> (Reg, Reg) {
        let Instruction::Call { dest, src } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_call called on non-Call instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (dest, src)
    }

    #[inline(always)]
    pub unsafe fn decode_return(ip: *const Self) -> Reg {
        let Instruction::Return { src } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_return called on non-Return instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        src
    }

    #[inline(always)]
    pub unsafe fn decode_jump(ip: *const Self) -> i32 {
        let Instruction::Jump { offset } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_jump called on non-Jump instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        offset
    }

    #[inline(always)]
    pub unsafe fn decode_jump_if_false(ip: *const Self) -> (Reg, i32) {
        let Instruction::JumpIfFalse { src, offset } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_jump_if_false called on non-JumpIfFalse instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (src, offset)
    }

    #[inline(always)]
    pub unsafe fn decode_jump_if_true(ip: *const Self) -> (Reg, i32) {
        let Instruction::JumpIfTrue { src, offset } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_jump_if_true called on non-JumpIfTrue instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (src, offset)
    }

    #[inline(always)]
    pub unsafe fn decode_jump_if_less(ip: *const Self) -> (Reg, Reg, i32) {
        let Instruction::JumpIfLess { src1, src2, offset } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_jump_if_less called on non-JumpIfLess instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (src1, src2, offset)
    }

    #[inline(always)]
    pub unsafe fn decode_jump_if_less_k(ip: *const Self) -> (Reg, Const, i32) {
        let Instruction::JumpIfLessK { src1, src2, offset } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_jump_if_less_k called on non-JumpIfLessK instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (src1, src2, offset)
    }

    #[inline(always)]
    pub unsafe fn decode_jump_if_less_equal(ip: *const Self) -> (Reg, Reg, i32) {
        let Instruction::JumpIfLessEqual { src1, src2, offset } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_jump_if_less_equal called on non-JumpIfLessEqual instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (src1, src2, offset)
    }

    #[inline(always)]
    pub unsafe fn decode_jump_if_less_equal_k(ip: *const Self) -> (Reg, Const, i32) {
        let Instruction::JumpIfLessEqualK { src1, src2, offset } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!(
                    "decode_jump_if_less_equal_k called on non-JumpIfLessEqualK instruction"
                );
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (src1, src2, offset)
    }

    #[inline(always)]
    pub unsafe fn decode_jump_if_greater(ip: *const Self) -> (Reg, Reg, i32) {
        let Instruction::JumpIfGreater { src1, src2, offset } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_jump_if_greater called on non-JumpIfGreater instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (src1, src2, offset)
    }

    #[inline(always)]
    pub unsafe fn decode_jump_if_greater_k(ip: *const Self) -> (Reg, Const, i32) {
        let Instruction::JumpIfGreaterK { src1, src2, offset } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_jump_if_greater_k called on non-JumpIfGreaterK instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (src1, src2, offset)
    }

    #[inline(always)]
    pub unsafe fn decode_jump_if_greater_equal(ip: *const Self) -> (Reg, Reg, i32) {
        let Instruction::JumpIfGreaterEqual { src1, src2, offset } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!(
                    "decode_jump_if_greater_equal called on non-JumpIfGreaterEqual instruction"
                );
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (src1, src2, offset)
    }

    #[inline(always)]
    pub unsafe fn decode_jump_if_greater_equal_k(ip: *const Self) -> (Reg, Const, i32) {
        let Instruction::JumpIfGreaterEqualK { src1, src2, offset } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!(
                    "decode_jump_if_greater_equal_k called on non-JumpIfGreaterEqualK instruction"
                );
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (src1, src2, offset)
    }

    #[inline(always)]
    pub unsafe fn decode_jump_if_equal(ip: *const Self) -> (Reg, Reg, i32) {
        let Instruction::JumpIfEqual { src1, src2, offset } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_jump_if_equal called on non-JumpIfEqual instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (src1, src2, offset)
    }

    #[inline(always)]
    pub unsafe fn decode_jump_if_equal_k(ip: *const Self) -> (Reg, Const, i32) {
        let Instruction::JumpIfEqualK { src1, src2, offset } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_jump_if_equal_k called on non-JumpIfEqualK instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (src1, src2, offset)
    }

    #[inline(always)]
    pub unsafe fn decode_jump_if_not_equal(ip: *const Self) -> (Reg, Reg, i32) {
        let Instruction::JumpIfNotEqual { src1, src2, offset } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_jump_if_not_equal called on non-JumpIfNotEqual instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (src1, src2, offset)
    }

    #[inline(always)]
    pub unsafe fn decode_jump_if_not_equal_k(ip: *const Self) -> (Reg, Const, i32) {
        let Instruction::JumpIfNotEqualK { src1, src2, offset } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!(
                    "decode_jump_if_not_equal_k called on non-JumpIfNotEqualK instruction"
                );
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        (src1, src2, offset)
    }

    #[inline(always)]
    pub unsafe fn decode_capture_value(ip: *const Self) -> Reg {
        let Instruction::CaptureValue { src } = (unsafe { *ip }) else {
            if cfg!(debug_assertions) {
                unreachable!("decode_capture_value called on non-CaptureValue instruction");
            } else {
                unsafe { unreachable_unchecked() }
            }
        };
        src
    }
}

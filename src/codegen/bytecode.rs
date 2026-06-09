use crate::runtime::{
    byte_function::Function,
    instruction::{Const, Reg},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    Add,
    AddK,
    Subtract,
    SubtractRK,
    SubtractKR,
    Multiply,
    MultiplyK,
    Divide,
    DivideRK,
    DivideKR,
    Modulo,
    ModuloRK,
    ModuloKR,
    Equal,
    EqualK,
    NotEqual,
    NotEqualK,
    Less,
    LessK,
    LessEqual,
    LessEqualK,
    Greater,
    GreaterK,
    GreaterEqual,
    GreaterEqualK,
    Not,
    Negate,
    Move,
    LoadConst,
    CreateMap,
    SetField,
    GetField,
    CreateClosure,
    CaptureValue,
    CreateRef,
    DerefSet,
    Deref,
    Call,
    Return,
    Jump,
    JumpIfFalse,
    JumpIfTrue,
    JumpIfLess,
    JumpIfLessK,
    JumpIfLessEqual,
    JumpIfLessEqualK,
    JumpIfGreater,
    JumpIfGreaterK,
    JumpIfGreaterEqual,
    JumpIfGreaterEqualK,
    JumpIfEqual,
    JumpIfEqualK,
    JumpIfNotEqual,
    JumpIfNotEqualK,
}

impl Function {
    pub fn emit_add(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::Add as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_add_k(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Const>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::AddK as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.extend_from_slice(&src2.0.to_le_bytes());
        index
    }

    pub fn emit_subtract(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::Subtract as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_subtract_rk(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Const>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::SubtractRK as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.extend_from_slice(&src2.0.to_le_bytes());
        index
    }

    pub fn emit_subtract_kr(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Const>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::SubtractKR as u8);
        self.bytes.push(dest.0);
        self.bytes.extend_from_slice(&src1.0.to_le_bytes());
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_multiply(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::Multiply as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_multiply_k(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Const>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::MultiplyK as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.extend_from_slice(&src2.0.to_le_bytes());
        index
    }

    pub fn emit_divide(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::Divide as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_divide_rk(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Const>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::DivideRK as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.extend_from_slice(&src2.0.to_le_bytes());
        index
    }

    pub fn emit_divide_kr(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Const>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::DivideKR as u8);
        self.bytes.push(dest.0);
        self.bytes.extend_from_slice(&src1.0.to_le_bytes());
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_modulo(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::Modulo as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_modulo_rk(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Const>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::ModuloRK as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.extend_from_slice(&src2.0.to_le_bytes());
        index
    }

    pub fn emit_modulo_kr(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Const>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::ModuloKR as u8);
        self.bytes.push(dest.0);
        self.bytes.extend_from_slice(&src1.0.to_le_bytes());
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_equal(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::Equal as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_equal_k(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Const>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::EqualK as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.extend_from_slice(&src2.0.to_le_bytes());
        index
    }

    pub fn emit_not_equal(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::NotEqual as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_not_equal_k(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Const>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::NotEqualK as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.extend_from_slice(&src2.0.to_le_bytes());
        index
    }

    pub fn emit_less(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::Less as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_less_k(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Const>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::LessK as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.extend_from_slice(&src2.0.to_le_bytes());
        index
    }

    pub fn emit_less_equal(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::LessEqual as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_less_equal_k(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Const>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::LessEqualK as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.extend_from_slice(&src2.0.to_le_bytes());
        index
    }

    pub fn emit_greater(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::Greater as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_greater_k(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Const>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::GreaterK as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.extend_from_slice(&src2.0.to_le_bytes());
        index
    }

    pub fn emit_greater_equal(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::GreaterEqual as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_greater_equal_k(
        &mut self,
        dest: impl Into<Reg>,
        src1: impl Into<Reg>,
        src2: impl Into<Const>,
    ) -> usize {
        let (dest, src1, src2) = (dest.into(), src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::GreaterEqualK as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src1.0);
        self.bytes.extend_from_slice(&src2.0.to_le_bytes());
        index
    }

    pub fn emit_not(&mut self, dest: impl Into<Reg>, src: impl Into<Reg>) -> usize {
        let (dest, src) = (dest.into(), src.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::Not as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src.0);
        index
    }

    pub fn emit_negate(&mut self, dest: impl Into<Reg>, src: impl Into<Reg>) -> usize {
        let (dest, src) = (dest.into(), src.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::Negate as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src.0);
        index
    }

    pub fn emit_move(&mut self, dest: impl Into<Reg>, src: impl Into<Reg>) -> usize {
        let (dest, src) = (dest.into(), src.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::Move as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src.0);
        index
    }

    pub fn emit_load_const(&mut self, dest: impl Into<Reg>, src: impl Into<Const>) -> usize {
        let (dest, src) = (dest.into(), src.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::LoadConst as u8);
        self.bytes.push(dest.0);
        self.bytes.extend_from_slice(&src.0.to_le_bytes());
        index
    }

    pub fn emit_create_map(&mut self, dest: impl Into<Reg>) -> usize {
        let dest = dest.into();
        let index = self.bytes.len();
        self.bytes.push(Opcode::CreateMap as u8);
        self.bytes.push(dest.0);
        index
    }

    pub fn emit_set_field(
        &mut self,
        object: impl Into<Reg>,
        key: impl Into<Reg>,
        value: impl Into<Reg>,
    ) -> usize {
        let (object, key, value) = (object.into(), key.into(), value.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::SetField as u8);
        self.bytes.push(object.0);
        self.bytes.push(key.0);
        self.bytes.push(value.0);
        index
    }

    pub fn emit_get_field(
        &mut self,
        dest: impl Into<Reg>,
        object: impl Into<Reg>,
        key: impl Into<Reg>,
    ) -> usize {
        let (dest, object, key) = (dest.into(), object.into(), key.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::GetField as u8);
        self.bytes.push(dest.0);
        self.bytes.push(object.0);
        self.bytes.push(key.0);
        index
    }

    pub fn emit_create_closure(&mut self, dest: impl Into<Reg>, captures: u8) -> usize {
        let dest = dest.into();
        let index = self.bytes.len();
        self.bytes.push(Opcode::CreateClosure as u8);
        self.bytes.push(dest.0);
        self.bytes.push(captures);
        index
    }

    pub fn emit_capture_value(&mut self, dest: impl Into<Reg>, src: impl Into<Reg>) -> usize {
        let (dest, src) = (dest.into(), src.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::CaptureValue as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src.0);
        index
    }

    pub fn emit_create_ref(&mut self, dest: impl Into<Reg>, src: impl Into<Reg>) -> usize {
        let (dest, src) = (dest.into(), src.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::CreateRef as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src.0);
        index
    }

    pub fn emit_deref_set(&mut self, dest: impl Into<Reg>, src: impl Into<Reg>) -> usize {
        let (dest, src) = (dest.into(), src.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::DerefSet as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src.0);
        index
    }

    pub fn emit_deref(&mut self, dest: impl Into<Reg>, src: impl Into<Reg>) -> usize {
        let (dest, src) = (dest.into(), src.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::Deref as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src.0);
        index
    }

    pub fn emit_call(&mut self, dest: impl Into<Reg>, src: impl Into<Reg>) -> usize {
        let (dest, src) = (dest.into(), src.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::Call as u8);
        self.bytes.push(dest.0);
        self.bytes.push(src.0);
        index
    }

    pub fn emit_return(&mut self, src: impl Into<Reg>) -> usize {
        let src = src.into();
        let index = self.bytes.len();
        self.bytes.push(Opcode::Return as u8);
        self.bytes.push(src.0);
        index
    }

    pub fn emit_jump(&mut self, offset: i32) -> usize {
        let index = self.bytes.len();
        self.bytes.push(Opcode::Jump as u8);
        self.bytes.extend_from_slice(&offset.to_le_bytes());
        index
    }

    pub fn emit_jump_if_false(&mut self, offset: i32, src: impl Into<Reg>) -> usize {
        let src = src.into();
        let index = self.bytes.len();
        self.bytes.push(Opcode::JumpIfFalse as u8);
        self.bytes.extend_from_slice(&offset.to_le_bytes());
        self.bytes.push(src.0);
        index
    }

    pub fn emit_jump_if_true(&mut self, offset: i32, src: impl Into<Reg>) -> usize {
        let src = src.into();
        let index = self.bytes.len();
        self.bytes.push(Opcode::JumpIfTrue as u8);
        self.bytes.extend_from_slice(&offset.to_le_bytes());
        self.bytes.push(src.0);
        index
    }

    pub fn emit_jump_if_less(
        &mut self,
        offset: i32,
        src1: impl Into<Reg>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (src1, src2) = (src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::JumpIfLess as u8);
        self.bytes.extend_from_slice(&offset.to_le_bytes());
        self.bytes.push(src1.0);
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_jump_if_less_k(
        &mut self,
        offset: i32,
        src1: impl Into<Reg>,
        src2: impl Into<Const>,
    ) -> usize {
        let (src1, src2) = (src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::JumpIfLessK as u8);
        self.bytes.extend_from_slice(&offset.to_le_bytes());
        self.bytes.push(src1.0);
        self.bytes.extend_from_slice(&src2.0.to_le_bytes());
        index
    }

    pub fn emit_jump_if_less_equal(
        &mut self,
        offset: i32,
        src1: impl Into<Reg>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (src1, src2) = (src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::JumpIfLessEqual as u8);
        self.bytes.extend_from_slice(&offset.to_le_bytes());
        self.bytes.push(src1.0);
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_jump_if_less_equal_k(
        &mut self,
        offset: i32,
        src1: impl Into<Reg>,
        src2: impl Into<Const>,
    ) -> usize {
        let (src1, src2) = (src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::JumpIfLessEqualK as u8);
        self.bytes.extend_from_slice(&offset.to_le_bytes());
        self.bytes.push(src1.0);
        self.bytes.extend_from_slice(&src2.0.to_le_bytes());
        index
    }

    pub fn emit_jump_if_greater(
        &mut self,
        offset: i32,
        src1: impl Into<Reg>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (src1, src2) = (src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::JumpIfGreater as u8);
        self.bytes.extend_from_slice(&offset.to_le_bytes());
        self.bytes.push(src1.0);
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_jump_if_greater_k(
        &mut self,
        offset: i32,
        src1: impl Into<Reg>,
        src2: impl Into<Const>,
    ) -> usize {
        let (src1, src2) = (src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::JumpIfGreaterK as u8);
        self.bytes.extend_from_slice(&offset.to_le_bytes());
        self.bytes.push(src1.0);
        self.bytes.extend_from_slice(&src2.0.to_le_bytes());
        index
    }

    pub fn emit_jump_if_greater_equal(
        &mut self,
        offset: i32,
        src1: impl Into<Reg>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (src1, src2) = (src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::JumpIfGreaterEqual as u8);
        self.bytes.extend_from_slice(&offset.to_le_bytes());
        self.bytes.push(src1.0);
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_jump_if_greater_equal_k(
        &mut self,
        offset: i32,
        src1: impl Into<Reg>,
        src2: impl Into<Const>,
    ) -> usize {
        let (src1, src2) = (src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::JumpIfGreaterEqualK as u8);
        self.bytes.extend_from_slice(&offset.to_le_bytes());
        self.bytes.push(src1.0);
        self.bytes.extend_from_slice(&src2.0.to_le_bytes());
        index
    }

    pub fn emit_jump_if_equal(
        &mut self,
        offset: i32,
        src1: impl Into<Reg>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (src1, src2) = (src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::JumpIfEqual as u8);
        self.bytes.extend_from_slice(&offset.to_le_bytes());
        self.bytes.push(src1.0);
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_jump_if_equal_k(
        &mut self,
        offset: i32,
        src1: impl Into<Reg>,
        src2: impl Into<Const>,
    ) -> usize {
        let (src1, src2) = (src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::JumpIfEqualK as u8);
        self.bytes.extend_from_slice(&offset.to_le_bytes());
        self.bytes.push(src1.0);
        self.bytes.extend_from_slice(&src2.0.to_le_bytes());
        index
    }

    pub fn emit_jump_if_not_equal(
        &mut self,
        offset: i32,
        src1: impl Into<Reg>,
        src2: impl Into<Reg>,
    ) -> usize {
        let (src1, src2) = (src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::JumpIfNotEqual as u8);
        self.bytes.extend_from_slice(&offset.to_le_bytes());
        self.bytes.push(src1.0);
        self.bytes.push(src2.0);
        index
    }

    pub fn emit_jump_if_not_equal_k(
        &mut self,
        offset: i32,
        src1: impl Into<Reg>,
        src2: impl Into<Const>,
    ) -> usize {
        let (src1, src2) = (src1.into(), src2.into());
        let index = self.bytes.len();
        self.bytes.push(Opcode::JumpIfNotEqualK as u8);
        self.bytes.extend_from_slice(&offset.to_le_bytes());
        self.bytes.push(src1.0);
        self.bytes.extend_from_slice(&src2.0.to_le_bytes());
        index
    }

    /// Patches any jump instruction at `index` with the correct byte offset.
    /// Works for all jump variants since offset is always at bytes [index+1..index+5].
    pub fn patch_jump(&mut self, index: usize) {
        let offset = self.bytes.len() as i32 - index as i32;
        let bytes = offset.to_le_bytes();
        self.bytes[index + 1] = bytes[0];
        self.bytes[index + 2] = bytes[1];
        self.bytes[index + 3] = bytes[2];
        self.bytes[index + 4] = bytes[3];
    }
}

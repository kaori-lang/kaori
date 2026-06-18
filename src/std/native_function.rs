use crate::runtime::value::Value;

#[derive(Clone, Copy)]
pub struct NativeFunction(fn(&[Value]) -> Value);

impl NativeFunction {
    pub const fn new(f: fn(&[Value]) -> Value) -> Self {
        Self(f)
    }

    pub fn call(&self, args: &[Value]) -> Value {
        (self.0)(args)
    }
}

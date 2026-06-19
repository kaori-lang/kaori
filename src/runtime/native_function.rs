use crate::{diagnostics::error::Error, runtime::value::Value};

type Func = fn(&[Value]) -> Result<Value, Error>;

#[derive(Clone, Copy)]
pub struct NativeFunction(Func);

impl NativeFunction {
    pub const fn new(f: Func) -> Self {
        Self(f)
    }

    pub fn call(&self, args: &[Value]) -> Result<Value, Error> {
        (self.0)(args)
    }
}

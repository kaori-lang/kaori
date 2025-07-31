#[derive(Debug)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Str(String),
}

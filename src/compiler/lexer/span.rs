#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub line: u32,
    pub start: usize,
    pub size: usize,
}

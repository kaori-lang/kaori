#[derive(Debug, Clone, Copy, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn merge(left: Span, right: Span) -> Span {
        Span {
            start: left.start,
            end: right.end,
        }
    }
}

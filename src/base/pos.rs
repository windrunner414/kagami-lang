pub type BytePos = usize;

#[derive(Debug, Clone)]
pub struct Span<I> {
    pub start: I,
    pub end: I,
}

impl<I: Ord> Span<I> {
    pub fn new(start: I, end: I) -> Span<I> {
        if start > end {
            panic!("Span end must be greater than or equal to start");
        }

        Span { start, end }
    }
}

impl<I> Span<I> {
    pub const fn new_unchecked(start: I, end: I) -> Span<I> {
        Span { start, end }
    }
}

#[derive(Debug, Clone)]
pub struct Spanned<I, T> {
    pub span: Span<I>,
    pub value: T,
}

impl<I, T> From<(Span<I>, T)> for Spanned<I, T> {
    fn from((span, value): (Span<I>, T)) -> Spanned<I, T> {
        Spanned { span, value }
    }
}

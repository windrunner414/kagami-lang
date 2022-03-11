use crate::base::pos::{BytePos, Span, Spanned};
use anyhow::anyhow;
use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;

pub type Errors = Vec<Spanned<BytePos, anyhow::Error>>;

impl<'input> From<lalrpop_util::ParseError<BytePos, Token<'input>, &'static str>>
    for Spanned<BytePos, anyhow::Error>
{
    fn from(error: ParseError<BytePos, Token<'input>, &'static str>) -> Self {
        match error {
            ParseError::InvalidToken { location } => Spanned::from((
                Span::new_unchecked(location, location + 1),
                anyhow!("Invalid Token"),
            )),
            ParseError::UnrecognizedEOF { location, expected } => Spanned::from((
                Span::new_unchecked(location, location + 1),
                anyhow!("Unexpected EOF, expected {:?}", expected),
            )),
            ParseError::UnrecognizedToken {
                token: (start, token, end),
                expected,
            } => Spanned::from((
                Span::new_unchecked(start, end),
                anyhow!("Unexpected token {}, expected {:?}", token, expected),
            )),
            ParseError::ExtraToken {
                token: (start, token, end),
            } => Spanned::from((
                Span::new_unchecked(start, end),
                anyhow!("Unexpected token {}", token),
            )),
            ParseError::User { error } => {
                Spanned::from((Span::new_unchecked(0, 0), anyhow!(error.to_string())))
            }
        }
    }
}

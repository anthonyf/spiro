use crate::{lexer::LexError, parser::ParseError, spirograph::SpirographError};
use std::fmt;

/// Anything that can go wrong between a `.spiro` file and a drawing.
#[derive(Debug)]
pub enum SpiroError {
    Read(std::io::Error),
    Lex(LexError),
    Parse(ParseError),
    Shape(SpirographError),
}

// These let `?` turn each step's error into a SpiroError.
impl From<std::io::Error> for SpiroError {
    fn from(error: std::io::Error) -> Self {
        SpiroError::Read(error)
    }
}

impl From<LexError> for SpiroError {
    fn from(error: LexError) -> Self {
        SpiroError::Lex(error)
    }
}

impl From<ParseError> for SpiroError {
    fn from(error: ParseError) -> Self {
        SpiroError::Parse(error)
    }
}

impl From<SpirographError> for SpiroError {
    fn from(error: SpirographError) -> Self {
        SpiroError::Shape(error)
    }
}

impl fmt::Display for SpiroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpiroError::Read(error) => write!(f, "can't read the file: {error}"),
            SpiroError::Lex(error) => write!(f, "{error}"),
            SpiroError::Parse(error) => write!(f, "{error}"),
            SpiroError::Shape(error) => write!(f, "{error}"),
        }
    }
}

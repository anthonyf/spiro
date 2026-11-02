use std::fmt;

/// Where something is in the source text. Both numbers start at 1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

#[derive(Debug)]
struct Reader<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    line: u32,
    column: u32,
}
impl<'a> Reader<'a> {
    fn new(source: &'a str) -> Reader<'a> {
        Reader {
            chars: source.chars().peekable(),
            line: 1,
            column: 0,
        }
    }

    fn next(&mut self) -> Option<char> {
        if let Some(c) = self.chars.next() {
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
            Some(c)
        } else {
            None
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    /// The position of the character most recently returned by `next`.
    fn position(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
        }
    }

    fn next_while(&mut self, mut keep: impl FnMut(char) -> bool) -> String {
        let mut result = String::new();
        while let Some(next_c) = self.peek() {
            if keep(next_c) {
                result.push(next_c);
                self.next();
            } else {
                break;
            }
        }
        result
    }
}

#[derive(Debug, PartialEq)]
pub enum LexError {
    InvalidCharacter {
        position: Position,
        char: char,
    },
    DivideByZero {
        position: Position,
    },
    InvalidNumber {
        position: Position,
        non_number: String,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    OpenBrace,
    CloseBrace,
    Word(String),
    Integer(i32),
    Float(f32),
    Fraction { numerator: i32, denominator: i32 },
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::InvalidCharacter { position, char } => {
                write!(f, "{position}: unexpected character '{char}'")
            }
            LexError::DivideByZero { position } => {
                write!(f, "{position}: a fraction can't have a zero denominator")
            }
            LexError::InvalidNumber {
                position,
                non_number,
            } => write!(f, "{position}: '{non_number}' is not a valid number"),
        }
    }
}

/// How a token reads in an error message, e.g. "found '}'".
impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::OpenBrace => write!(f, "'{{'"),
            TokenKind::CloseBrace => write!(f, "'}}'"),
            TokenKind::Word(word) => write!(f, "'{word}'"),
            TokenKind::Integer(integer) => write!(f, "{integer}"),
            TokenKind::Float(float) => write!(f, "{float}"),
            TokenKind::Fraction {
                numerator,
                denominator,
            } => write!(f, "{numerator}/{denominator}"),
        }
    }
}

pub fn lex(source: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut chars = Reader::new(source);
    while let Some(c) = chars.next() {
        let start = chars.position();
        match c {
            '{' => {
                tokens.push(Token {
                    kind: TokenKind::OpenBrace,
                    position: start,
                });
            }
            '}' => {
                tokens.push(Token {
                    kind: TokenKind::CloseBrace,
                    position: start,
                });
            }
            '#' => {
                chars.next_while(|next_c| next_c != '\n');
            }
            c if c.is_alphabetic() => {
                let mut word = String::new();
                word.push(c);

                chars
                    .next_while(|next_c| next_c.is_alphanumeric() || next_c == '_')
                    .chars()
                    .for_each(|next_c| word.push(next_c));

                tokens.push(Token {
                    kind: TokenKind::Word(word),
                    position: start,
                });
            }
            c if c.is_ascii_digit()
                || (c == '-' && chars.peek().is_some_and(|next_c| next_c.is_ascii_digit())) =>
            {
                let mut number = String::new();
                number.push(c);
                chars
                    .next_while(|next_c| next_c.is_ascii_digit() || next_c == '.' || next_c == '/')
                    .chars()
                    .for_each(|next_c| number.push(next_c));

                // it is a fraction if it contains only a single '/' character and no decimal point
                let is_fraction = number.matches('/').count() == 1 && !number.contains('.');
                // it is a float if it contains a single decimal point and no '/' character
                let has_decimal_point = number.matches('.').count() == 1 && !number.contains('/');

                if is_fraction {
                    let parts: Vec<&str> = number.split('/').collect();
                    let numerator: i32 =
                        parts[0].parse().map_err(|_| LexError::InvalidCharacter {
                            position: chars.position(),
                            char: '/',
                        })?;
                    let denominator: i32 =
                        parts[1].parse().map_err(|_| LexError::InvalidCharacter {
                            position: chars.position(),
                            char: '/',
                        })?;
                    if denominator == 0 {
                        return Err(LexError::DivideByZero { position: start });
                    }
                    tokens.push(Token {
                        kind: TokenKind::Fraction {
                            numerator,
                            denominator,
                        },
                        position: start,
                    });
                } else if has_decimal_point {
                    let float: f32 = number.parse().map_err(|_| LexError::InvalidCharacter {
                        position: chars.position(),
                        char: '.',
                    })?;
                    tokens.push(Token {
                        kind: TokenKind::Float(float),
                        position: start,
                    });
                } else {
                    let integer: i32 = number.parse().map_err(|_| LexError::InvalidNumber {
                        position: start,
                        non_number: number.clone(),
                    })?;
                    tokens.push(Token {
                        kind: TokenKind::Integer(integer),
                        position: start,
                    });
                }
            }

            c if c.is_whitespace() => {}
            _ => {
                return Err(LexError::InvalidCharacter {
                    position: start,
                    char: c,
                });
            }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenKind::*;

    /// Shorthand for a position in test expectations.
    fn at(line: u32, column: u32) -> Position {
        Position { line, column }
    }

    /// Shorthand for an expected token.
    fn token(line: u32, column: u32, kind: TokenKind) -> Token {
        Token {
            kind,
            position: at(line, column),
        }
    }

    /// Lexes `source` and keeps only the token kinds, for tests that
    /// don't care about positions.
    fn kinds(source: &str) -> Vec<TokenKind> {
        lex(source)
            .unwrap()
            .into_iter()
            .map(|token| token.kind)
            .collect()
    }

    fn word(text: &str) -> TokenKind {
        Word(text.to_string())
    }

    #[test]
    fn test_reader_next_and_peek() {
        let mut reader = Reader::new("abc");
        assert_eq!(reader.next(), Some('a'));
        assert_eq!(reader.peek(), Some('b'));
        assert_eq!(reader.next(), Some('b'));
        assert_eq!(reader.peek(), Some('c'));
        assert_eq!(reader.next(), Some('c'));
        assert_eq!(reader.peek(), None);
        assert_eq!(reader.next(), None);
    }

    #[test]
    fn test_reader_position() {
        let mut reader = Reader::new("a\nb");
        assert_eq!(reader.position(), at(1, 0));
        reader.next();
        assert_eq!(reader.position(), at(1, 1));
        reader.next();
        assert_eq!(reader.position(), at(2, 0));
        reader.next();
        assert_eq!(reader.position(), at(2, 1));
    }

    #[test]
    fn test_lex_empty() {
        assert_eq!(kinds(""), vec![]);
    }

    #[test]
    fn test_lex_whitespace() {
        assert_eq!(kinds("   \n\t  "), vec![]);
    }

    #[test]
    fn test_lex_braces() {
        assert_eq!(
            lex("{ }").unwrap(),
            vec![token(1, 1, OpenBrace), token(1, 3, CloseBrace)]
        );
    }

    #[test]
    fn test_lex_comments() {
        assert_eq!(
            lex("{ # this is a comment\n }").unwrap(),
            vec![token(1, 1, OpenBrace), token(2, 2, CloseBrace)]
        );
    }

    #[test]
    fn test_lex_word() {
        assert_eq!(
            kinds("{ word_123 }"),
            vec![OpenBrace, word("word_123"), CloseBrace]
        );
    }

    #[test]
    fn test_lex_multiple_words() {
        assert_eq!(
            lex("{ word1 word2 }").unwrap(),
            vec![
                token(1, 1, OpenBrace),
                token(1, 3, word("word1")),
                token(1, 9, word("word2")),
                token(1, 15, CloseBrace),
            ]
        );
    }

    #[test]
    fn test_lex_single_character_word() {
        assert_eq!(kinds("{ a }"), vec![OpenBrace, word("a"), CloseBrace]);
    }

    #[test]
    fn test_lex_positive_integer() {
        assert_eq!(kinds("{ 123 }"), vec![OpenBrace, Integer(123), CloseBrace]);
    }

    #[test]
    fn test_lex_negative_integer() {
        assert_eq!(
            kinds("{ -123 }"),
            vec![OpenBrace, Integer(-123), CloseBrace]
        );
    }

    #[test]
    fn test_lex_positive_float() {
        assert_eq!(
            kinds("{ 123.456 }"),
            vec![OpenBrace, Float(123.456), CloseBrace]
        );
    }

    #[test]
    fn test_lex_negative_float() {
        assert_eq!(
            kinds("{ -123.456 }"),
            vec![OpenBrace, Float(-123.456), CloseBrace]
        );
    }

    #[test]
    fn test_lex_fraction() {
        assert_eq!(
            kinds("{ 1/2 }"),
            vec![
                OpenBrace,
                Fraction {
                    numerator: 1,
                    denominator: 2
                },
                CloseBrace
            ]
        );
    }

    #[test]
    fn test_lex_negative_fraction() {
        assert_eq!(
            kinds("{ -1/2 }"),
            vec![
                OpenBrace,
                Fraction {
                    numerator: -1,
                    denominator: 2
                },
                CloseBrace
            ]
        );
    }

    #[test]
    fn test_lex_number_positions() {
        assert_eq!(
            lex("123 -4 1.5 1/2").unwrap(),
            vec![
                token(1, 1, Integer(123)),
                token(1, 5, Integer(-4)),
                token(1, 8, Float(1.5)),
                token(
                    1,
                    12,
                    Fraction {
                        numerator: 1,
                        denominator: 2
                    }
                ),
            ]
        );
    }

    #[test]
    fn test_lex_fraction_with_float() {
        assert_eq!(
            lex("{ 1.5 / 2 }").unwrap_err(),
            LexError::InvalidCharacter {
                position: at(1, 7),
                char: '/',
            }
        );
    }

    #[test]
    fn test_lex_fraction_with_zero_denominator() {
        assert_eq!(
            lex("{ 1/0 }").unwrap_err(),
            LexError::DivideByZero { position: at(1, 3) }
        );
    }

    #[test]
    fn test_lex_invalid_number_multiple_slash() {
        assert_eq!(
            lex("{ 1/2/3 }").unwrap_err(),
            LexError::InvalidNumber {
                position: at(1, 3),
                non_number: "1/2/3".to_string(),
            }
        );
    }

    #[test]
    fn test_lex_invalid_number_multiple_decimal() {
        assert_eq!(
            lex("{ 1.2.3 }").unwrap_err(),
            LexError::InvalidNumber {
                position: at(1, 3),
                non_number: "1.2.3".to_string(),
            }
        );
    }

    #[test]
    fn test_lex_large_integer() {
        assert_eq!(
            lex("{ 99999999999 }").unwrap_err(),
            LexError::InvalidNumber {
                position: at(1, 3),
                non_number: "99999999999".to_string(),
            }
        );
    }

    #[test]
    fn test_lex_fraction_with_decimal() {
        assert_eq!(
            lex("{ 1.5/2 }").unwrap_err(),
            LexError::InvalidNumber {
                position: at(1, 3),
                non_number: "1.5/2".to_string(),
            }
        );
    }

    #[test]
    fn test_lex_hex_numbers_are_words() {
        assert_eq!(kinds("{ x123 }"), vec![OpenBrace, word("x123"), CloseBrace]);
    }

    #[test]
    fn test_lex_invalid_character() {
        assert_eq!(
            lex("{@}").unwrap_err(),
            LexError::InvalidCharacter {
                position: at(1, 2),
                char: '@',
            }
        );
    }

    #[test]
    fn test_lex_large_document() {
        let source = r#"
background white

# The same classic curve, written two ways
layer {
  pen x008080 1.5             # teal, as a hex color
  classic {
    ring 96
    arm 60 speed 3
    arm 28.8 speed -5
  }
}
        "#;
        assert_eq!(
            lex(source).unwrap(),
            vec![
                token(2, 1, word("background")),
                token(2, 12, word("white")),
                token(5, 1, word("layer")),
                token(5, 7, OpenBrace),
                token(6, 3, word("pen")),
                token(6, 7, word("x008080")),
                token(6, 15, Float(1.5)),
                token(7, 3, word("classic")),
                token(7, 11, OpenBrace),
                token(8, 5, word("ring")),
                token(8, 10, Integer(96)),
                token(9, 5, word("arm")),
                token(9, 9, Integer(60)),
                token(9, 12, word("speed")),
                token(9, 18, Integer(3)),
                token(10, 5, word("arm")),
                token(10, 9, Float(28.8)),
                token(10, 14, word("speed")),
                token(10, 20, Integer(-5)),
                token(11, 3, CloseBrace),
                token(12, 1, CloseBrace),
            ]
        );
    }
}

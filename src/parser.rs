use crate::{
    document::{Document, Layer, Pen, Shape},
    error::SpiroError,
    lexer::{Token, TokenKind, lex},
    spirograph::Arm,
};
use macroquad::prelude::Color;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken { expected: String, found: Token },
    UnexpectedEndOfInput { expected: String },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found } => write!(
                f,
                "{}: expected {expected}, found {}",
                found.position, found.kind
            ),
            ParseError::UnexpectedEndOfInput { expected } => {
                write!(f, "the file ended too soon: expected {expected}")
            }
        }
    }
}

/// Lexes and parses a whole `.spiro` source file into a document.
pub fn parse_source(source: &str) -> Result<Document, SpiroError> {
    let tokens = lex(source)?;
    Ok(parse(&tokens)?)
}

/// Parses tokens from the lexer into a document.
pub fn parse(tokens: &[Token]) -> Result<Document, ParseError> {
    Parser::new(tokens).document()
}

fn unexpected_token(token: &Token, expected: &str) -> ParseError {
    ParseError::UnexpectedToken {
        expected: expected.to_string(),
        found: token.clone(),
    }
}

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    tokens: &'a [Token],
    index: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, index: 0 }
    }

    fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.index)
    }

    fn next(&mut self) -> Option<&'a Token> {
        let token = self.tokens.get(self.index);
        if token.is_some() {
            self.index += 1;
        }
        token
    }

    fn peek_or_unexpected_end(&self, expected: &str) -> Result<&'a Token, ParseError> {
        let token = self
            .peek()
            .ok_or_else(|| ParseError::UnexpectedEndOfInput {
                expected: expected.to_string(),
            })?;
        Ok(token)
    }

    // expects the next token to be a word with the given text, and
    // returns an error if it is not
    fn expect_word(&mut self, text: &str) -> Result<(), ParseError> {
        let token = self.peek_or_unexpected_end(&format!("word '{text}'"))?;

        if let TokenKind::Word(word) = &token.kind
            && word == text
        {
            self.next();
            return Ok(());
        }
        Err(unexpected_token(token, &format!("word '{text}'")))
    }

    fn peek_word(&self, text: &str) -> bool {
        if let Some(token) = self.peek()
            && let TokenKind::Word(word) = &token.kind
        {
            return word == text;
        }
        false
    }

    fn expect(&mut self, kind: &TokenKind) -> Result<(), ParseError> {
        let token = self.peek_or_unexpected_end(&kind.to_string())?;
        if &token.kind == kind {
            self.next();
            return Ok(());
        }
        Err(unexpected_token(token, &kind.to_string()))
    }

    // accepts integer, decimal or fraction, and returns a f32
    fn decimal(&mut self) -> Result<f32, ParseError> {
        let token = self.peek_or_unexpected_end("decimal")?;
        match &token.kind {
            TokenKind::Integer(i) => {
                self.next();
                Ok(*i as f32)
            }
            TokenKind::Float(f) => {
                self.next();
                Ok(*f)
            }
            TokenKind::Fraction {
                numerator: n,
                denominator: d,
            } => {
                self.next();
                if *d == 0 {
                    return Err(ParseError::UnexpectedToken {
                        expected: "non-zero denominator".to_string(),
                        found: token.clone(),
                    });
                }
                Ok(*n as f32 / *d as f32)
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: "decimal".to_string(),
                found: token.clone(),
            }),
        }
    }

    // accepts only integers
    fn integer(&mut self) -> Result<i32, ParseError> {
        let token = self.peek_or_unexpected_end("integer")?;

        if let TokenKind::Integer(i) = &token.kind {
            self.next();
            Ok(*i)
        } else {
            Err(unexpected_token(token, "integer"))
        }
    }

    fn color(&mut self) -> Result<Color, ParseError> {
        let token = self.peek_or_unexpected_end("color")?;

        if let TokenKind::Word(w) = &token.kind {
            self.next();
            match w.as_str() {
                "beige" => Ok(macroquad::prelude::BEIGE),
                "black" => Ok(macroquad::prelude::BLACK),
                "blue" => Ok(macroquad::prelude::BLUE),
                "brown" => Ok(macroquad::prelude::BROWN),
                "darkblue" => Ok(macroquad::prelude::DARKBLUE),
                "darkbrown" => Ok(macroquad::prelude::DARKBROWN),
                "darkgray" => Ok(macroquad::prelude::DARKGRAY),
                "darkgreen" => Ok(macroquad::prelude::DARKGREEN),
                "darkpurple" => Ok(macroquad::prelude::DARKPURPLE),
                "gold" => Ok(macroquad::prelude::GOLD),
                "gray" => Ok(macroquad::prelude::GRAY),
                "green" => Ok(macroquad::prelude::GREEN),
                "lightgray" => Ok(macroquad::prelude::LIGHTGRAY),
                "lime" => Ok(macroquad::prelude::LIME),
                "magenta" => Ok(macroquad::prelude::MAGENTA),
                "maroon" => Ok(macroquad::prelude::MAROON),
                "orange" => Ok(macroquad::prelude::ORANGE),
                "pink" => Ok(macroquad::prelude::PINK),
                "purple" => Ok(macroquad::prelude::PURPLE),
                "red" => Ok(macroquad::prelude::RED),
                "skyblue" => Ok(macroquad::prelude::SKYBLUE),
                "violet" => Ok(macroquad::prelude::VIOLET),
                "white" => Ok(macroquad::prelude::WHITE),
                "yellow" => Ok(macroquad::prelude::YELLOW),
                _ =>
                // if the first character is an x, then try to parse the rest as a hex color code
                {
                    if let Some(hex) = w.strip_prefix('x') {
                        if hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
                            let r = u8::from_str_radix(&hex[0..2], 16)
                                .map_err(|_| unexpected_token(token, "hex color code"))?;
                            let g = u8::from_str_radix(&hex[2..4], 16)
                                .map_err(|_| unexpected_token(token, "hex color code"))?;
                            let b = u8::from_str_radix(&hex[4..6], 16)
                                .map_err(|_| unexpected_token(token, "hex color code"))?;
                            Ok(Color::from_rgba(r, g, b, 255))
                        } else {
                            Err(unexpected_token(token, "hex color code"))
                        }
                    } else {
                        Err(unexpected_token(token, "color"))
                    }
                }
            }
        } else {
            Err(unexpected_token(token, "color"))
        }
    }

    fn pen(&mut self) -> Result<Pen, ParseError> {
        self.expect_word("pen")?;
        let color = self.color()?;
        let thickness = self.decimal()?;
        Ok(Pen { color, thickness })
    }

    fn arm(&mut self) -> Result<Arm, ParseError> {
        self.expect_word("arm")?;
        let length = self.decimal()?;
        self.expect_word("speed")?;
        let speed = self.integer()?;

        let phase = if self.peek_word("phase") {
            self.expect_word("phase")?;
            self.decimal()?
        } else {
            0.0
        };

        Ok(Arm {
            length,
            speed,
            phase,
        })
    }

    fn unsigned(&mut self) -> Result<u32, ParseError> {
        let token = self.peek_or_unexpected_end("unsigned integer")?;

        if let TokenKind::Integer(i) = &token.kind {
            let n = u32::try_from(*i).map_err(|_| unexpected_token(token, "unsigned integer"))?;
            self.next();
            Ok(n)
        } else {
            Err(unexpected_token(token, "unsigned integer"))
        }
    }

    fn toy(&mut self) -> Result<Shape, ParseError> {
        self.expect_word("toy")?;
        self.expect_word("ring")?;
        let ring = self.unsigned()?;
        self.expect_word("wheel")?;
        let wheel = self.unsigned()?;
        self.expect_word("hole")?;
        let hole = self.decimal()?;
        Ok(Shape::Toy { ring, wheel, hole })
    }

    fn classic(&mut self) -> Result<Shape, ParseError> {
        self.expect_word("classic")?;
        self.expect(&TokenKind::OpenBrace)?;
        self.expect_word("ring")?;
        let ring = self.decimal()?;
        let mut arms = Vec::new();
        while self.peek_word("arm") {
            arms.push(self.arm()?);
        }
        self.expect(&TokenKind::CloseBrace)?;
        Ok(Shape::Classic { ring, arms })
    }

    fn fractal(&mut self) -> Result<Shape, ParseError> {
        self.expect_word("fractal")?;
        self.expect(&TokenKind::OpenBrace)?;
        self.expect_word("length")?;
        let length = self.decimal()?;
        self.expect_word("speed")?;
        let speed = self.integer()?;
        self.expect_word("length_multiplier")?;
        let length_multiplier = self.decimal()?;
        self.expect_word("speed_multiplier")?;
        let speed_multiplier = self.integer()?;
        self.expect_word("count")?;
        let iterations = self.unsigned()?;
        self.expect(&TokenKind::CloseBrace)?;
        Ok(Shape::Fractal {
            length,
            speed,
            length_multiplier,
            speed_multiplier,
            iterations,
        })
    }

    fn shape(&mut self) -> Result<Shape, ParseError> {
        let word_token = self
            .peek()
            .ok_or_else(|| ParseError::UnexpectedEndOfInput {
                expected: "toy, fractal or classic".to_string(),
            })?;
        match word_token.kind {
            TokenKind::Word(ref w) if w == "toy" => self.toy(),
            TokenKind::Word(ref w) if w == "classic" => self.classic(),
            TokenKind::Word(ref w) if w == "fractal" => self.fractal(),
            _ => Err(unexpected_token(word_token, "toy, fractal or classic")),
        }
    }

    fn layer(&mut self) -> Result<Layer, ParseError> {
        self.expect_word("layer")?;
        self.expect(&TokenKind::OpenBrace)?;
        let pen = self.pen()?;
        let shape = self.shape()?;
        self.expect(&TokenKind::CloseBrace)?;
        Ok(Layer { pen, shape })
    }

    fn document(&mut self) -> Result<Document, ParseError> {
        let background_color = if self.peek_word("background") {
            self.expect_word("background")?;
            self.color()?
        } else {
            macroquad::prelude::WHITE
        };
        let mut layers = Vec::new();
        while self.peek().is_some() {
            layers.push(self.layer()?);
        }
        Ok(Document {
            background_color,
            layers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::{_classic, _toy};
    use crate::lexer::Position;

    #[test]
    fn test_parser_expect_word() {
        let tokens = lex("hello world").unwrap();
        let mut parser = Parser::new(&tokens);
        assert!(parser.expect_word("hello").is_ok());
        assert!(parser.expect_word("world").is_ok());
        assert_eq!(
            parser.expect_word("foo"),
            Err(ParseError::UnexpectedEndOfInput {
                expected: "word 'foo'".to_string()
            })
        );
    }

    #[test]
    fn test_parser_expect() {
        let tokens = lex("hello world -123").unwrap();
        let mut parser = Parser::new(&tokens);
        assert!(parser.expect(&TokenKind::Word("hello".to_string())).is_ok());
        assert!(parser.expect(&TokenKind::Word("world".to_string())).is_ok());
        assert!(parser.expect(&TokenKind::Integer(-123)).is_ok());
        assert_eq!(
            parser.expect(&TokenKind::Word("foo".to_string())),
            Err(ParseError::UnexpectedEndOfInput {
                expected: "'foo'".to_string()
            })
        );
    }

    #[test]
    fn test_parser_decimal() {
        let tokens = lex("123 45.67 1/2").unwrap();
        let mut parser = Parser::new(&tokens);
        assert_eq!(parser.decimal().unwrap(), 123.0);
        assert_eq!(parser.decimal().unwrap(), 45.67);
        assert_eq!(parser.decimal().unwrap(), 0.5);
    }

    #[test]
    fn test_parser_integer() {
        let tokens = lex("123 -456").unwrap();
        let mut parser = Parser::new(&tokens);
        assert_eq!(parser.integer().unwrap(), 123);
        assert_eq!(parser.integer().unwrap(), -456);
    }

    #[test]
    fn test_parser_color() {
        let tokens = lex("red x00ff00 blue").unwrap();
        let mut parser = Parser::new(&tokens);
        assert_eq!(parser.color().unwrap(), macroquad::prelude::RED);
        assert_eq!(parser.color().unwrap(), Color::from_rgba(0, 255, 0, 255));
        assert_eq!(parser.color().unwrap(), macroquad::prelude::BLUE);
    }

    #[test]
    fn test_parser_non_ascii_color() {
        let tokens = lex("xaébcd").unwrap();
        let mut parser = Parser::new(&tokens);
        assert_eq!(
            parser.color().unwrap_err(),
            ParseError::UnexpectedToken {
                expected: "hex color code".to_string(),
                found: Token {
                    kind: TokenKind::Word("xaébcd".to_string()),
                    position: Position { line: 1, column: 1 }
                },
            }
        )
    }

    #[test]
    fn test_parser_pen() {
        let tokens = lex("pen red 2.5").unwrap();
        let mut parser = Parser::new(&tokens);
        let pen = parser.pen().unwrap();
        assert_eq!(pen.color, macroquad::prelude::RED);
        assert_eq!(pen.thickness, 2.5);
    }

    #[test]
    fn test_parser_arm_no_phase() {
        let tokens = lex("arm 10.0 speed 5").unwrap();
        let mut parser = Parser::new(&tokens);
        let arm = parser.arm().unwrap();
        assert_eq!(arm.length, 10.0);
        assert_eq!(arm.speed, 5);
        assert_eq!(arm.phase, 0.0);
    }

    #[test]
    fn test_parser_arm_with_phase() {
        let tokens = lex("arm 10.0 speed 5 phase 1.57").unwrap();
        let mut parser = Parser::new(&tokens);
        let arm = parser.arm().unwrap();
        assert_eq!(arm.length, 10.0);
        assert_eq!(arm.speed, 5);
        assert_eq!(arm.phase, 1.57);
    }

    #[test]
    fn test_parser_toy() {
        let tokens = lex("toy ring 96 wheel 36 hole 28.8").unwrap();
        let mut parser = Parser::new(&tokens);
        let shape = parser.toy().unwrap();
        assert_eq!(
            shape,
            Shape::Toy {
                ring: 96,
                wheel: 36,
                hole: 28.8
            }
        );
    }

    #[test]
    fn test_parser_toy_invalid() {
        let tokens = lex("toy ring -96 wheel 36 hole 28.8").unwrap();
        let mut parser = Parser::new(&tokens);
        let error = parser.toy().unwrap_err();
        assert_eq!(
            error,
            ParseError::UnexpectedToken {
                expected: "unsigned integer".to_string(),
                found: Token {
                    kind: TokenKind::Integer(-96),
                    position: Position {
                        line: 1,
                        column: 10
                    }
                },
            }
        );
    }

    #[test]
    fn test_parser_classic() {
        let tokens = lex("classic {
    ring 96
    arm 60 speed 3
    arm 28.8 speed -5
  }")
        .unwrap();
        let mut parser = Parser::new(&tokens);
        let shape = parser.classic().unwrap();
        assert_eq!(
            shape,
            Shape::Classic {
                ring: 96.0,
                arms: vec![
                    Arm {
                        length: 60.0,
                        speed: 3,
                        phase: 0.0
                    },
                    Arm {
                        length: 28.8,
                        speed: -5,
                        phase: 0.0
                    }
                ]
            }
        );
    }

    #[test]
    fn test_parser_classic_no_arms() {
        let tokens = lex("classic { ring 96 }").unwrap();
        let mut parser = Parser::new(&tokens);
        let shape = parser.classic().unwrap();
        assert_eq!(
            shape,
            Shape::Classic {
                ring: 96.0,
                arms: vec![]
            }
        );
    }

    #[test]
    fn test_parser_classic_missing_closing_brace() {
        let tokens = lex("classic { ring 96 arm 60 speed 3").unwrap();
        let mut parser = Parser::new(&tokens);
        let error = parser.classic().unwrap_err();
        assert_eq!(
            error,
            ParseError::UnexpectedEndOfInput {
                expected: "'}'".to_string()
            }
        );
    }

    #[test]
    fn test_parser_fractal() {
        let tokens = lex("fractal {
    length 60                 # the first arm...
    speed -3
    length_multiplier 1/3     # ...each next one a third as long...
    speed_multiplier -4       # ...and 4x faster, reversed
    count 7
  }")
        .unwrap();
        let mut parser = Parser::new(&tokens);
        let shape = parser.fractal().unwrap();
        assert_eq!(
            shape,
            Shape::Fractal {
                length: 60.0,
                speed: -3,
                length_multiplier: 1.0 / 3.0,
                speed_multiplier: -4,
                iterations: 7
            }
        );
    }

    #[test]
    fn test_parser_shape_toy() {
        let tokens = lex("toy ring 96 wheel 36 hole 28.8").unwrap();
        let mut parser = Parser::new(&tokens);
        let shape = parser.shape().unwrap();
        assert_eq!(
            shape,
            Shape::Toy {
                ring: 96,
                wheel: 36,
                hole: 28.8
            }
        );
    }

    #[test]
    fn test_parser_shape_classic() {
        let tokens = lex("classic {
            ring 96
            arm 60 speed 3
            arm 28.8 speed -5
        }")
        .unwrap();
        let mut parser = Parser::new(&tokens);
        let shape = parser.shape().unwrap();
        assert_eq!(
            shape,
            Shape::Classic {
                ring: 96.0,
                arms: vec![
                    Arm {
                        length: 60.0,
                        speed: 3,
                        phase: 0.0
                    },
                    Arm {
                        length: 28.8,
                        speed: -5,
                        phase: 0.0
                    },
                ]
            }
        );
    }

    #[test]
    fn test_parser_shape_fractal() {
        let tokens = lex("fractal {
            length 60
            speed -3
            length_multiplier 1/3
            speed_multiplier -4
            count 7
        }")
        .unwrap();
        let mut parser = Parser::new(&tokens);
        let shape = parser.shape().unwrap();
        assert_eq!(
            shape,
            Shape::Fractal {
                length: 60.0,
                speed: -3,
                length_multiplier: 1.0 / 3.0,
                speed_multiplier: -4,
                iterations: 7
            }
        );
    }

    #[test]
    fn test_parser_readme_doc() {
        let tokens = lex("background white

# The same classic curve, written two ways
layer {
  pen x008080 1.5             # teal, as a hex color
  classic {
    ring 96
    arm 60 speed 3
    arm 28.8 speed -5
  }
}

layer {
  pen x008080 1.5
  toy ring 96 wheel 36 hole 28.8
}

# A Coding Train style fractal: 7 arms
layer {
  pen red 0.5
  fractal {
    length 60                 # the first arm...
    speed -3
    length_multiplier 1/3     # ...each next one a third as long...
    speed_multiplier -4       # ...and 4x faster, reversed
    count 7
  }
}")
        .unwrap();
        let mut parser = Parser::new(&tokens);
        let doc = parser.document().unwrap();
        assert_eq!(doc.layers.len(), 3);
    }

    #[test]
    fn test_parse_source_toy_matches_builder() {
        let doc = parse_source("layer { pen black 3 toy ring 96 wheel 36 hole 28.8 }").unwrap();
        assert_eq!(doc, _toy(96, 36, 28.8));
    }

    #[test]
    fn test_parse_source_classic_matches_builder() {
        let doc = parse_source(
            "background white
             layer {
               pen black 3
               classic { ring 96 arm 60 speed 3 arm 28.8 speed -5 }
             }",
        )
        .unwrap();
        assert_eq!(doc, _classic());
    }

    #[test]
    fn test_parse_source_default_background_is_white() {
        let doc = parse_source("").unwrap();
        assert_eq!(doc.background_color, macroquad::prelude::WHITE);
        assert!(doc.layers.is_empty());
    }

    #[test]
    fn test_parse_source_leftover_token() {
        let error =
            parse_source("layer { pen red 1 toy ring 96 wheel 36 hole 28.8 } }").unwrap_err();
        assert_eq!(
            error.to_string(),
            "line 1, column 52: expected word 'layer', found '}'"
        );
    }

    #[test]
    fn test_parse_source_lex_error() {
        let error = parse_source("layer @").unwrap_err();
        assert_eq!(
            error.to_string(),
            "line 1, column 7: unexpected character '@'"
        );
    }

    #[test]
    fn test_parse_source_unknown_shape() {
        let error = parse_source("layer { pen red 1 circle { } }").unwrap_err();
        assert_eq!(
            error.to_string(),
            "line 1, column 19: expected toy, fractal or classic, found 'circle'"
        );
    }

    #[test]
    fn test_parse_source_ends_too_soon() {
        let error = parse_source("layer { pen red").unwrap_err();
        assert_eq!(
            error.to_string(),
            "the file ended too soon: expected decimal"
        );
    }

    /// Every `.spiro` file in `examples/` must parse and evaluate.
    #[test]
    fn test_example_files_parse() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");
        let mut count = 0;
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|ext| ext == "spiro") {
                let source = std::fs::read_to_string(&path).unwrap();
                let doc =
                    parse_source(&source).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
                assert!(!doc.layers.is_empty(), "{}: no layers", path.display());
                doc.spirographs()
                    .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
                count += 1;
            }
        }
        assert!(count > 0, "no example files found");
    }
}

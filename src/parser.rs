use crate::{bytes::BorrowedBytes, stream::{OwnedStream, Stream}, util::{
        constants::{punctuators::*, *},
        is_numeric, is_numeric_or_decimal_point, is_numeric_or_negative,
    }};
use std::collections::HashMap;
pub enum ParseError {

}
#[derive(Debug, Clone)]
pub enum LexedSymbol<'a> {
    Punctuator(u8),
    String(BorrowedBytes<'a>),
    Number(f64),
    Boolean(bool),
    Null
}

pub enum Value<'a> {
    String(BorrowedBytes<'a>),
    Number(f64),
    Boolean(bool),
    Object(HashMap<BorrowedBytes<'a>, Value<'a>>),
    Array(Vec<Value<'a>>),
    Null,
    /// This value is only used in case the input lexed JSON is completely empty.
    Nothing
}

pub type LexedJson<'a> = Vec<LexedSymbol<'a>>;

struct Parser<'a> {
    stream: OwnedStream<LexedSymbol<'a>>,
}
impl<'a> Parser<'a> {
    pub fn new(input: LexedJson<'a>) -> Parser {
        Parser {
            stream: OwnedStream::new(input),
        }
    }

    pub fn parse(&mut self) -> Result<Value, String> {
        if self.stream.is_eof() { return Ok(Value::Nothing) };

        let initial = self.stream.current().unwrap();

        match initial {
            LexedSymbol::Boolean(b) => Ok(Value::Boolean(*b)),
            LexedSymbol::Null => Ok(Value::Null),
            LexedSymbol::Number(n) => Ok(Value::Number(*n)),
            LexedSymbol::String(_) => {
                let sym = self.stream.current_owned_unchecked();
                match sym {
                    LexedSymbol::String(s) => Ok(Value::String(s)),
                    _ => unreachable!()
                }
            },
            LexedSymbol::Punctuator(p) => { let x = *p; self.parse_from_punctuator(x) }
        }
    }

    fn parse_from_punctuator(&mut self, punctuator: u8) -> Result<Value, String> {
        match punctuator {
            ARRAY_OPEN => {

            },
            OBJECT_OPEN => {},
            _ => { return Err(format!("unexpected symbol {} at position {}", punctuator as char, self.stream.position()))}
        }

        todo!()
    }

    fn parse_array(&mut self) -> Result<Value, String> {
        // at this point the stream is pointing at the opening punctuator for the array.
        let mut inner: Vec<Value> = vec![];

        while !self.stream.is_eof() {
            let next = self.stream.next_owned_unchecked();
            match next {
                LexedSymbol::Punctuator(p) => {

                },
                _ => {}//inner.push(self.parse()?)
            }
        }

        todo!()
    }

}
pub struct Lexer<'a> {
    stream: Stream<'a, u8>,
}
impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            stream: Stream::new(input.as_bytes()),
        }
    }

    pub fn as_lexed(&mut self) -> Result<LexedJson<'a>, String> {
        self.lex_expression()
    }

    fn lex_expression(&mut self) -> Result<LexedJson<'a>, String> {
        let mut tokens: LexedJson = vec![];

        loop {
            self.skip_whitespace();
            if self.stream.is_eof() {
                return Ok(tokens);
            };
            let initial_character = self.stream.current_unchecked();
            match initial_character {
                ARRAY_OPEN | ARRAY_CLOSE | ARRAY_DELIMITER | OBJECT_OPEN | OBJECT_CLOSE
                | OBJECT_KV_DELIMITER => tokens.push(LexedSymbol::Punctuator(initial_character)),
                STRING_BOUNDARY => tokens.push(self.parse_string()?),
                _ => {
                    if is_numeric_or_negative(initial_character) {
                        tokens.push(self.parse_number()?);
                        continue;
                    }

                    let next_4 = self.stream.slice_len(self.stream.position(), 4);
                    if next_4.eq(NULL) {
                        tokens.push(LexedSymbol::Null);
                        self.stream.skip_n(4);
                        continue;
                    } else if next_4.eq(TRUE) {
                        tokens.push(LexedSymbol::Boolean(true));
                        self.stream.skip_n(4);
                        continue;
                    } else if self.stream.slice_len(self.stream.position(), FALSE.len()).eq(FALSE) {
                        tokens.push(LexedSymbol::Boolean(false));
                        self.stream.skip_n(5);
                        continue;
                    }

                    return Err(format!(
                        "unexpected symbol {} at position {}",
                        String::from_utf8_lossy(&[initial_character]),
                        self.stream.position()
                    ));
                }
            }
            self.stream.skip();
        }
    }

    fn parse_number(&mut self) -> Result<LexedSymbol<'a>, String> {
        let start = self.stream.position();

        while !self.stream.is_eof() {
            let next_char = *self.stream.next_unchecked();

            if !is_numeric_or_decimal_point(next_char) || self.stream.peek().is_none() {
                return Ok(LexedSymbol::Number(
                    std::str::from_utf8(self.stream.slice_unchecked(start, self.stream.position()))
                        .ok()
                        .unwrap()
                        .parse::<f64>()
                        .ok()
                        .unwrap(),
                ));
            }
        }

        Err("number parse failure".to_owned())
    }

    fn parse_string(&mut self) -> Result<LexedSymbol<'a>, String> {
        let start = self.stream.position() + 1;

        while !self.stream.is_eof() {
            let next_char = *self.stream.next_unchecked();

            if next_char == ESCAPE {
                self.stream.skip();
            } else if next_char == STRING_BOUNDARY {
                return Ok(LexedSymbol::String(BorrowedBytes::from(
                    self.stream.slice_unchecked(start, self.stream.position()),
                )));
            }
        }

        Err("Unexpected end of input".to_owned())
    }

    fn skip_whitespace(&mut self) {
        while !self.stream.is_eof() {
            let character = self.stream.current_unchecked();
            if !WHITESPACE.contains(&character) {
                break;
            }
            self.stream.skip();
        }
    }
}

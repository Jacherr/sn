use crate::{
    bytes::BorrowedBytes,
    stream::{OwnedStream, Stream},
    util::{
        constants::{punctuators::*, *},
        is_numeric, is_numeric_or_decimal_point, is_numeric_or_negative,
    },
};
use std::collections::HashMap;
#[derive(Debug)]
pub enum ParseError<'a> {
    UnexpectedSymbol(u8),
    UnexpectedValue(LexedSymbol<'a>),
    UnexpectedEndOfInput,
    ObjectDuplicateKey(BorrowedBytes<'a>)
}
#[derive(Debug, Clone, PartialEq)]
pub enum LexedSymbol<'a> {
    Punctuator(u8),
    String(BorrowedBytes<'a>),
    Number(f64),
    Boolean(bool),
    Null,
}

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    String(BorrowedBytes<'a>),
    Number(f64),
    Boolean(bool),
    Object(HashMap<BorrowedBytes<'a>, Value<'a>>),
    Array(Vec<Value<'a>>),
    Null,
    /// This value is used if the input is empty,
    /// and also internally for parsing arrays and objects to signal that no
    /// next entry exists.
    Nothing,
}

pub type LexedJson<'a> = Vec<LexedSymbol<'a>>;

pub struct Parser<'a> {
    stream: OwnedStream<LexedSymbol<'a>>,
}
impl<'a> Parser<'a> {
    pub fn new(input: LexedJson<'a>) -> Parser {
        Parser {
            stream: OwnedStream::new(input),
        }
    }

    pub fn parse(&mut self) -> Result<Value<'a>, ParseError<'a>> {
        if self.stream.is_eof() {
            return Ok(Value::Nothing);
        };

        let initial = self.stream.current().unwrap();

        match initial {
            LexedSymbol::Boolean(b) => Ok(Value::Boolean(*b)),
            LexedSymbol::Null => Ok(Value::Null),
            LexedSymbol::Number(n) => Ok(Value::Number(*n)),
            LexedSymbol::String(_) => {
                let sym = self.stream.current_owned_unchecked();
                match sym {
                    LexedSymbol::String(s) => Ok(Value::String(s)),
                    _ => unreachable!(),
                }
            }
            LexedSymbol::Punctuator(p) => {
                let x = *p;
                self.parse_from_punctuator(x)
            }
        }
    }

    fn parse_from_punctuator(&mut self, punctuator: u8) -> Result<Value<'a>, ParseError<'a>> {
        match punctuator {
            ARRAY_OPEN => { self.parse_array() }
            OBJECT_OPEN => { self.parse_object() }
            _ => return Err(ParseError::UnexpectedSymbol(punctuator)),
        }
    }

    fn parse_array(&mut self) -> Result<Value<'a>, ParseError<'a>> {
        // at this point the stream is pointing at the opening punctuator for the array.
        let mut inner: Vec<Value> = vec![];
        let mut has_read_initial= false;

        while !self.stream.is_eof() {
            let next = self.stream.next_owned_unchecked();

            match next {
                LexedSymbol::Punctuator(p) => {
                    let parsed = self.parse_array_punctuator(p)?;
                    if parsed == Value::Nothing {
                        break;
                    } else {
                        inner.push(parsed);
                    }
                }
                _ => {
                    // this guard exists to allow the first element to not be delimited (a.k.a [1])
                    // but disallows subsequent elements from not being delimited (a.k.a [1 1])
                    if !has_read_initial {
                        inner.push(self.parse()?);
                        has_read_initial = true;
                    } else {
                        return Err(ParseError::UnexpectedValue(next));
                    }
                }
            }
        }

        Ok(Value::Array(inner))
    }

    fn parse_array_punctuator(
        &mut self,
        punctuator: u8,
    ) -> Result<Value<'a>, ParseError<'a>> {
        match punctuator {
            ARRAY_CLOSE => Ok(Value::Nothing),
            ARRAY_DELIMITER => {
                // we're on the delimiter, must skip past it to get to the expression to parse
                self.stream.skip();
                if self.stream.is_eof() { return Err(ParseError::UnexpectedEndOfInput) }
                Ok(self.parse()?)
            }
            _ => return self.parse_from_punctuator(punctuator),
        }
    }

    fn parse_object(&mut self) -> Result<Value<'a>, ParseError<'a>> {
        let mut inner: HashMap<BorrowedBytes<'a>, Value<'a>> = HashMap::new();

        let mut is_first_entry = true;

        while !self.stream.is_eof() {
            // the value read here is always a key or object closed (if empty object) in valid json
            let try_key = self.stream.next_owned_unchecked();
            let key: BorrowedBytes<'a>;

            // checking that the key is a string
            match try_key {
                LexedSymbol::String(s) => key = s,
                LexedSymbol::Punctuator(p) => {
                    // this checks if the object is empty, by seeing if the punctuator is an object close, but this is only valid for
                    // the first iteration - subsequent checks for object close happen later in this loop
                    if p == OBJECT_CLOSE && is_first_entry {
                        break;
                    }
                    println!("d");
                    return Err(ParseError::UnexpectedSymbol(p))
                },
                _ => return Err(ParseError::UnexpectedValue(try_key))
            }

            // check if key is already used - not valid in json
            if inner.contains_key(&key) {
                return Err(ParseError::ObjectDuplicateKey(key));
            };

            // in valid json this should always be a divider between key and value, so just check it is and then move on
            Parser::validate_punctual_equality(self.stream.next_owned(), OBJECT_KV_DELIMITER)?;

            // next entry in the data should be the value itself, but this can be any type so we will just parse it
            // we are still on the divider at this stage so we will skip to the start of the value
            self.stream.skip();
            if self.stream.is_eof() { return Err(ParseError::UnexpectedEndOfInput) }
            let value = self.parse()?;

            inner.insert(key, value);

            // next thing in the object could either be a delimiter between entries or a closing character
            // delimiter is not valid if there are no more items, so we need to check for this
            let try_next_punctual = self.stream.next_owned();
            let next_punctual;

            match try_next_punctual {
                Some(p) => next_punctual = p,
                None => return Err(ParseError::UnexpectedEndOfInput)
            };

            let next_is_delim = Parser::validate_punctual_equality(Some(next_punctual.clone()), OBJECT_ENTRY_DELIMITER).is_ok();

            if next_is_delim {
                // all this is doing is checking if the thing after the delim
                // is a string (also need to check that it exists, could be EOF)
                let after_delim = self.stream.peek_owned();
                if let Some(after) = after_delim {
                    match after {
                        LexedSymbol::String(_) => {},
                        LexedSymbol::Punctuator(p) => return Err(ParseError::UnexpectedSymbol(p)),
                        _ => return Err(ParseError::UnexpectedValue(after))
                    }
                } else {
                    return Err(ParseError::UnexpectedEndOfInput)
                }
            } else {
                // finish parsing if the array is closed now
                let next_is_close = Parser::validate_punctual_equality(Some(next_punctual.clone()), OBJECT_CLOSE).is_ok();
                if next_is_close { break; };

                // anything past this point is a guaranteed error, so we just
                // choose the correct error to use based on what the current
                // value is...
                return match next_punctual {
                    LexedSymbol::Punctuator(p) => Err(ParseError::UnexpectedSymbol(p)),
                    _ => Err(ParseError::UnexpectedValue(next_punctual))
                }
            }

            is_first_entry = false;
        }

        Ok(Value::Object(inner))
    }

    fn validate_punctual_equality(actual: Option<LexedSymbol<'a>>, expected: u8) -> Result<(), ParseError<'a>> {
        if let Some(sym) = actual {
            match sym {
                LexedSymbol::Punctuator(p) => {
                    if p != expected {
                        Err(ParseError::UnexpectedSymbol(p))
                    } else {
                        Ok(())
                    }
                },
                _ => Err(ParseError::UnexpectedValue(sym))
            }
        }
        else { Err(ParseError::UnexpectedEndOfInput) }
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
                    } else if self
                        .stream
                        .slice_len(self.stream.position(), FALSE.len())
                        .eq(FALSE)
                    {
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

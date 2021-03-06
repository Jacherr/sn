use crate::{
    bytes::Bytes,
    stream::Stream,
    util::{
        constants::{punctuators::*, *},
        is_numeric_like, is_numeric_or_negative,
    },
};
use std::collections::HashMap;
#[derive(Debug)]
/// An error returned by the parser
/// in the event that the input JSON is malformed.
pub enum ParseError {
    /// The parser encountered a symbol (character) in a place it wasn't expecting.
    UnexpectedSymbol(char),
    /// The parser reached the end of the input prematurely.
    UnexpectedEndOfInput,
    /// An internal error that gets thrown if a number somehow fails to parse.
    /// If this is returned, please open an issue.
    NumberParseError
}

#[derive(Debug, PartialEq)]
/// A value as represented in parsed JSON.
pub enum Value<'a> {
    /// A string, composed of bytes borrowed from the input.
    String(Bytes<'a>),
    /// A 64-bit precision floating point number.
    Number(f64),
    /// A boolean.
    Boolean(bool),
    /// An object, represented as a HashMap of a String to a Value.
    Object(HashMap<Bytes<'a>, Value<'a>>),
    /// An array, represented as a Vec of Values.
    Array(Vec<Value<'a>>),
    /// Null (No value).
    Null,
    /// The supplied JSON is completely empty.
    Nothing,
}

/// The parser itself. Create a new parser with the `new` method,
/// and parse it using the `parse` method.
pub struct Parser<'a> {
    stream: Stream<'a, u8>,
}
impl<'a> Parser<'a> {
    /// Create a new parser from raw JSON, either as a &str or as a &[u8].
    pub fn new<T: AsBytes + ?Sized>(input: &'a T) -> Parser {
        Parser {
            stream: Stream::new(input.bytes()),
        }
    }

    /// Parse a single Value from the start of the input.
    pub fn parse(&mut self) -> Result<Value<'a>, ParseError> {
        self.stream.reset();
        self.parse_val()         
    }

    fn parse_val(&mut self) -> Result<Value<'a>, ParseError> {
        self.skip_whitespace_no_eof()?;

        let initial = self.stream.current_unchecked();

        match initial {
            FALSE_IDENT | TRUE_IDENT => self.parse_boolean(),
            NULL_IDENT => self.parse_null(),
            STRING_BOUNDARY => Ok(Value::String(self.parse_string()?)),
            OBJECT_OPEN | ARRAY_OPEN => self.parse_from_punctuator(initial),
            _ => {
                if is_numeric_or_negative(initial) {
                    self.parse_number()
                } else {
                    Err(ParseError::UnexpectedSymbol(initial as char))
                }
            }
        }
    }

    fn parse_from_punctuator(&mut self, punctuator: u8) -> Result<Value<'a>, ParseError> {
        match punctuator {
            ARRAY_OPEN => self.parse_array(),
            OBJECT_OPEN => self.parse_object(),
            _ => Err(ParseError::UnexpectedSymbol(punctuator as char)),
        }
    }

    fn parse_array(&mut self) -> Result<Value<'a>, ParseError> {
        // at this point the stream is pointing at the opening punctuator for the array.
        let mut inner: Vec<Value> = vec![];
        let mut has_read_initial = false;

        while !self.stream.is_eof() {
            self.stream.skip();
            self.skip_whitespace_no_eof()?;

            let next = self.stream.current_unchecked();

            match next {
                ARRAY_DELIMITER => {
                    let parsed = self.parse_array_punctuator(next)?;
                    inner.push(parsed);
                }
                ARRAY_CLOSE => {
                    break;
                }
                _ => {
                    // this guard exists to allow the first element to not be delimited (a.k.a [1])
                    // but disallows subsequent elements from not being delimited (a.k.a [1 1])
                    if !has_read_initial {
                        inner.push(self.parse_val()?);
                        has_read_initial = true;
                    } else {
                        return Err(ParseError::UnexpectedSymbol(next as char));
                    }
                }
            }
        }

        Ok(Value::Array(inner))
    }

    fn parse_array_punctuator(&mut self, punctuator: u8) -> Result<Value<'a>, ParseError> {
        match punctuator {
            ARRAY_CLOSE => Ok(Value::Nothing),
            ARRAY_DELIMITER => {
                // we're on the delimiter, must skip past it to get to the expression to parse
                self.stream.skip();
                if self.stream.is_eof() {
                    return Err(ParseError::UnexpectedEndOfInput);
                }
                Ok(self.parse_val()?)
            }
            _ => self.parse_from_punctuator(punctuator),
        }
    }

    fn parse_boolean(&mut self) -> Result<Value<'a>, ParseError> {
        let next_4 = self.stream.slice_len(self.stream.position(), 4);
        if next_4.eq(TRUE) {
            self.stream.skip_n(3);
            Ok(Value::Boolean(true))
        } else if self.stream.slice_len(self.stream.position(), 5).eq(FALSE) {
            self.stream.skip_n(4);
            Ok(Value::Boolean(false))
        } else {
            Err(ParseError::UnexpectedSymbol(
                self.stream.current_unchecked() as char,
            ))
        }
    }

    fn parse_null(&mut self) -> Result<Value<'a>, ParseError> {
        let next_4 = self.stream.slice_len(self.stream.position(), 4);
        if next_4.eq(NULL) {
            self.stream.skip_n(3);
            Ok(Value::Null)
        } else {
            Err(ParseError::UnexpectedSymbol(
                self.stream.current_unchecked() as char,
            ))
        }
    }

    fn parse_number(&mut self) -> Result<Value<'a>, ParseError> {
        let start = self.stream.position();
        let mut is_first_iteration = true;
        let mut previous_was_exponent = false;
        let mut have_seen_exponent = false;
        self.skip_whitespace_no_eof()?;

        while !self.stream.is_eof() {
            let next_char = self.stream.current_unchecked();

            if next_char == NEGATIVE {
                if is_first_iteration || previous_was_exponent {
                    self.stream.skip();
                    continue;
                } else {
                    return Err(ParseError::UnexpectedSymbol(next_char as char))
                }
            } else if next_char == EXPONENT {
                if is_first_iteration || have_seen_exponent { return Err(ParseError::UnexpectedSymbol(next_char as char)) };
                previous_was_exponent = true;
                have_seen_exponent = true;
            } else if next_char == POSITIVE {
                if !previous_was_exponent {
                    return Err(ParseError::UnexpectedSymbol(next_char as char))
                }
            }

            if next_char != EXPONENT { previous_was_exponent = false; }

            if !is_numeric_like(next_char) || self.stream.peek().is_none() {
                let res = Ok(Value::Number(
                    std::str::from_utf8(self.stream.slice_unchecked(start, self.stream.position()))
                        .ok()
                        .unwrap()
                        .parse::<f64>()
                        .ok()
                        .unwrap(),
                ));
                self.stream.unskip();
                return res;
            }

            self.stream.skip();
            is_first_iteration = false;
        }

        Err(ParseError::NumberParseError)
    }

    fn parse_object(&mut self) -> Result<Value<'a>, ParseError> {
        let mut inner: HashMap<Bytes<'a>, Value<'a>> = HashMap::new();

        let mut is_first_entry = true;

        while !self.stream.is_eof() {
            self.stream.skip();

            self.skip_whitespace_no_eof()?;

            // the value read here should always be a string boundary
            let mut next = self.stream.current_unchecked();

            let key: Bytes<'a>;

            // checking that the key is a string or if this is an empty object
            match next {
                STRING_BOUNDARY => key = self.parse_string()?,
                OBJECT_CLOSE => {
                    // this check disallows { "key": "value", }, but permits {}
                    // by checking if any entries have been parsed yet
                    // we should never get to this point if the json is
                    // { "key": "value" } because another check for } is made
                    // later in this loop.
                    if !is_first_entry {
                        return Err(ParseError::UnexpectedSymbol(next as char));
                    } else {
                        return Ok(Value::Object(inner));
                    }
                }
                _ => return Err(ParseError::UnexpectedSymbol(next as char)),
            }

            // still on string closing boundary
            self.stream.skip();

            self.skip_whitespace_no_eof()?;

            next = self.stream.current_unchecked();
            if next != OBJECT_KV_DELIMITER {
                return Err(ParseError::UnexpectedSymbol(next as char));
            };

            // next entry in the data should be the value itself, but this can be any type so we will just parse it
            // we are still on the divider at this stage so we will skip to the start of the value
            self.stream.skip();
            self.skip_whitespace_no_eof()?;

            let value = self.parse_val()?;
            inner.insert(key, value);

            self.stream.skip();
            self.skip_whitespace_no_eof()?;

            // next thing in the object could either be a delimiter between entries or a closing character
            // delimiter is not valid if there are no more items, so we need to check for this
            next = self.stream.current_unchecked();

            match next {
                OBJECT_ENTRY_DELIMITER => {
                    is_first_entry = false;
                    continue;
                }
                OBJECT_CLOSE => {
                    break;
                }
                _ => return Err(ParseError::UnexpectedSymbol(next as char)),
            }
        }

        Ok(Value::Object(inner))
    }

    fn parse_string(&mut self) -> Result<Bytes<'a>, ParseError> {
        let start = self.stream.position() + 1;

        while !self.stream.is_eof() {
            let next_char = *self.stream.next_unchecked();

            if next_char == ESCAPE {
                self.stream.skip();
            } else if next_char == STRING_BOUNDARY {
                return Ok(Bytes::from(
                    self.stream.slice_unchecked(start, self.stream.position()),
                ));
            }
        }

        Err(ParseError::UnexpectedEndOfInput)
    }

    /// Skips whitespace and checks if there is anything left.
    fn skip_whitespace_no_eof(&mut self) -> Result<(), ParseError> {
        while !self.stream.is_eof() {
            let character = self.stream.current_unchecked();
            if !WHITESPACE.contains(&character) {
                break;
            }
            self.stream.skip();
        }
        if self.stream.is_eof() {
            Err(ParseError::UnexpectedEndOfInput)
        } else {
            Ok(())
        }
    }
}

/// A parser that converts the input String to a static str, so that the
/// struct may live for as long as is necessary and beyond the lifetime of
/// the input String.
/// This is useful if you intend to store the output this parser in a struct
/// or similar.
pub struct StaticParser {
    parser: Parser<'static>,
    ptr: *mut str
}
impl StaticParser {
    /// Create a new static parser instance.
    pub unsafe fn new(input: String) -> Self {
        let ptr: *mut str = Box::into_raw(input.into_boxed_str());
        let data = &*ptr;
        StaticParser {
            parser: Parser::new(data),
            ptr
        }
    }

    /// Parses a single Value from the start of the input using the inner Parser instance.
    pub fn parse(&mut self) -> Result<Value<'static>, ParseError> {
        self.parser.parse()
    }
}
impl Drop for StaticParser {
    fn drop(&mut self) {
       unsafe { Box::from_raw(self.ptr) };
    }
}

pub trait AsBytes {
    fn bytes<'a>(&'a self) -> &'a [u8];
}

impl AsBytes for str {
    fn bytes<'a>(&'a self) -> &'a [u8] {
        self.as_bytes()
    }
}
impl AsBytes for [u8] {
    fn bytes<'a>(&'a self) -> &'a [u8] {
        self
    }
}
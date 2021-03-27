//! # sn
//!
//! sn is a minimalistic and simple Rust JSON parser.<br>
//! sn operates by borrowing slices of the input string, meaning no data is copied during the parsing process, which helps improve efficiency.
//!
//! ## WIP
//!
//! This library is still in a very early working state and is subject to change a lot as it matures.
//! Use At Your Own Risk.
//!
//! ## Bytes
//! `Bytes` is a struct used internally to represent string slices, encoded as &[u8].
//! The primary use case for this struct is the Debug implementation, which converts the contents
//! to a string.
//!
//! ## Examples
//! Loading a JSON file from the filesystem and parsing it into a Value:
//!
//! ```rust
//! use sn::Parser;
//!
//! let raw_json = r#"{ "a": [1, 2, 3], "b": null }"#;
//! let mut parser = Parser::new(raw_json.as_bytes());
//! let parsed_json = parser.parse();
//!
//! println!("{:?}", parsed_json);
//! ```
//!
//! ## Usage
//!
//! Add `sn` to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! sn = "0.1.2"
//! ```

#![deny(missing_docs)]

mod bytes;
mod parser;
mod stream;
mod util;

use parser::{AsBytes, ParseError, Parser, StaticParser, Value};

/// Parse a JSON input.
/// The input may be represented as a &str or as a &[u8].
/// The output only lives for as long as the input.
pub fn parse<'a, T: AsBytes>(input: &'a T) -> Result<Value<'a>, ParseError> {
    let mut parser = Parser::new(input);
    parser.parse()
}

/// Statically parse a JSON input.
/// The input may be represented as a &str or as a &[u8].
/// This function is marked unsafe because it leaks data and therefore has the potential to segfault.
pub unsafe fn parse_owned(input: String) -> Result<Value<'static>, ParseError> {
    let mut parser = StaticParser::new(input);
    parser.parse()
}
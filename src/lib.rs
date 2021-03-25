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

pub use parser::{ParseError, Parser, Value};
pub use stream::Stream;

# sn

sn is a minimalistic and simple Rust JSON parser.<br>
sn operates by borrowing slices of the input string, meaning no data is copied during the parsing process, which helps improve efficiency.

## WIP

This library is still in a very early working state and is subject to change a lot as it matures.
Use At Your Own Risk.

## Examples
Loading a JSON file from the filesystem and parsing it into a Value:

```rust
use sn::Parser;
use std::fs::read_to_string;

fn main() {
    let raw_json = read_to_string("./my_json.json").unwrap();
    let mut parser = Parser::new(raw_json.as_bytes());
    let parsed_json = parser.parse();

    println!("{:?}", parsed_json);
}
```

## Usage

Add `sn` to your Cargo.toml:

```toml
[dependencies]
sn = "0.1.3"
```
//#![feature(test)]
//extern crate test;

mod bytes;
pub mod parser;
pub mod stream;
mod util;

/*
mod tests {
    use crate::parser::Parser;
    use std::fs::read;

  #[test]
  fn parse() {
    let bytes = read("./test/large.json").unwrap();
    let string = String::from_utf8_lossy(&bytes);
    let mut parser = Parser::new(string.as_bytes());
    let output = parser.parse();
    if output.is_err() { println!("{:?}", output) };
  }
  #[bench]
  fn twitter(b: &mut test::Bencher) {
    let raw = read("./test/twitter.json").unwrap();
    b.iter(|| {
      test::black_box(Parser::new(&raw).parse());
    });
  }
  #[bench]
  fn citm_catalog(b: &mut test::Bencher) {
    let raw = read("./test/citm_catalog.json").unwrap();
    b.iter(|| {
      test::black_box(Parser::new(&raw).parse());
    });
  }
  #[bench]
  fn canada(b: &mut test::Bencher) {
    let raw = read("./test/canada.json").unwrap();
    b.iter(|| {
      test::black_box(Parser::new(&raw).parse());
    });
  }
  #[bench]
  fn large(b: &mut test::Bencher) {
    let raw = read("./test/large.json").unwrap();
    b.iter(|| {
      test::black_box(Parser::new(&raw).parse());
    });
  }
}
*/
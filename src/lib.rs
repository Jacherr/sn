#![feature(test)]
extern crate test;

mod bytes;
mod parser;
mod stream;
mod util;

#[cfg(test)]
mod tests {
    use std::{fs::read};

    use crate::parser::*;
    #[test]
    fn lexer_test() {
        let raw = &read("./test/twitter.json").unwrap();
        let json = String::from_utf8_lossy(raw);
        let mut lexer = Lexer::new(&json);
        let lexed = lexer.as_lexed().unwrap();
        let out = Parser::new(lexed).parse();
        assert_eq!(out.is_ok(), true)
    }
    #[bench]
    fn twitter(b: &mut test::Bencher) {
        let raw = &read("./test/twitter.json").unwrap();
        let json = test::black_box(String::from_utf8_lossy(raw));
        b.iter(|| {
          let mut lexer = Lexer::new(&json);
          let lexed = lexer.as_lexed().unwrap();
          Parser::new(lexed).parse();
        });
    }
    #[bench]
    fn citm_catalog(b: &mut test::Bencher) {
        let raw = &read("./test/citm_catalog.json").unwrap();
        let json = test::black_box(String::from_utf8_lossy(raw));
        b.iter(|| {
          let mut lexer = Lexer::new(&json);
          let lexed = lexer.as_lexed().unwrap();
          Parser::new(lexed).parse();
        });
    }
    #[bench]
    fn canada(b: &mut test::Bencher) {
        let raw = &read("./test/canada.json").unwrap();
        let json = test::black_box(String::from_utf8_lossy(raw));
        b.iter(|| {
          let mut lexer = Lexer::new(&json);
          let lexed = lexer.as_lexed().unwrap();
          Parser::new(lexed).parse();
        });
    }
    #[bench]
    fn large(b: &mut test::Bencher) {
        let raw = &read("./test/large.json").unwrap();
        let json = test::black_box(String::from_utf8_lossy(raw));
        b.iter(|| {
          let mut lexer = Lexer::new(&json);
          let lexed = lexer.as_lexed().unwrap();
          Parser::new(lexed).parse();
        });
    }
}

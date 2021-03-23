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
        let raw = &read("./test/large.json").unwrap();
        let json = String::from_utf8_lossy(raw);
        let mut lexer = Lexer::new(&json);
        let out = lexer.as_lexed();
        if out.is_err() { println!("{:?}", out) };
    }
    #[bench]
    fn twitter(b: &mut test::Bencher) {
        let raw = &read("./test/twitter.json").unwrap();
        let json = test::black_box(String::from_utf8_lossy(raw));
        b.iter(|| {
          let mut lexer = Lexer::new(&json);
          lexer.as_lexed()
        });
    }
    #[bench]
    fn citm_catalog(b: &mut test::Bencher) {
        let raw = &read("./test/citm_catalog.json").unwrap();
        let json = test::black_box(String::from_utf8_lossy(raw));
        b.iter(|| {
          let mut lexer = Lexer::new(&json);
          lexer.as_lexed()
        });
    }
    #[bench]
    fn canada(b: &mut test::Bencher) {
        let raw = &read("./test/canada.json").unwrap();
        let json = test::black_box(String::from_utf8_lossy(raw));
        b.iter(|| {
          let mut lexer = Lexer::new(&json);
          lexer.as_lexed()
        });
    }
    #[bench]
    fn large(b: &mut test::Bencher) {
        let raw = &read("./test/large.json").unwrap();
        let json = test::black_box(String::from_utf8_lossy(raw));
        b.iter(|| {
          let mut lexer = Lexer::new(&json);
          lexer.as_lexed()
        });
    }
}

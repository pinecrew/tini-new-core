#![allow(unused_variables, dead_code)]
use std::iter::Peekable;
use std::str::Chars;

pub struct StringReader<'a> {
    src: Peekable<Chars<'a>>,
}

impl<'a> StringReader<'a> {
    pub fn new(src: &'a str) -> StringReader {
        StringReader { src: src.chars().peekable() }
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.src.peek()
    }

    pub fn take(&mut self) -> Option<char> {
        self.src.next()
    }

    pub fn drop(&mut self) {
        let _ = self.take();
    }

    pub fn take_while<F>(&mut self, f: F) -> Option<String>
    where
        F: Fn(&char) -> bool,
    {
        let mut buffer = Vec::new();
        while let Some(value) = self.src.peek() {
            if !f(value) {
                break;
            }
            let item = self.src.next().unwrap();
            buffer.push(item);
        }
        Some(buffer.iter().collect::<String>())
    }

    pub fn drop_while<F>(&mut self, f: F) -> usize
    where
        F: Fn(&char) -> bool,
    {
        let mut count = 0;
        while let Some(value) = self.src.peek() {
            if !f(value) {
                break;
            }
            let _ = self.src.next();
            count += 1;
        }
        count
    }
}

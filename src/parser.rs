#![allow(unused_variables, dead_code)]
use crate::reader::StringReader;

pub struct Parser<'a> {
    reader: StringReader<'a>,
    comment: char,
    line: usize,
    pos: usize,
}

#[derive(Debug)]
pub enum ParseResult {
    Token(Token),
    Error { line: usize, pos: usize },
}

#[derive(Debug)]
pub enum Token {
    EndOfLine,
    Comment { text: String },
    Header { name: String },
    KeyValue { key: String, value: String },
    ListHeader { name: String },
    ListValue { value: String },
}

impl<'a> Parser<'a> {
    pub fn new(comment: char, data: &'a str) -> Parser<'a> {
        Parser { comment, reader: StringReader::new(data), line: 0, pos: 0 }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = ParseResult;

    fn next(&mut self) -> Option<Self::Item> {
        // получаем первый токен
        let chr = self.reader.peek();

        if chr.is_none() {
            return None;
        }

        let chr = chr.unwrap();

        // токен начинается с комментария
        if chr == &self.comment {
            // поглащяем символ комментария
            self.reader.drop();
            // читаем до конца строки
            let text = self.reader.take_while(|&c| c != '\n').unwrap();
            Some(ParseResult::Token(Token::Comment { text }))
        // токен начинается со перевода строки
        } else if chr == &'\n' {
            // поглащяем токен
            self.reader.drop();
            Some(ParseResult::Token(Token::EndOfLine))
        // токен начинается с определения секции
        } else if chr == &'[' {
            // поглащаем
            self.reader.drop();
            // собираем имя
            let name = self.reader.take_while(|&c| c != ']').unwrap();
            // ещё поглащяем
            self.reader.drop();
            Some(ParseResult::Token(Token::Header { name }))
        // токен это пробел или табуляция
        } else if chr.is_whitespace() {
            // поглащаем
            self.reader.drop_while(|&c| c.is_whitespace());
            self.next()
        // токен это ключ
        } else if chr.is_alphanumeric() {
            let key = self.reader.take_while(|&c| c != '=').unwrap();
            self.reader.drop();
            let value = self.reader.take_while(|&c| c != '\n' && c != self.comment).unwrap();
            Some(ParseResult::Token(Token::KeyValue { key, value }))
        } else {
            None
        }
    }
}

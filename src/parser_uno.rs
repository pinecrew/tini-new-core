#![allow(unused_variables, dead_code)]
use crate::reader::StringReader;

pub struct Parser<'a> {
    reader: StringReader<'a>,
    comment: char,
    line: usize,
    pos: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseResult {
    Token(Token),
    Error { line: usize, pos: usize },
}

#[derive(Debug, Eq, PartialEq)]
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
        Parser { comment, reader: StringReader::new(data), line: 1, pos: 1 }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = ParseResult;

    fn next(&mut self) -> Option<Self::Item> {
        let chr = self.reader.peek()?;

        // токен начинается с комментария
        if chr == &self.comment {
            // поглащяем символ комментария
            self.reader.drop();
            // читаем до конца строки
            let text = self.reader.take_while(|&c| c != '\n');

            self.pos += text.len() + 1;

            Some(ParseResult::Token(Token::Comment { text }))
        // токен начинается со перевода строки
        } else if chr == &'\n' {
            // поглащяем \n
            self.reader.drop();

            self.line += 1;
            self.pos = 1;

            Some(ParseResult::Token(Token::EndOfLine))
        // токен начинается с определения секции
        } else if chr == &'[' {
            // поглащаем [
            self.reader.drop();
            // собираем имя
            let name = self.reader.take_while(|&c| c != ']');
            // поглащяем ]
            self.reader.drop();

            self.pos += name.len() + 2;

            Some(ParseResult::Token(Token::Header { name }))
        // токен это пробел или табуляция
        } else if chr.is_whitespace() {
            // поглащаем символы
            let count = self.reader.drop_while(|&c| c.is_whitespace());

            self.pos += count;

            // и рекурсивно возвращаем токен
            self.next()
        // токен это ключ
        } else if chr.is_alphanumeric() {
            // получаем ключ
            let key = self.reader.take_while(|&c| c != '=').trim().to_string();
            // поглощаем =
            self.reader.drop();
            // дропаем whitespace
            let skip = self.reader.drop_while(|&c| c.is_whitespace());
            // получаем значение
            let value = self.reader.take_while(|&c| c != '\n' && c != self.comment).trim().to_string();

            self.pos += key.len() + value.len() + 1 + skip;

            Some(ParseResult::Token(Token::KeyValue { key, value }))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser_uno::{ParseResult, Parser, Token};

    #[test]
    fn one_line_comment() {
        let parser = Parser::new(';', "; hello\n\n");
        let expected = vec![
            ParseResult::Token(Token::Comment { text: " hello".to_string() }),
            ParseResult::Token(Token::EndOfLine),
            ParseResult::Token(Token::EndOfLine),
        ];
        let parsed: Vec<ParseResult> = parser.into_iter().collect();
        assert_eq!(expected, parsed);
    }

    #[test]
    fn section_with_one_line_comment() {
        let parser = Parser::new(';', "\t[section]; comment text");
        let expected = vec![
            ParseResult::Token(Token::Header { name: "section".to_string() }),
            ParseResult::Token(Token::Comment { text: " comment text".to_string() }),
        ];
        let parsed: Vec<ParseResult> = parser.into_iter().collect();
        assert_eq!(expected, parsed);
    }

    #[test]
    fn section_with_next_line_comment() {
        let parser = Parser::new('#', "[section]\n# comment text");
        let expected = vec![
            ParseResult::Token(Token::Header { name: "section".to_string() }),
            ParseResult::Token(Token::EndOfLine),
            ParseResult::Token(Token::Comment { text: " comment text".to_string() }),
        ];
        let parsed: Vec<ParseResult> = parser.into_iter().collect();
        assert_eq!(expected, parsed);
    }

    #[test]
    fn key_value_with_one_line_comment() {
        let parser = Parser::new(';', "key = value ; this is key with value");
        let expected = vec![
            ParseResult::Token(Token::KeyValue { key: "key".to_string(), value: "value".to_string() }),
            ParseResult::Token(Token::Comment { text: " this is key with value".to_string() }),
        ];
        let parsed: Vec<ParseResult> = parser.into_iter().collect();
        assert_eq!(expected, parsed);
    }

    #[test]
    fn all_in_one_ini() {
        let ini_config = r#"; DO NOT EDIT
; game configuration file
[graphics] ; graphics options
screen_size = 1280x720 ; fmt: WxH
monitor = 0 # not comment

[game]
; i know, you don't like it
skip_cutscene = true"#;
        let parser = Parser::new(';', ini_config);
        let expected = vec![
            ParseResult::Token(Token::Comment { text: " DO NOT EDIT".to_string() }),
            ParseResult::Token(Token::EndOfLine),
            ParseResult::Token(Token::Comment { text: " game configuration file".to_string() }),
            ParseResult::Token(Token::EndOfLine),
            ParseResult::Token(Token::Header { name: "graphics".to_string() }),
            ParseResult::Token(Token::Comment { text: " graphics options".to_string() }),
            ParseResult::Token(Token::EndOfLine),
            ParseResult::Token(Token::KeyValue { key: "screen_size".to_string(), value: "1280x720".to_string() }),
            ParseResult::Token(Token::Comment { text: " fmt: WxH".to_string() }),
            ParseResult::Token(Token::EndOfLine),
            ParseResult::Token(Token::KeyValue { key: "monitor".to_string(), value: "0 # not comment".to_string() }),
            ParseResult::Token(Token::EndOfLine),
            ParseResult::Token(Token::EndOfLine),
            ParseResult::Token(Token::Header { name: "game".to_string() }),
            ParseResult::Token(Token::EndOfLine),
            ParseResult::Token(Token::Comment { text: " i know, you don't like it".to_string() }),
            ParseResult::Token(Token::EndOfLine),
            ParseResult::Token(Token::KeyValue { key: "skip_cutscene".to_string(), value: "true".to_string() }),
        ];
        let parsed: Vec<ParseResult> = parser.into_iter().collect();
        assert_eq!(expected, parsed);
    }
}

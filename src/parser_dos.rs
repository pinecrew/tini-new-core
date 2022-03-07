#![allow(unused_variables, dead_code)]
use crate::reader::StringReader;

#[derive(Debug, Eq, PartialEq)]
pub enum Lexeme {
    // \n
    EndOfLine,
    // =[]
    Separator(char),
    // \t | \s
    Whitespace(String),
    // ; (.*)$
    Commect(String),
    // is_token
    Token(String),
}

pub fn is_token(character: char, ignore: char) -> bool {
    character.is_alphanumeric() || character.is_ascii_punctuation() && character != ignore
}

pub fn parse(data: &str, comment: char) -> Vec<Lexeme> {
    let mut reader = StringReader::new(data);
    let mut result = Vec::new();
    let mut line = 1;
    let mut pos = 1;

    while let Some(&chr) = reader.peek() {
        if chr == comment {
            reader.drop();
            let text = reader.take_while(|&c| c != '\n');

            pos += 1 + text.len();

            result.push(Lexeme::Commect(text));
        } else if chr == '\n' {
            reader.drop();
            result.push(Lexeme::EndOfLine);

            line += 1;
            pos = 1;
        } else if chr == '[' {
            reader.drop();
            result.push(Lexeme::Separator('['));
            let text = reader.take_while(|&c| c != ']');

            pos += 2 + text.len();

            result.push(Lexeme::Token(text));
            reader.drop();
            result.push(Lexeme::Separator(']'));
        } else if chr == '=' {
            reader.drop();
            result.push(Lexeme::Separator('='));

            pos += 1;
        } else if chr.is_whitespace() {
            let text = reader.take_while(|&c| c.is_whitespace());

            pos += text.len();

            result.push(Lexeme::Whitespace(text));
        } else if is_token(chr, comment) {
            let text = reader.take_while(|&c| is_token(c, comment));

            pos += text.len();

            result.push(Lexeme::Token(text));
        } else {
            panic!("unknown token `{chr}` at {line}:{pos}");
        }
    }

    result
}

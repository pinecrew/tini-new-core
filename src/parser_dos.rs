#![allow(unused_variables, dead_code)]
use crate::reader::StringReader;

#[derive(Debug, Eq, PartialEq)]
pub enum Lexeme {
    // \n | \r\n
    EndOfLine,
    // =[]
    Separator(char),
    // \t | \s
    Whitespace(String),
    // ; (.*)$
    Comment(String),
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
            let text = reader.take_while(|&c| c != '\n' && c != '\r');

            pos += 1 + text.len();

            result.push(Lexeme::Comment(text));
        } else if chr == '\n' {
            reader.drop();
            result.push(Lexeme::EndOfLine);

            line += 1;
            pos = 1;
        } else if chr == '\r' {
            reader.drop();
            pos += 1;
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

#[cfg(test)]
mod test {
    use crate::parser_dos::{parse, Lexeme};

    #[test]
    fn one_line_comment() {
        let parsed = parse("; hello\n\n", ';');
        let expected = vec![
            Lexeme::Comment(" hello".to_string()),
            Lexeme::EndOfLine,
            Lexeme::EndOfLine
        ];
        assert_eq!(expected, parsed)
    }

    #[test]
    fn section_with_one_line_comment() {
        let parsed = parse("\t[section]; comment text", ';');
        let expected = vec![
            Lexeme::Whitespace("\t".to_string()),
            Lexeme::Separator('['),
            Lexeme::Token("section".to_string()),
            Lexeme::Separator(']'),
            Lexeme::Comment(" comment text".to_string())
        ];
        assert_eq!(expected, parsed);
    }

    #[test]
    fn section_with_next_line_comment() {
        let parsed = parse("[section]\n# comment text\n", '#');
        let expected = vec![
            Lexeme::Separator('['),
            Lexeme::Token("section".to_string()),
            Lexeme::Separator(']'),
            Lexeme::EndOfLine,
            Lexeme::Comment(" comment text".to_string()),
            Lexeme::EndOfLine
        ];
        assert_eq!(expected, parsed);
    }

    #[test]
    fn key_value_with_one_line_comment() {
        let parsed = parse("key = value ; this is key with value", ';');
        let expected = vec![
            Lexeme::Token("key".to_string()),
            Lexeme::Whitespace(" ".to_string()),
            Lexeme::Separator('='),
            Lexeme::Whitespace(" ".to_string()),
            Lexeme::Token("value".to_string()),
            Lexeme::Whitespace(" ".to_string()),
            Lexeme::Comment(" this is key with value".to_string())
        ];
        assert_eq!(expected, parsed);
    }

    #[test]
    fn all_in_one_ini() {
        let ini_config = r#"
; pre section comment
[section] ; side comment
; pre kv comment
param1 = a # b ; side comment"#;

        let parsed = parse(ini_config, ';');
        let expected = vec![
            Lexeme::EndOfLine,
            Lexeme::Comment(" pre section comment".to_string()),
            Lexeme::EndOfLine,
            Lexeme::Separator('['),
            Lexeme::Token("section".to_string()),
            Lexeme::Separator(']'),
            Lexeme::Whitespace(" ".to_string()),
            Lexeme::Comment(" side comment".to_string()),
            Lexeme::EndOfLine,
            Lexeme::Comment(" pre kv comment".to_string()),
            Lexeme::EndOfLine,
            Lexeme::Token("param1".to_string()),
            Lexeme::Whitespace(" ".to_string()),
            Lexeme::Separator('='),
            Lexeme::Whitespace(" ".to_string()),
            Lexeme::Token("a".to_string()),
            Lexeme::Whitespace(" ".to_string()),
            Lexeme::Token("#".to_string()),
            Lexeme::Whitespace(" ".to_string()),
            Lexeme::Token("b".to_string()),
            Lexeme::Whitespace(" ".to_string()),
            Lexeme::Comment(" side comment".to_string()),
        ];

        assert_eq!(expected, parsed);
    }
}

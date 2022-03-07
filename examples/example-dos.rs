extern crate tini;

fn main() {
    let text = include_str!("../examples/example-dos.ini");
    let lexemes = tini::parser_dos::parse(text, ';');
    for lexeme in lexemes {
        println!("{lexeme:?}")
    }
}

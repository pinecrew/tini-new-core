extern crate tini;

fn main() {
    let text = include_str!("../examples/example-uno.ini");
    for item in tini::parser_uno::Parser::new(';', text) {
        println!("{:?}", item);
    }
}

extern crate tini;

fn main() {
    let text = include_str!("../examples/example-1.ini");
    for item in tini::parser::Parser::new(';', text) {
        println!("{:?}", item);
    }
}

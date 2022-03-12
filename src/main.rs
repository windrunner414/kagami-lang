mod ast;
mod base;
mod parser;

use std::env::args;
use std::fs::read_to_string;
use std::io;

fn main() -> io::Result<()> {
    let mut args = args();
    args.next();
    let file = args
        .next()
        .unwrap_or_else(|| "tests/code/1.kgm".to_string());

    let input = read_to_string(file).unwrap();
    let mut errors = parser::Errors::new();

    let kagami_ast = parser::KagamiModuleParser::new()
        .parse(&mut errors, &input)
        .unwrap();

    if errors.is_empty() {
        println!("{:#?}", kagami_ast);
    } else {
        println!("{:#?}", errors);
    }

    Ok(())
}

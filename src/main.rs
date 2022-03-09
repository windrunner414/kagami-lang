pub mod ast;

use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io;

lalrpop_mod!(pub parser, "/parser/kagami.rs");

fn main() -> io::Result<()> {
    let mut args = args();
    args.next();
    let file = args.next().unwrap_or("test/code/1.kgm".to_string());

    let input = read_to_string(file).unwrap();
    let kagami_ast = parser::KagamiModuleParser::new().parse(&input).unwrap();

    println!("{:#?}", kagami_ast);

    Ok(())
}

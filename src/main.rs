use std::env;
use crate::parser::Parser;

pub mod parser;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let mut parser = Parser::new(args);
    parser.parse();
    parser.display_command();
}

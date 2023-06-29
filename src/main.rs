use std::collections::VecDeque;
use std::env;

use crate::lexer::Lexer;
use crate::parser::Parser;

pub mod lexer;
pub mod parser;
pub mod utils;

fn main() {
    let args: VecDeque<String> = env::args().skip(1).collect();
    let mut parser = Lexer::new(args);
    parser.read();
    parser.display_command();

    let mut runner = Parser::new(parser.tokens.clone());
    runner.parse();
}


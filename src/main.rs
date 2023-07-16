use std::collections::VecDeque;
use std::env;

use crate::lexer::Lexer;
use crate::parser::Parser;

pub mod lexer;
pub mod parser;
pub mod utils;

fn main() {
    let args: VecDeque<String> = env::args().skip(1).collect();
    let mut lexer = Lexer::new(args);
    lexer.read();
    
    let mut parser = Parser::new(lexer.tokens.clone());
    parser.parse();
}


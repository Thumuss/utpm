use std::collections::VecDeque;
use std::env;

use crate::parser::Parser;
use crate::runner::Runner;

pub mod parser;
pub mod runner;
pub mod utils;

fn main() {
    let args: VecDeque<String> = env::args().skip(1).collect();
    let mut parser = Parser::new(args);
    parser.parse();
    parser.display_command();

    let mut runner = Runner::new(parser.tokens.clone());
    runner.run();
}


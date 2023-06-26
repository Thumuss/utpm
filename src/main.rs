use std::collections::VecDeque;
use std::env;

use crate::parser::Parser;
use crate::runner::Runner;

pub mod parser;
pub mod runner;
pub mod utils;

fn main() {
    let mut args: VecDeque<String> = env::args().collect();
    args.pop_front();
    let mut parser = Parser::new(args);
    parser.parse();
    parser.display_command();

    let mut runner = Runner::new(parser.tokens.clone());
    runner.run();
}


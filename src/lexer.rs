use std::collections::VecDeque;

#[derive(Clone, PartialEq, Eq)]
pub enum CLIOptions {
    Create,
    Link,
    Unlink,
    List,

    Help,
    Force,
    
    NoCopy,
    NoInteractive,

    Unknown,

    Token(String),
}

pub struct Lexer {
    pub tokens: VecDeque<CLIOptions>,
    args: VecDeque<String>,
}

impl Lexer {
    pub fn new(args: VecDeque<String>) -> Self {
        Self {
            tokens: VecDeque::new(),
            args,
        }
    }

    pub fn read(&mut self) {
        let mut i: usize = 0;
        let mut next: u32 = 0;
        while i < self.args.len() {
            match self.args[i].as_str() {
                "create" | "c" => self.tokens.push_back(CLIOptions::Create),

                "link" | "lk" => self.tokens.push_back(CLIOptions::Link),
                "list" | "ls" => self.tokens.push_back(CLIOptions::List),

                "--help" | "-h" => self.tokens.push_back(CLIOptions::Help),

                "--force" | "-f" => self.tokens.push_back(CLIOptions::Force),

                "--no-copy" | "-nc" => self.tokens.push_back(CLIOptions::NoCopy),

                _ => {
                    if next > 0 {
                        self.tokens
                            .push_back(CLIOptions::Token(self.args[i].clone()));
                        next -= 1;
                    } else {
                        self.tokens.push_back(CLIOptions::Unknown)
                    }
                }
            };
            i += 1;
        }
    }
}

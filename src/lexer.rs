use std::collections::VecDeque;

#[derive(Clone, PartialEq, Eq)]
pub enum CLIOptions {
    Create,
    Link,
    Unlink,

    Help,
    Force,

    Unknown,

    Token(String),
}

impl CLIOptions {
    fn display(&self) -> String {
        match self {
            CLIOptions::Create => String::from("create"),
            CLIOptions::Link => String::from("link"),
            CLIOptions::Unlink => String::from("unlink"),

            CLIOptions::Help => String::from("--help"),
            CLIOptions::Force => String::from("--force"),
            CLIOptions::Unknown => String::from("unknown"),
            CLIOptions::Token(aaa) => aaa.to_string(),
        }
    }
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

    pub fn display_command(&self) {
        for e in &self.tokens {
            print!("{} ", e.display());
        }
        println!();
    }

    pub fn read(&mut self) {
        let mut i: usize = 0;
        let mut next: u32 = 0;
        while i < self.args.len() {
            match self.args[i].as_str() {
                "c" | "create" => {
                    self.tokens.push_back(CLIOptions::Create);
                }
                "link" => {
                    self.tokens.push_back(CLIOptions::Link);
                }
                "--help" | "-h" => {
                    self.tokens.push_back(CLIOptions::Help);
                    return;
                },
                "--force" | "-f" => {
                    self.tokens.push_back(CLIOptions::Force);
                    return;
                },

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

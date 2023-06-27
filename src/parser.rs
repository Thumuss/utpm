use std::collections::VecDeque;

#[derive(Clone, PartialEq, Eq)]
pub enum CLIOptions {
    New, // Fait
    Create,
    Delete,
    Install, // Fait
    Uninstall,
    Run, // Fait

    // Flags
    Help,

    // Rest
    Unknown,

    // Text
    Token(String),
}

impl CLIOptions {
    fn display(&self) -> String {
        match self {
            CLIOptions::New => String::from("new"),
            CLIOptions::Create => String::from("create"),
            CLIOptions::Delete => String::from("delete"),
            CLIOptions::Install => String::from("install"),
            CLIOptions::Uninstall => String::from("uninstall"),
            CLIOptions::Run => String::from("run"),

            CLIOptions::Help => String::from("help"),
            CLIOptions::Unknown => String::from("unknown"),
            CLIOptions::Token(aaa) => aaa.to_string(),
        }
    }
}

pub struct Parser {
    pub tokens: VecDeque<CLIOptions>,
    args: VecDeque<String>,
}

impl Parser {
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

    pub fn parse(&mut self) {
        let mut i: usize = 0;
        let mut next: u32 = 0;
        while i < self.args.len() {
            match self.args[i].as_str() {
                "init" => {
                    self.tokens.push_back(CLIOptions::New);
                }
                "c" | "create" => {
                    self.tokens.push_back(CLIOptions::Create);
                    next += 1;
                }
                "d" | "delete" => {
                    self.tokens.push_back(CLIOptions::Delete);
                    next += 1;
                }
                "install" | "i"| "a" | "add" => {
                    self.tokens.push_back(CLIOptions::Install);
                    next += 1;
                }
                "uninstall" | "uni"| "rm" | "remove" => {
                    self.tokens.push_back(CLIOptions::Uninstall);
                    next += 1;
                }
                "compile" | "c" => {
                    self.tokens.push_back(CLIOptions::Run);
                    next += 1;
                }
                "--help" | "help" | "-h" => {
                    self.tokens.push_back(CLIOptions::Help);
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

use std::collections::VecDeque;

#[derive(Clone, PartialEq, Eq)]
pub enum CLIOptions {
    Init,
    Create,
    Delete,
    Install,
    Uninstall,
    Compile,
    Config,
    Link,
    Unlink,
    Refresh,

    // Flags
    Help,
    Global,

    // Rest
    Unknown,

    // Text
    Token(String),
}

impl CLIOptions {
    fn display(&self) -> String {
        match self {
            CLIOptions::Init => String::from("init"),
            CLIOptions::Refresh => String::from("refresh"),
            CLIOptions::Create => String::from("create"),
            CLIOptions::Delete => String::from("delete"),
            CLIOptions::Install => String::from("install"),
            CLIOptions::Uninstall => String::from("uninstall"),
            CLIOptions::Compile => String::from("run"),
            CLIOptions::Link => String::from("link"),
            CLIOptions::Unlink => String::from("unlink"),
            CLIOptions::Config => String::from("config"),

            CLIOptions::Help => String::from("--help"),
            CLIOptions::Global => String::from("--global"),
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
                "init" => {
                    self.tokens.push_back(CLIOptions::Init);
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
                "compile" => {
                    self.tokens.push_back(CLIOptions::Compile);
                }
                "refresh" => {
                    self.tokens.push_back(CLIOptions::Refresh);
                    next += 1;
                }
                "link" => {
                    self.tokens.push_back(CLIOptions::Link);
                }
                "--help" | "help" | "-h" => {
                    self.tokens.push_back(CLIOptions::Help);
                    return;
                },
                "--global" | "-g" => {
                    self.tokens.push_back(CLIOptions::Global);
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

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

    //For Non-interactive
    Name,
    Entrypoint,
    SelectVersion,

    Author,
    License,
    Description,

    Repository,
    Homepage,
    Keyword,
    Compiler,
    Exclude,



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
                
                // For Non-interactive:

                "--no-interactive" | "-ni" => self.tokens.push_back(CLIOptions::NoInteractive),

                "--name" | "-n" => {
                    self.tokens.push_back(CLIOptions::Name);
                    next += 1;
                }
                "--ver" => {
                    self.tokens.push_back(CLIOptions::SelectVersion);
                    next += 1;
                }
                "--entrypoint" | "-e" => {
                    self.tokens.push_back(CLIOptions::Entrypoint);
                    next += 1;
                }

                "--author" | "-a" => {
                    self.tokens.push_back(CLIOptions::Author);
                    next += 1;
                }

                "--licence" | "-l" => {
                    self.tokens.push_back(CLIOptions::License);
                    next += 1;
                }
                "--description" | "--desc" | "-d" => {
                    self.tokens.push_back(CLIOptions::Description);
                    next += 1;
                }
                "--repository" | "-r" => {
                    self.tokens.push_back(CLIOptions::Repository);
                    next += 1;
                }
                "--homepage" | "-hp" => {
                    self.tokens.push_back(CLIOptions::Homepage);
                    next += 1;
                }
                "--keyword" | "-k" => {
                    self.tokens.push_back(CLIOptions::Keyword);
                    next += 1;
                }
                "--compiler" | "-c" => {
                    self.tokens.push_back(CLIOptions::Compiler);
                    next += 1;
                }
                "--exclude" | "-x" => {
                    self.tokens.push_back(CLIOptions::Exclude);
                    next += 1;
                }



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

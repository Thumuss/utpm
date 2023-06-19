pub enum CLIOptions {
    New,
    Create,
    Delete,
    Add,
    Remove,
    Help,
    Unknown,
    Token(String),
}

impl CLIOptions {
    fn display(&self) -> String {
        match self {
            CLIOptions::New => String::from("new"),
            CLIOptions::Create => String::from("create"),
            CLIOptions::Delete => String::from("delete"),
            CLIOptions::Add => String::from("add"),
            CLIOptions::Remove => String::from("remove"),
            CLIOptions::Help => String::from("help"),
            CLIOptions::Unknown => String::from("unknown"),
            CLIOptions::Token(aaa) => aaa.clone(),
        }
    }
}

pub struct Parser {
    tokens: Vec<CLIOptions>,
    args: Vec<String>,
}

impl Parser {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            tokens: vec![],
            args,
        }
    }

    pub fn display_command(&self) {
        for e in &self.tokens {
            print!("{} ", e.display());
        }
        println!("");
    }

    pub fn parse(&mut self) {
        let mut i = 0;
        while i < self.args.len() {
            match self.args[i].as_str() {
                "n" | "new" | "init" => {
                    self.tokens.push(CLIOptions::New);
                    i += 1;
                    self.tokens.push(CLIOptions::Token(self.args[i].clone()))
                }
                "c" | "create" => {
                    self.tokens.push(CLIOptions::Create);
                    i += 1;
                    self.tokens.push(CLIOptions::Token(self.args[i].clone()))
                }
                "d" | "delete" => {
                    self.tokens.push(CLIOptions::Delete);
                    i += 1;
                    self.tokens.push(CLIOptions::Token(self.args[i].clone()))
                }
                "a" | "add" => {
                    self.tokens.push(CLIOptions::Add);
                    i += 1;
                    self.tokens.push(CLIOptions::Token(self.args[i].clone()))
                }
                "r" | "remove" => {
                    self.tokens.push(CLIOptions::Remove);
                    i += 1;
                    self.tokens.push(CLIOptions::Token(self.args[i].clone()))
                }
                "h" | "--help" | "help" => self.tokens.push(CLIOptions::Help),
                _ => self.tokens.push(CLIOptions::Unknown),
            };
            i += 1;
        };
    }
}

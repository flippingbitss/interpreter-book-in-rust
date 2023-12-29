use crate::{lexer, parser::Parser};

const PROMPT: &'static str = ">> ";

pub fn start() {
    loop {
        print!("{PROMPT}");
        for line in std::io::stdin().lines() {
            match line {
                Ok(line) => {
                    let l = lexer::Lexer::new(line.as_bytes());
                    let mut p = Parser::new(l);
                    match p.parse() {
                        Ok(prog) => {
                            for s in prog.stmts {
                                println!("{}", s);
                            }
                        }
                        Err(errors) => {
                            println!("failed to parse input. Errors:\n {:?}", errors);
                        }
                    }
                }
                Err(_) => panic!("unknown error"),
            }
        }
    }
}

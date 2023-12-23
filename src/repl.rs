use crate::{lexer, token::TokenType};

const PROMPT: &'static str = ">> ";

pub fn start() {
    loop {
        print!("{PROMPT}");
        for line in std::io::stdin().lines() {
            match line {
                Ok(line) => {
                    let mut l = lexer::Lexer::new(line.as_bytes());
                    let mut tok = l.next_token();
                    while tok.token_type != TokenType::EOF {
                        println!("{tok}");
                        tok = l.next_token();
                    }
                },
                Err(_) => panic!("unknown error"),
            }
        }
    }
}

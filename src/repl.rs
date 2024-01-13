use crate::{lexer, parser::Parser, evaluator::eval_program, env::Env};

pub fn start() {
    loop {
        println!("Try out the RPPL - (Read-parse-print-loop)\n>>");
        for line in std::io::stdin().lines() {
            match line {
                Ok(line) => {
                    let l = lexer::Lexer::new(line.as_bytes());
                    let mut p = Parser::new(l);
                    let mut env = Env::new();
                    match p.parse() {
                        Ok(prog) => {
                            match eval_program(prog, &mut env) {
                                Ok(value) => println!("{}", value),
                                Err(err) => println!("Error during eval: {}", err)
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

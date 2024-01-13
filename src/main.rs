mod lexer;
mod token;
mod repl;
mod parser;
mod ast;
mod object;
mod evaluator;
mod env;

fn main() {
    repl::start();
}

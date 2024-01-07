mod lexer;
mod token;
mod repl;
mod parser;
mod ast;
mod object;
mod evaluator;

fn main() {
    repl::start();
}

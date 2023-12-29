Implementation of [Writing An Interpreter In Go](https://interpreterbook.com/) in Rust

Things to improve
----------
- [ ] Support float literals
- [ ] Support Utf-8 charset
- [ ] Use arena alloc or a single `Vec` for AST storage
- [ ] Make `Lexer` use an `impl Iterator<Token>`
- [ ] Attach error messages to result types

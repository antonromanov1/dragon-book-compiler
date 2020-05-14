mod lexer;
mod symbols;
mod parser;

fn main() {
    let lex = lexer::Lexer::new();
    let parser = parser::Parser::new(lex);
    parser.program();
    println!("");
}

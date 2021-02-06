mod ir;
mod lexer;
mod parser;
mod symbols;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Lexical analyzer needs 1 argument - source file name");
        return ();
    }
    let lex = Lexer::new(&(args[1]));
    let mut parser = Parser::new(lex);
    parser.program();
    println!("");
}

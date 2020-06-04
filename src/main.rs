mod lexer;
mod parser;
mod generate;
use lexer::*;
use parser::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Lexical analyzer needs 1 argument - source file name");
        return ()
    }
    let mut lex = Lexer::new(&(args[1]));
    let mut parser = Parser::new(lex);

    let set = parser.program();
    generate::generate(set.0, set.1, set.2, &(args[1]));
}

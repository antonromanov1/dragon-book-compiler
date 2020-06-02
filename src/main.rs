mod lexer;
use lexer::Lexer;
use lexer::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Lexical analyzer needs 1 argument - source file name");
        return ()
    }
    let mut lex = Lexer::new(&(args[1]));

    loop {
        match lex.scan() {
            Token::Word(a) => match a {
                Word::Word(y) => println!("    Word:{},", y.lexeme),
                _ => (),
            },
            Token::Num(b) => println!("Integer:{},", b.value),
            Token::Real(c) => println!("Real number:{},", c.value),
            Token::Token(d) => println!("    Unknown token:{},",
                                        std::char::from_u32(d.tag).unwrap()),
            Token::Eof => break,
        };
    }
}

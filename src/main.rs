mod lexer;
use lexer::Lexer;
use lexer::*;

fn main() {
    let mut lex = Lexer::new("input.kt");

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

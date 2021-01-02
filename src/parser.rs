use std::rc::Rc;
use std::cell::RefCell;

use crate::ir::*;
use crate::lexer::*;
use crate::symbols::*;

#[allow(dead_code)]
pub struct Parser {
    // lex - lexical analyzer for this parser
    // look - lookahead token
    // top - current or top symbol table
    // enclosing - pointer to enclosing loop
    // temp_count - number of temporary variables

    lex: Lexer,
    look: Token,
    top: Option<Box<Env>>,
    enclosing: Option<Box<dyn StmtAble>>,
    temp_count: Rc<RefCell<u8>>,
}

#[allow(dead_code)]
impl Parser {
    fn move_(&mut self) {
        self.look = self.lex.scan();
    }

    pub fn new(l: Lexer) -> Parser {
        let mut p = Parser {
            lex: l,
            look: Token::Token(TokenBase {
                tag: 0,
            }),
            top: None,
            enclosing: None,
            temp_count: Rc::new(RefCell::new(0)),
        };
        p.move_();
        p
    }

    fn error(&self, s: &str) -> ! {
        println!("Syntax error on line {}: {}", self.lex.line_num, s);
        std::process::exit(0);
    }

    fn expected(&self, s: &str, expected: &str) -> ! {
        print!("Syntax error near line {}: ", self.lex.line_num);
        println!("{}, expected '{}'", s, expected);
        std::process::exit(0);
    }

    fn match_(&mut self, t: u32) {
        match self.look.get_tag() {
            Some(a) => {
                if a == t {
                    self.move_();
                }
                else {
                    // A temporary decision, tag is 4 bytes, bad cast
                    // TODO
                    self.expected(&format!("{}", (a as u8) as char),
                                  &format!("{}", (t as u8) as char));
                }
            },
            None => panic!("Unexpected event"),
        };
    }

    /*
    fn block() -> Option<Box<dyn StmtAble>> {
    }
     */
}

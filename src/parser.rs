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
    // labels - number of labels

    lex: Lexer,
    look: Token,
    top: Option<Box<Env>>,
    enclosing: Option<Box<dyn StmtAble>>,
    temp_count: Rc<RefCell<u8>>,
    labels: Rc<RefCell<u32>>,
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
            labels: Rc::new(RefCell::new(0)),
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
            Some(tag) => {
                if tag == t {
                    self.move_();
                }
                else {
                    // TODO: change, it is a temporary decision, tag is 4 bytes, bad cast
                    self.expected(&format!("{}", (tag as u8) as char),
                                  &format!("{}", (t as u8) as char));
                }
            },
            None => panic!("End of file reached"),
        };
    }

    /*
    fn block() -> Option<Box<dyn StmtAble>> {
    }
     */

    fn term(&mut self) -> Box<dyn ExprAble> {
        let mut x = self.unary();
        while self.look.get_tag().unwrap() == '*' as u32 ||
              self.look.get_tag().unwrap() == '/' as u32 {

            let tok = self.look.clone();
            self.move_();
            x = Box::new(Arith::new(tok, x, self.unary(), self.lex.line_num,
                                    self.temp_count.clone()));
        }
        x
    }

    fn unary(&mut self) -> Box<dyn ExprAble> {
        if self.look.get_tag().unwrap() == '-' as u32 {
            self.move_();
            return Box::new(Unary::new(Token::Word(Word::Word(word_minus())), self.unary(),
                                       self.temp_count.clone()));
        }
        else {
            return self.factor();
        }
    }

    fn factor(&mut self) -> Box<dyn ExprAble> {
        match self.look.get_tag() {
            Some(tag) => {
                /*
                if tag == '(' as u32 {
                    self.move_();
                    let x = self.bool_();
                    self.match_(')');
                    return x;
                }
                else if tag == Tag::Num as u32 {
                ...
                }
                */

                if tag == Tag::Num as u32 {
                    let x = Box::new(Constant::new(self.look.clone(), type_int()));
                    self.move_();
                    return x;
                }
                else if tag == Tag::Real as u32 {
                    let x = Box::new(Constant::new(self.look.clone(), type_float()));
                    self.move_();
                    return x;
                }
                else if tag == Tag::True as u32 {
                    let x = Box::new(constant_true());
                    self.move_();
                    return x;
                }
                else if tag == Tag::False as u32 {
                    let x = Box::new(constant_false());
                    self.move_();
                    return x;
                }
                else if tag == Tag::Id as u32 {
                    let s = self.look.to_string();
                    #[allow(unused_assignments)]
                    let mut id: Option<Id> = None;

                    match &self.look {
                        Token::Word(word) => {
                            match &word {
                                Word::Word(w) => {
                                    id = (*self.top.as_ref().unwrap()).get(&w);
                                },
                                _ => panic!("Unreachable"),
                            }
                        },
                        _ => panic!("Unreachable"),
                    }

                    match id {
                        None => self.error(&format!("{} undeclared", s)),
                        _ => {}
                    }
                    self.move_();
                    return Box::new(id.unwrap());
                }
                else {
                    self.error("syntax error");
                }
            },
            None => panic!("End of file reached"),
        }
    }
}

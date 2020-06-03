use crate::lexer::*;
use std::collections::HashMap;

enum Stmt {
    Print(String),
}

pub struct Node {
    left: Box<Node>,
    right: Box<Node>,
    stmt: Stmt,
    is_null: bool,
}

pub struct Parser {
    lex: Lexer,
    look: Token,
}

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
        };
        p.move_();
        p
    }

    fn error(&self, s: &str) {
        println!("Near line {}: {}", self.lex.line_num, s);
        std::process::exit(0);
    }

    fn match_(&mut self, t: u32) {
        if self.look.get_tag() == Some(t) {
            self.move_();
        }
        else {
            self.error("syntax error");
        }
    }

    /*
    pub fn program(&mut self) -> (HashMap<String, TypeBase>, *mut Node) {
        (HashMap::new(), 0 as *mut Node)
    }
    */
}

use crate::lexer::*;
use std::collections::HashMap;

enum Stmt {
    Print(String),
}

pub struct Node {
    left: *mut Node,
    right: *mut Node,
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
            self.error("syntax error1");
        }
    }

    fn match_word(&mut self, s: &str) {
        match &self.look {
            Token::Word(a) => {
                match a {
                    Word::Word(x) => {
                        if &(x.lexeme) == s {
                            self.move_();
                        }
                        else {
                            self.error(&x.lexeme);
                        }
                    },
                    Word::Type(y) => {
                        if &(y.word.lexeme) == s {
                            self.move_();
                        }
                        else {
                            self.error("syntax error3");
                        }
                    }
                };
            },
            _ => {
                self.error("syntax error4");
            }
        }
    }

    fn read_word(&self, s: &str) -> bool {
        match &self.look {
            Token::Word(a) => {
                match a {
                    Word::Word(x) => {
                        if &x.lexeme == s {
                            true
                        }
                        else {
                            false
                        }
                    },
                    Word::Type(y) => {
                        if &y.word.lexeme == s {
                            true
                        }
                        else {
                            false
                        }
                    },
                }
            },
            _ => {
                self.error("syntax error5");
                false
            },
        }
    }

    fn not_id(&self) -> String {
        self.error("should be identifier here");
        String::new()
    }

    pub fn program(&mut self) -> (u32, HashMap<String, TypeBase>, *mut Node) {
        self.match_word("def");
        self.match_word("main");
        self.match_('(' as u32);
        self.match_(')' as u32);
        self.match_('{' as u32);

        let mut id = String::new();
        while self.read_word("let") {
            self.match_word("let");
            id = match &self.look {
                Token::Word(a) => {
                    match a {
                        Word::Word(x) => {
                            if x.token.tag == Tag::Id as u32 {
                                x.lexeme.clone()
                            }
                            else {
                                self.not_id()
                            }
                        },
                        Word::Type(y) => self.not_id(),
                    }
                },
                _ => self.not_id(),
            };
            self.match_(':' as u32);
        }
        (0, HashMap::new(), 0 as *mut Node)
    }
}

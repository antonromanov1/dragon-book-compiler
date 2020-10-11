use crate::lexer::*;
use std::collections::HashMap;

enum Stmt {
    Null,
    // Print(String),
}

#[allow(dead_code)]
pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    stmt: Stmt,
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
                    /*Word::Type(y) => {
                        if &(y.word.lexeme) == s {
                            self.move_();
                        }
                        else {
                            self.error("syntax error3");
                        }
                    }*/
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
                    /* Word::Type(y) => {
                        if &y.word.lexeme == s {
                            true
                        }
                        else {
                            false
                        }
                    },*/
                }
            },
            _ => {
                // self.error("syntax error5");
                false
            },
        }
    }

    #[inline]
    #[allow(dead_code)]
    fn not_id(&self) -> String {
        self.error("should be identifier here");
        String::new()
    }

    #[inline]
    fn exit_with_type_base(&self) -> TypeBase {
        self.error("should be a type");
        // empty structure, added only in order to satisfy
        // rustc, needed because rustc does not care that
        // Parser::error() exits the process
        TypeBase {
            word: WordBase {
                token: TokenBase {
                    tag: 0,
                },
            lexeme: String::new(),
            },
            width: 0,
        }
    }

    fn stmt(&mut self) -> Option<Box<Node>> {
        /*
        match self.look.get_tag() {
            ';' as u32 => {
                self.move_();
                None
            },

            Tag::If
        }
         */
        self.move_();
        None
        /*Some(Box::new(Node {
            left: None,
            right: None,
            stmt: Stmt::Print("".to_string()),
        }))*/
    }

    fn stmts(&mut self) -> Option<Box<Node>> {
        match &self.look {
            Token::Eof => None,
            _ => {
                Some(Box::new(Node {
                    left: self.stmt(),
                    right: self.stmts(),
                    stmt: Stmt::Null,
                }))
            },
        }
    }

    pub fn program(&mut self) -> (u32, HashMap<String, TypeBase>, Option<Box<Node>>) {
        let mut used: u32 = 0;
        let mut variables = HashMap::<String, TypeBase>::new();

        self.match_word("def");
        self.match_word("main");
        self.match_('(' as u32);
        self.match_(')' as u32);
        self.match_('{' as u32);

        // variable declarations handling here
        while self.read_word("let") {
            self.match_word("let");
            let id = match &self.look {
                Token::Word(x) => match x {
                    Word::Word(a) => a.lexeme.clone(),
                    // _ => String::new(),
                },
                _ => String::new(),
            };
            self.match_(Tag::Id as u32);
            self.match_(':' as u32);
            let type_ = match &self.look {
                Token::Word(a) => match a {
                        Word::Word(x) => {
                            let mut w: u8 = 0;
                            if x.lexeme == "uint32" {
                                w = 4;
                            }
                            else if x.lexeme == "uint64" {
                                w = 8;
                            }

                            used = used + w as u32;
                            TypeBase {
                                word: x.clone(),
                                width: w,
                            }
                        },
                        /*_ => {
                            self.exit_with_type_base()
                        },*/
                },
                _ => self.exit_with_type_base()
            };
            self.move_();
            self.match_(';' as u32);

            variables.insert(id, type_);
        }

        let ast = self.stmts();

        (used, variables, ast)
    }
}

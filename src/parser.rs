use crate::lexer::*;
use std::collections::HashMap;

#[allow(dead_code)]
struct Set {
    id: String,
}

enum Stmt {
    Null,
    // Assign(Set),
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
            _ => panic!("Unexpected event"),
        };
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
                            self.expected(&x.lexeme, s);
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
    }

    fn stmt(&mut self, variables: &HashMap<String, TypeBase>)
        -> Option<Box<Node>> {
        match &self.look {
            Token::Word(w) => {
                match w {
                    Word::Word(wo) => {
                        match variables.get(&wo.lexeme) {
                            Some(_x) => self.move_(),
                            None => panic!("{}: no variable", wo.lexeme),
                        }
                    },
                }
            },
            _ => self.move_(),
        }
        None

        /*self.move_();
        None*/

        /*Some(Box::new(Node {
            left: None,
            right: None,
            stmt: Stmt::Print("".to_string()),
        }))*/
    }

    fn stmts(&mut self, variables: &HashMap<String, TypeBase>)
        -> Option<Box<Node>> {
        match &self.look {
            Token::Eof => None,
            _ => {
                Some(Box::new(Node {
                    left: self.stmt(&variables),
                    right: self.stmts(&variables),
                    stmt: Stmt::Null,
                }))
            },
        }
    }

    /// variable declarations handling here
    #[inline]
    fn declarations(&mut self, used: &mut usize,
                    variables: &mut HashMap::<String, TypeBase>) {
        while self.read_word("let") {
            self.match_word("let");
            let id = match &self.look {
                Token::Word(x) => match x {
                    Word::Word(a) => a.lexeme.clone(),
                    // _ => String::new(),
                },
                // TODO: String::new() is wrong, should be replaced
                _ => String::new(),
            };
            self.match_(Tag::Id as u32);
            self.match_(':' as u32);
            let type_ = match &self.look {
                Token::Word(a) => match a {
                    Word::Word(x) => {
                        let w: usize;
                        if x.lexeme == "uint32" {
                            w = 4;
                        }
                        else if x.lexeme == "uint64" {
                            w = 8;
                        }
                        else {
                            println!("Unknown type {}", x.lexeme);
                            std::process::exit(0);
                        }

                        *used = *used + w;
                        TypeBase::new(x.clone(), w)
                    },
                },
                _ => self.error("Should be a type")
            };
            self.move_();
            self.match_(';' as u32);

            variables.insert(id, type_);
        }
    }

    pub fn program(&mut self) -> (usize, HashMap<String, TypeBase>, Option<Box<Node>>) {
        let mut used: usize = 0;
        let mut variables = HashMap::<String, TypeBase>::new();

        self.match_word("def");
        self.match_word("main");
        self.match_('(' as u32);
        self.match_(')' as u32);
        self.match_('{' as u32);

        self.declarations(&mut used, &mut variables);

        let ast = self.stmts(&variables);

        (used, variables, ast)
    }
}

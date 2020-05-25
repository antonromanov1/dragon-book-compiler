mod input;
use crate::lexer::tag;
use crate::lexer::word;
use crate::symbols::type_mod;
use std::collections::HashMap;

static line: u32 = 1;

pub struct Lexer {
    peek: char,
    words: HashMap<word::Word, String>
}

impl Lexer {
    pub fn reserve(&mut self, w: word::Word) {
        self.words.insert(w.lexeme, w);
    }

    pub fn new() -> Lexer{
        let mut lex = Lexer{peek: ' ', words: HashMap::new()};
        Lexer::reserve(&mut lex, word::Word::new("if".to_string(), tag::IF));
        Lexer::reserve(&mut lex, word::Word::new("else".to_string(),
                                                   tag::ELSE));
        Lexer::reserve(&mut lex, word::Word::new("while".to_string(),
                                                   tag::WHILE));
        Lexer::reserve(&mut lex, word::Word::new("do".to_string(),
                                                    tag::DO));
        Lexer::reserve(&mut lex, word::Word::new("break".to_string(),
                                                   tag::BREAK));

        Lexer::reserve(&mut lex, word::True);
        Lexer::reserve(&mut lex, word::False);

        Lexer::reserve(&mut lex, type_mod::Int);
        Lexer::reserve(&mut lex, type_mod::Char);
        Lexer::reserve(&mut lex, type_mod::Bool);
        Lexer::reserve(&mut lex, type_mod::Float);
        lex
    }

    pub fn readch(&mut self) {
        match input::read() {
            Some(x) => self.peek = x,
            None => panic!("Can not read a character"),
        }
    }

    pub fn readch_(&mut self, c: char) -> bool{
        self.readch();
        if self.peek != c {
            false
        }
        self.peek = ' ';
        true
    }

    pub fn scan(&mut self) -> u32 {
        while self.readch() {
            if self.peek == ' ' || self.peek == '\t' {
                continue;
            }
            else if self.peek == '\n' {
                line = line + 1;
            }
            else {
                break;
            }
        }
        match self.peek {
            '&' => tag::AND,
            '|' => tag::OR,
            '=' => tag::EQ,
            '!' => tag::NE,
            '<' => tag::LE,
            '>' => tag::GE,
        }
        if character::is_digit(self.peek) {
            let v: u32 = 0;
            loop {
                v = v * 10 + character::digit(self.peek, 10);
                self.readch();
                if character::is_digit(self.peek) {
                    break;
                }
            }
            if self.peek != '.' {
                v
            }
            let x: f64 = v as f64;
            let d: f64 = 10;
            loop {
                self.readch();
                if ! character::is_digit(self.peek) {
                    break;
                }
                x = x + character::digit(self.peek, 10) / d;
                d = d * 10;
            }
            x as u32
        }
        if character::is_letter(self.peek) {
            let mut b = String::new();
            loop {
                b.push(self.peek);
                self.readch();
                if character::is_letter_or_digit(self.peek) {
                    break;
                }
            }
            let w = self.words.get();
            if w != None {
                w.token
            }
            let w2 = word::Word::new(b, tag::ID);
            self.words.insert(b, w2);
            w2
        }
        let tok = self.peek as u32;
        self.peek = ' ';
        tok
    }
}

use std::io::BufReader;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

/// This enumeration represents token types except for symbols such {, }, etc.
#[allow(dead_code)]
pub enum Tag {
    And = 256,
    Basic, // primitive types such as char, bool, int, float and array
    Break,
    Do,
    Else,
    Eq_,
    False,
    // Ge,
    Id,
    If,
    // Index,
    // Le,
    Minus,
    // Ne,
    Num,
    Or,
    Real,
    Temp,
    True,
    While,
}

#[allow(dead_code)]
pub struct TokenBase {
    pub tag: u32,
}

#[allow(dead_code)]
impl TokenBase {
    fn new(c: u32) -> TokenBase {
        TokenBase {
            tag: c,
        }
    }
}

#[allow(dead_code)]
pub struct WordBase {
    pub token: TokenBase,
    pub lexeme: String,
}

impl WordBase {
    pub fn new(s: String, tag: u32) -> WordBase {
        WordBase {
            token: TokenBase::new(tag),
            lexeme: s,
        }
    }
}

#[allow(dead_code)]
impl Clone for WordBase {
    fn clone(&self) -> Self {
        WordBase {
            token: TokenBase {
                tag: self.token.tag,
            },
            lexeme: self.lexeme.clone(),
        }
    }
}

#[allow(dead_code)]
impl PartialEq for WordBase {
    fn eq(&self, other: &Self) -> bool {
        if (*self).token.tag != (*other).token.tag {
            false;
        }
        (*self).lexeme == (*other).lexeme
    }
}

impl Eq for WordBase {}

use std::hash::{Hash, Hasher};

impl Hash for WordBase {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.token.tag.hash(state);
        self.lexeme.hash(state);
    }
}

#[inline]
fn word_and() -> WordBase {
    WordBase::new("&&".to_string(), Tag::And as u32)
}

#[inline]
fn word_or() -> WordBase {
    WordBase::new("||".to_string(), Tag::Or as u32)
}

#[inline]
fn word_eq() -> WordBase {
    WordBase::new("==".to_string(), Tag::Eq_ as u32)
}

#[inline]
pub fn word_true() -> WordBase {
    WordBase::new("true".to_string(), Tag::True as u32)
}

#[inline]
pub fn word_false() -> WordBase {
    WordBase::new("false".to_string(), Tag::False as u32)
}

#[inline]
pub fn word_minus() -> WordBase {
    WordBase::new("minus".to_string(), Tag::Minus as u32)
}

#[allow(dead_code)]
pub struct Num {
    token: TokenBase,
    pub value: u32,
}

#[allow(dead_code)]
impl Num {
    pub fn new(v: u32) -> Num {
        Num {
            token: TokenBase {
                tag: Tag::Num as u32,
            },
            value: v,
        }
    }
}

#[allow(dead_code)]
pub struct Real {
    token: TokenBase,
    pub value: f32,
}

#[allow(dead_code)]
impl Real {
    pub fn new(v: f32) -> Real {
        Real {
            token: TokenBase {
                tag: Tag::Real as u32,
            },
            value: v,
        }
    }
}

#[allow(dead_code)]
pub struct TypeBase {
    pub word: WordBase,
    width: usize,
}

impl PartialEq for TypeBase {
    fn eq(&self, other: &Self) -> bool {
        if self.word.token.tag != other.word.token.tag {
            return false;
        }
        true
    }
}

#[inline]
pub fn type_int() -> TypeBase {
    TypeBase {
        word: WordBase::new("int".to_string(), Tag::Basic as u32),
        width: 4,
    }
}

#[inline]
pub fn type_float() -> TypeBase {
    TypeBase {
        word: WordBase::new("float".to_string(), Tag::Basic as u32),
        width: 8,
    }
}

#[inline]
pub fn type_char() -> TypeBase {
    TypeBase {
        word: WordBase::new("char".to_string(), Tag::Basic as u32),
        width: 1,
    }
}

#[inline]
pub fn type_bool() -> TypeBase {
    TypeBase {
        word: WordBase::new("bool".to_string(), Tag::Basic as u32),
        width: 1,
    }
}

#[inline]
pub fn numeric(p: &TypeBase) -> bool {
    if *p == type_int() || *p == type_float() {
        true
    }
    else {
        false
    }
}

#[allow(dead_code)]
impl TypeBase {
    pub fn new(w: WordBase, wid: usize) -> TypeBase {
        TypeBase {
            word: w,
            width: wid,
        }
    }

    #[inline]
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn max(p1: &TypeBase, p2: &TypeBase) -> Option<TypeBase> {
        if ! numeric(p1) || ! numeric(p2) {
            None
        }
        else if *p1 == type_float() || *p2 == type_float() {
            Some(type_float())
        }
        else if *p1 == type_int() || *p2 == type_int() {
            Some(type_int())
        }
        else {
            Some(type_char())
        }
    }
}

impl Clone for TypeBase {
    fn clone(&self) -> Self {
        TypeBase {
            word: self.word.clone(),
            width: self.width,
        }
    }
}

#[allow(dead_code)]
pub enum Word {
    Word(WordBase),
    Type(TypeBase),
}

impl Clone for Word {
    fn clone(&self) -> Self {
        match &*self {
            Word::Word(word) => Word::Word(word.clone()),
            Word::Type(type_) => Word::Type(type_.clone()),
        }
    }
}

#[allow(dead_code)]
pub enum Token {
    Token(TokenBase),
    Word(Word),
    Num(Num),
    Real(Real),
    Eof,
}

impl Token {
    pub fn get_tag(&self) -> Option<u32> {
        match &*self {
            Token::Token(a) => Some(a.tag),
            Token::Word(b) => {
                match b {
                    Word::Word(x) => Some(x.token.tag),
                    Word::Type(y) => Some(y.word.token.tag),
                }
            } // TODO: find out why comma is not here
            Token::Num(c) => Some(c.token.tag),
            Token::Real(d) => Some(d.token.tag),
            Token::Eof => None
        }
    }

    pub fn to_string(&self) -> String {
        match &*self {
            Token::Token(a) => {
                let mut s = String::new();
                s.push(std::char::from_u32(a.tag).unwrap());
                s
            },
            Token::Word(b) => {
                match b {
                    Word::Word(x) => x.lexeme.clone(),
                    Word::Type(y) => y.word.lexeme.clone(),
                }
            },
            Token::Num(c) => format!("{}", c.value),
            Token::Real(d) => format!("{}", d.value),
            _ => panic!(),
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match &*self {
            Token::Token(tok) => {
                Token::Token(TokenBase {
                    tag: tok.tag,
                })
            },
            Token::Word(word) => {
                match word {
                    Word::Word(word_base) => {
                        Token::Word(Word::Word(word_base.clone()))
                    },
                    Word::Type(type_base) => {
                        Token::Word(Word::Type(type_base.clone()))
                    },
                }
            },
            Token::Num(num) => {
                Token::Num(Num {
                    token: TokenBase {
                        tag: num.token.tag,
                    },
                    value: num.value,
                })
            }
            Token::Real(real) => {
                Token::Real(Real {
                    token: TokenBase {
                        tag: real.token.tag,
                    },
                    value: real.value,
                })
            }
            _ => panic!("token clone"),
        }
    }
}

#[allow(dead_code)]
pub struct Lexer {
    buf_reader: BufReader<File>,
    pub line_num: u32, // uses for syntax error reports
    // line: String,
    peek: char,
    eof: bool,
    words: HashMap<String, Word>
}

#[allow(dead_code)]
impl Lexer {
    fn reserve(&mut self, w: WordBase) {
        self.words.insert(w.lexeme.clone(), Word::Word(w));
    }

    pub fn new(file_name: &str) -> Lexer {
        let mut lex = Lexer {
            buf_reader: BufReader::new(File::open(file_name).
                                                    expect("open failed")),
            line_num: 1,
            // line: String::new(),
            peek: ' ',
            eof: false,
            words: HashMap::new(),
        };

        lex.reserve(WordBase::new("if".to_string(),    Tag::If as u32));
        lex.reserve(WordBase::new("else".to_string(),  Tag::Else as u32));
        lex.reserve(WordBase::new("while".to_string(), Tag::While as u32));
        lex.reserve(WordBase::new("do".to_string(),    Tag::Do as u32));
        lex.reserve(WordBase::new("break".to_string(), Tag::Break as u32));

        lex.reserve(word_true());
        lex.reserve(word_false());

        lex.reserve(type_int().word);
        lex.reserve(type_char().word);
        lex.reserve(type_bool().word);
        lex.reserve(type_float().word);

        lex
    }

    fn read_char(&mut self) {
        let mut buffer = [0; 1];
        match self.buf_reader.read(&mut buffer) {
            Ok(x) => {
                if x != 0 {
                    self.peek = buffer[0] as char;
                }
                else {
                    self.eof = true;
                }
            }
            Err(_y) => panic!("read() failed{}", _y),
        };
    }

    fn readch(&mut self, c: char) -> bool {
        self.read_char();
        if self.peek != c {
            return false;
        }
        self.peek = ' ';
        true
    }

    pub fn scan(&mut self) -> Token {
        loop {
            if self.peek == ' ' || self.peek == '\t' {
                ()
            }
            else if self.peek == '\n' {
                self.line_num = self.line_num + 1;
            }
            else {
                break;
            }

            self.read_char();

            if self.eof {
                return Token::Eof;
            }
        }

        match self.peek {
            '&' => if self.readch('&') {
                return Token::Word(Word::Word(word_and()))
            }
            else {
                return Token::Token(TokenBase::new('&' as u32))
            },
            '|' => if self.readch('|') {
                return Token::Word(Word::Word(word_or()))
            }
            else {
                return Token::Token(TokenBase::new('|' as u32))
            },
            '=' => if self.readch('=') {
                return Token::Word(Word::Word(word_eq()))
            }
            else {
                return Token::Token(TokenBase::new('=' as u32))
            },
            _ => (),
        }

        // Number handling
        if self.peek.is_digit(10) {
            let mut v: u32 = 0;
            loop {
                v = 10 * v + self.peek.to_digit(10).unwrap();
                self.read_char();
                if ! self.peek.is_digit(10) {
                    break;
                }
            }
            if self.peek != '.' {
                return Token::Num(Num::new(v))
            }
            let mut x = v as f32;
            let mut d = 10 as f32;
            loop {
                self.read_char();
                if ! self.peek.is_digit(10) {
                    break;
                }
                x = x + self.peek.to_digit(10).unwrap() as f32 / d;
                d = d * 10 as f32;
            }
            return Token::Real(Real::new(x))
        }

        // Word handle
        if self.peek.is_alphabetic() {
            let mut s = String::new();
            loop {
                s.push(self.peek);
                self.read_char();

                if !(self.peek.is_alphabetic() || self.peek.is_digit(10)) {
                    break;
                }
            }

            match self.words.get(&s) {
                Some(x) => {
                    let w = match x {
                        Word::Word(y) => y.clone(),
                        Word::Type(z) => z.word.clone(),
                    };
                    return Token::Word(Word::Word(w));
                }
                None => {
                    let w = WordBase {
                        token: TokenBase {
                            tag: Tag::Id as u32,
                        },
                        lexeme: s.clone(),
                    };
                    self.words.insert(s, Word::Word(w.clone()));
                    return Token::Word(Word::Word(w))
                }
            }
        }

        let tok = Token::Token(TokenBase::new(self.peek as u32));
        self.peek = ' ';
        tok
    }
}

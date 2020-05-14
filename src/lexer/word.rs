use crate::lexer::tag;

pub struct Word {
    token: u32,
    lexeme: String,
}

impl Word {
    pub fn new(s: String, tag: u32) -> Word{
        Word {
            token: tag,
            lexeme: s,
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for c in self.lexeme.chars() {
            s.push(c);
        }
        s
    }
}

pub static and: Word = Word::new("&&".to_string(), tag::AND);
pub static or: Word = Word::new("||".to_string(), tag::OR);
pub static eq: Word = Word::new("==".to_string(), tag::EQ);
pub static ne: Word = Word::new("!=".to_string(), tag::NE);
pub static le: Word = Word::new("<=".to_string(), tag::LE);
pub static ge: Word = Word::new(">=".to_string(), tag::GE);
pub static minus: Word = Word::new("minus".to_string(), tag::MINUS);
pub static True: Word = Word::new("true".to_string(), tag::TRUE);
pub static False: Word = Word::new("false".to_string(), tag::FALSE);
pub static temp: Word = Word::new("t".to_string(), tag::TEMP);

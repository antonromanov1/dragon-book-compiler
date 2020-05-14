use crate::lexer::tag;

pub struct Type {
    lexeme: String,
    token: u32,
    width: u32,
}

impl Type {
    pub fn new(s: String, tag: u32, w: u32) -> Type {
        Type {
            lexeme: s,
            token: tag,
            width: w,
        }
    }
}

pub static Int: Type = Type::new("int".to_string(), tag::BASIC, 4);
pub static Float: Type = Type::new("float".to_string(), tag::BASIC, 8);
pub static Char: Type = Type::new("char".to_string(), tag::BASIC, 1);
pub static Bool: Type = Type::new("bool".to_string(), tag::BASIC, 1);

pub fn numeric(p: Type) -> bool {
    if p == Char || p == Int || p == Float {
        true
    }
    else {
        false
    }
}

pub fn max(p1: Type, p2: Type) -> Option<Type> {
    if ! numeric(p1) || ! numeric(p2) {
        None
    }
    else if p1 == Float || p2 == Float {
        Some(Float)
    }
    else if p1 == Int || p2 == Int {
        Some(Int)
    }
    else {
        Some(Char)
    }
}

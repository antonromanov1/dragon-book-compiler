use crate::symbols::type_mod;

pub struct Array {
    lexeme: String,
    token: u32,
    width: u32,

    of: type_mod::Type,
    size: usize,
}

impl Array {
    pub fn new(sz: usize, p: type_mod::Type) -> Array {
        Array {
            lexeme: "[]",
            token: tag::INDEX,
            width: sz * p.width,
            size: sz,
            of: p,
        }
    }

    pub fn to_string() -> String {
        let mut s = String::new();
        s.push_str("[");
        s.push(self.size as char);
        s.push_str("]");
        s.push_str(&self.of.to_string());
        s
    }
}

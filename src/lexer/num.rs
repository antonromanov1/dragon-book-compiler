use crate::lexer::tag;

pub struct Num {
    token: u32,
    value: u32,
}

impl Num {
    pub fn new(v: u32) -> Num {
        Num {
            token: tag::NUM,
            value: v,
        }
    }

    pub fn to_string(&self) -> String {
        std::char::from_u32(self.value).unwrap().to_string()
    }
}

use crate::lexer::tag;

pub struct Real {
    token: u32,
    value: f64,
}

impl Real {
    pub fn new(v: f64) -> Real {
        Real {
            token: tag::REAL,
            value: v,
        }
    }

    pub fn to_string(&self) -> String {
        std::char::from_u32(self.value as u32).unwrap().to_string()
    }
}

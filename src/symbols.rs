use std::collections::HashMap;

use crate::ir::*;
use crate::lexer::*;

pub struct Env {
    table: HashMap<WordBase, Id>,
    pub prev: Option<Box<Env>>,
}

impl Env {
    pub fn new(n: Option<Box<Env>>) -> Env {
        Env {
            table: HashMap::new(),
            prev: n,
        }
    }

    pub fn put(&mut self, w: WordBase, i: Id) {
        self.table.insert(w, i);
    }

    pub fn get(&self, w: &WordBase) -> Option<Id> {
        match self.table.get(w) {
            Some(id) => {
                return Some(id.clone());
            }
            None => {}
        };

        let mut e = &(self.prev);
        match e {
            Some(ptr) => loop {
                match (*ptr).table.get(w) {
                    Some(id) => {
                        return Some(id.clone());
                    }
                    None => {
                        e = &(e.as_ref().unwrap().prev);
                        match e {
                            Some(_a) => continue,
                            None => break,
                        }
                    }
                };
            },
            None => {
                return None;
            }
        };
        None
    }
}

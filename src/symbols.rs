use std::collections::HashMap;

use crate::lexer::*;
use crate::ir::*;

#[allow(dead_code)]
struct Env {
    table: HashMap<WordBase, IdBase>,
    prev: Option<Box<Env>>,
}

#[allow(dead_code)]
impl Env {
    pub fn new(n: Option<Box<Env>>) -> Env {
        Env {
            table: HashMap::new(),
            prev: n,
        }
    }

    #[allow(dead_code)]
    pub fn put(&mut self, w: WordBase, i: IdBase) {
        self.table.insert(w, i);
    }

    pub fn get(&self, w: &WordBase) -> Option<IdBase> {
        match self.table.get(w) {
            Some(id) => {
                return Some(id.clone());
            },
            None => {},
        };

        let mut e = &(self.prev);
        match e {
            Some(ptr) => {
                loop {
                    match (*ptr).table.get(w) {
                        Some(id) => {
                            return Some(id.clone());
                        },
                        None => {
                            e = &(e.as_ref().unwrap().prev);
                            match e {
                                Some(_a) => continue,
                                None => break,
                            }
                        },
                    };
                }
            },
            None => {
                return None;
            },
        };
        None
    }
}

/*
// a work piece
pub fn block() {
    let top = None;
    let mut top = Some(Box::new(Env::new(top)));
    top = top.unwrap().prev;
}
*/

use std::collections::HashMap;
use crate::inter::id::Id;

#[derive(Clone)]
pub struct Env {
    table: HashMap<u32, Id>,
    prev: *mut Env,
}

impl Env {
    pub fn new(n: *const Env) -> Env {
        let e: Env;
        e.prev: n as *mut Env;
        e
    }

    pub fn put(&mut self, w: u32, i: Id) {
        self.table.insert(w, i);
    }

    pub fn get(w: u32) -> Option<Id> {
        let mut e = self as *const Env;
        loop {
            if e == 0 as *const Env {
                break;
            }
            match (*e).table.get() {
                Some(x) => x,
                None => (),
            }
            e = (*e).prev;
        }
        None
    }
}

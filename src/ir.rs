use std::rc::Rc;
use std::cell::RefCell;

use crate::lexer::*;

#[allow(dead_code)]
struct Node {
    lexer_line: u32,
}

#[allow(dead_code)]
impl Node {
    fn new(line: u32) -> Node {
        Node {
            lexer_line: line,
        }
    }

    fn error(&self, s: &str) -> ! {
        println!("near line {}: {}", self.lexer_line, s);
        std::process::exit(0);
    }
}

#[allow(dead_code)]
fn emit_label(i: u32) {
    print!("L{}:", i);
}

fn emit(s: String) {
    println!("\t{}", s);
}

// Expressions:

/*
#[allow(dead_code)]
enum Expr {
    Constant {
        expr: ExprBase,
    },
    Expr(ExprBase),
    Id {
        expr: ExprBase,
        offset: u32,
    },
    Op(Op),
}
*/

pub trait ExprAble {
    fn gen(&self) -> Box<dyn ExprAble>;
    fn reduce(&self) -> Box<dyn ExprAble>;
    fn jumping(&self, t: u32, f: u32);
    fn emit_jumps(&self, test: String, t: u32, f: u32);
    fn to_string(&self) -> String;

    fn get_type(&self) -> &Option<TypeBase>;
}

pub struct ExprBase {
    op: Token,
    type_: Option<TypeBase>, // TODO: understand do I really need Option<TypeBase> instead of
                             // TypeBase
}

impl ExprBase {
    pub fn new(tok: Token, p: Option<TypeBase>) -> ExprBase {
        ExprBase {
            op: tok,
            type_: p,
        }
    }
}

impl Clone for ExprBase {
    fn clone(&self) -> Self {
        ExprBase {
            op: self.op.clone(),
            type_: match &self.type_ {
                Some(type_base) => Some(type_base.clone()),
                None => None,
            },
        }
    }
}

impl ExprAble for ExprBase {
    fn gen(&self) -> Box<dyn ExprAble> {
        Box::new(self.clone())
    }

    fn reduce(&self) -> Box<dyn ExprAble> {
        Box::new(self.clone())
    }

    fn emit_jumps(&self, test: String, t: u32, f: u32) {
        if t != 0 && f != 0 {
            emit(format!("if {} goto L{}", test, t));
            emit(format!("goto L{}", f));
        }
        else if t != 0 {
            emit(format!("if {} goto L{}", test, t));
        }
        else if f != 0 {
            emit(format!("iffalse {} goto L{}", test, f));
        }
    }

    fn to_string(&self) -> String {
        self.op.to_string()
    }

    fn jumping(&self, t: u32, f: u32) {
        self.emit_jumps(self.to_string(), t, f);
    }

    fn get_type(&self) -> &Option<TypeBase> {
        &self.type_
    }
}

#[allow(dead_code)]
struct Temp {
    expr_base: ExprBase,
    number: u8,
}

#[allow(dead_code)]
impl Temp {
    fn new(p: TypeBase, temp_count: Rc<RefCell<u8>>) -> Temp {
        {
            let mut reference = temp_count.borrow_mut();
            *reference = *reference + 1;
        }
        Temp {
            expr_base: ExprBase {
                op: Token::Word(Word::Word(WordBase {
                    lexeme: "t".to_string(),
                    token: TokenBase {
                        tag: Tag::Temp as u32,
                    },
                })),
                type_: Some(p),
            },
            number: *temp_count.borrow(),
        }
    }
}

impl ExprAble for Temp {
    fn to_string(&self) -> String {
        format!("t{}", self.number)
    }

    // Explicitly inherited:

    fn gen(&self) -> Box<dyn ExprAble> {
        self.expr_base.gen()
    }

    fn reduce(&self) -> Box<dyn ExprAble> {
        self.expr_base.reduce()
    }

    fn jumping(&self, t: u32, f: u32) {
        self.expr_base.jumping(t, f);
    }

    fn emit_jumps(&self, test: String, t: u32, f: u32) {
        self.expr_base.emit_jumps(test, t, f);
    }

    fn get_type(&self) -> &Option<TypeBase> {
        &self.expr_base.type_
    }
}

pub struct Id {
    expr_base: ExprBase,
    offset: u32,
}

impl Id {
    #[allow(dead_code)]
    pub fn new(id: WordBase, p: TypeBase, b: u32) -> Id {
        Id {
            expr_base: ExprBase::new(Token::Word(Word::Word(id)), Some(p)),
            offset: b,
        }
    }
}

impl Clone for Id {
    fn clone(&self) -> Self {
        Id {
            expr_base: self.expr_base.clone(),
            offset: self.offset,
        }
    }
}

impl ExprAble for Id {
    // All explicitly inherited

    fn gen(&self) -> Box<dyn ExprAble> {
        self.expr_base.gen()
    }

    fn reduce(&self) -> Box<dyn ExprAble> {
        self.expr_base.reduce()
    }

    fn jumping(&self, t: u32, f: u32) {
        self.expr_base.jumping(t, f);
    }

    fn emit_jumps(&self, test: String, t: u32, f: u32) {
        self.expr_base.emit_jumps(test, t, f);
    }

    fn to_string(&self) -> String {
        self.expr_base.to_string()
    }

    fn get_type(&self) -> &Option<TypeBase> {
        self.expr_base.get_type()
    }
}

#[allow(dead_code)]
struct OpBase {
    expr_base: ExprBase, // TODO: refactor
    pub temp_count: Rc<RefCell<u8>>,
}

impl OpBase {
    #[allow(dead_code)]
    pub fn new(tok: Token, p: Option<TypeBase>, count: Rc<RefCell<u8>>) -> OpBase {
        OpBase {
            expr_base: ExprBase::new(tok, p),
            temp_count: count,
        }
    }
}

impl ExprAble for OpBase {
    fn reduce(&self) -> Box<dyn ExprAble> {
        let x = self.gen();
        let t = Box::new(
                  Temp::new((*self.get_type()).as_ref().unwrap().clone(), self.temp_count.clone()));
        emit(format!("{} = {}", t.to_string(), x.to_string()));
        t
    }

    // Inherited:

    fn gen(&self) -> Box<dyn ExprAble> {
        self.expr_base.gen()
    }

    fn emit_jumps(&self, test: String, t: u32, f: u32) {
        self.expr_base.emit_jumps(test, t, f);
    }

    fn to_string(&self) -> String {
        self.expr_base.to_string()
    }

    fn jumping(&self, t: u32, f: u32) {
        self.expr_base.jumping(t, f);
    }

    fn get_type(&self) -> &Option<TypeBase> {
        &self.expr_base.type_
    }
}

pub struct Arith {
    op_base: OpBase,
    expr1: Box<dyn ExprAble>,
    expr2: Box<dyn ExprAble>,
    line: u32,
}

impl Arith {
    fn error(line: u32, s: &str) -> ! {
        let node = Node::new(line);
        node.error(s);
    }

    pub fn new(tok: Token, x1: Box<dyn ExprAble>, x2: Box<dyn ExprAble>, line: u32,
               count: Rc<RefCell<u8>>) -> Arith {
        let mut ret = Arith {
            op_base: OpBase::new(tok, None, count),
            expr1: x1,
            expr2: x2,
            line: line,
        };

        let type1 = (*ret.expr1).get_type().as_ref().unwrap();
        let type2 = (*ret.expr2).get_type().as_ref().unwrap();
        match TypeBase::max(type1, type2) {
            Some(type_base) => ret.op_base.expr_base.type_ = Some(type_base),
            None => Arith::error(line, "type error"),
        };
        ret
    }
}

impl ExprAble for Arith {
    fn gen(&self) -> Box<dyn ExprAble> {
        /*
        Box::new(Arith {
            op_base: OpBase {
                expr_base: ExprBase {
                    op: self.op_base.expr_base.op.clone(),
                    type_: match &self.op_base.expr_base.type_ {
                        Some(type_base) => Some(type_base.clone()),
                        None => None,
                    },
                },
            },
            expr1: self.expr1.reduce(),
            expr2: self.expr2.reduce(),
            line: self.line,
        })
        */
        Box::new(Arith::new(self.op_base.expr_base.op.clone(), self.expr1.reduce(),
                            self.expr2.reduce(), self.line, self.op_base.temp_count.clone()))
    }

    fn to_string(&self) -> String {
        format!("{} {} {}",
            (*self.expr1).to_string(),
            self.op_base.expr_base.op.to_string(),
            (*self.expr2).to_string()
        )
    }

    // Explicitly inherited:

    fn reduce(&self) -> Box<dyn ExprAble> {
        self.op_base.reduce()
    }

    fn jumping(&self, t: u32, f: u32) {
        self.op_base.jumping(t, f);
    }

    fn emit_jumps(&self, test: String, t: u32, f: u32) {
        self.op_base.emit_jumps(test, t, f);
    }

    fn get_type(&self) -> &Option<TypeBase> {
        self.op_base.get_type()
    }
}

pub struct Unary {
    op_base: OpBase,
    expr: Box<dyn ExprAble>,
}

impl Unary {
    pub fn new(tok: Token, x: Box<dyn ExprAble>, count: Rc<RefCell<u8>>) -> Unary {
        let type_ = TypeBase::max(&type_int(), (*x).get_type().as_ref().unwrap());
        if type_ == None {
            panic!("type error"); // TODO: add output of line of source code
        }

        Unary {
            op_base: OpBase::new(tok, type_, count),
            expr: x,
        }
    }
}

impl ExprAble for Unary {
    fn gen(&self) -> Box<dyn ExprAble> {
        Box::new(
            Unary::new(self.op_base.expr_base.op.clone(), (*self.expr).reduce(),
                       self.op_base.temp_count.clone()))
    }

    fn to_string(&self) -> String {
        self.op_base.expr_base.op.to_string().clone() + &(*self.expr).to_string()
    }

    // Explicitly inherited
    fn reduce(&self) -> Box<dyn ExprAble> {
        self.op_base.reduce()
    }

    fn jumping(&self, t: u32, f: u32) {
        self.op_base.jumping(t, f);
    }

    fn emit_jumps(&self, test: String, t: u32, f: u32) {
        self.op_base.emit_jumps(test, t, f);
    }

    fn get_type(&self) -> &Option<TypeBase> {
        self.op_base.get_type()
    }
}

pub struct Constant {
    expr_base: ExprBase,
}

impl Constant {
    pub fn new(tok: Token, p: TypeBase) -> Constant {
        Constant {
            expr_base: ExprBase::new(tok, Some(p)),
        }
    }
}

#[inline]
pub fn constant_true() -> Constant {
    Constant {
        expr_base: ExprBase::new(Token::Word(Word::Word(word_true())), Some(type_bool())),
    }
}

#[inline]
pub fn constant_false() -> Constant {
    Constant {
        expr_base: ExprBase::new(Token::Word(Word::Word(word_false())), Some(type_bool())),
    }
}

impl ExprAble for Constant {
    fn jumping(&self, t: u32, f: u32) {
        match &self.expr_base.op {
            Token::Word(word) => {
                match word {
                    Word::Word(base) => {
                        if (base.lexeme == "true".to_string()) && (t != 0) {
                            emit(format!("goto L{}", t));
                        }
                        else if (base.lexeme == "false".to_string()) && (f != 0) {
                            emit(format!("goto L{}", f));
                        }
                    }
                    _ => {}
                }
            },
            _ => {}
        }
    }

    // Explicitly inherited:

    fn gen(&self) -> Box<dyn ExprAble> {
        self.expr_base.gen()
    }

    fn reduce(&self) -> Box<dyn ExprAble> {
        self.expr_base.reduce()
    }

    fn emit_jumps(&self, test: String, t: u32, f: u32) {
        self.expr_base.emit_jumps(test, t, f);
    }

    fn to_string(&self) -> String {
        self.expr_base.to_string()
    }

    fn get_type(&self) -> &Option<TypeBase> {
        self.expr_base.get_type()
    }
}

pub fn new_label(labels: Rc<RefCell<u32>>) {
    *labels.borrow_mut() += 1;
}

/*
pub trait LogicalAble: ExprAble {
    fn get_temp_count() -> Rc<RefCell<u8>>;

    fn gen(&self, temp_count: Rc<RefCell<u8>>) -> Box<dyn ExprAble> {
        Temp
    }

    fn check(p1: TypeBase, p2: TypeBase) -> Option<TypeBase> {
        if p1 == type_bool() && p2 == type_bool() {
            Some(type_bool())
        }
        else {
            None
        }
    }
}

pub struct LogicalBase {
    expr_base: ExprBase,
    pub expr1: Box<dyn ExprAble>,
    pub expr2: Box<dyn ExprAble>,
    temp_count: Rc<RefCell<u8>>,
}
*/

/*
impl LogicalBase {
    pub fn new(tok: Token, x1: Box<dyn ExprAble>, x2: Box<dyn ExprAble>) -> LogicalBase {
        let type_: Option<TypeBase> = LogicalAble::check((*x1).get_type().unwrap(), (*x2).get_type().unwrap());
        match type_ {
            None => panic!("type error"), // TODO: should print line
            Some(t) => {
                LogicalBase {
                    expr_base: ExprBase::new(tok, Some(t)),
                    expr1: x1,
                    expr2: x2,
                }
            }
        }
    }
}
*/

// Statements:

pub trait StmtAble {
    fn gen(&self, b: u32, a: u32);
    fn get_after(&self) -> u32;
}

#[allow(dead_code)]
pub struct Break {
    after: u32,
    stmt: Box<dyn StmtAble>,
}

#[allow(dead_code)]
impl Break {
    pub fn new(enclosing: Option<Box<dyn StmtAble>>) -> Break {
        Break {
            after: 0,
            stmt: match enclosing {
                None => panic!("unenclosed break"),
                Some(cycle_ptr) => cycle_ptr,
            },
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

use crate::lexer::*;

struct Node {
    lexer_line: u32,
}

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

pub fn emit_label(i: u32) {
    print!("L{}:", i);
}

fn emit(s: String) {
    println!("\t{}", s);
}

// Expressions:

pub trait ExprAble {
    fn gen(&self) -> Box<dyn ExprAble>;
    fn reduce(&self) -> Box<dyn ExprAble>;
    fn jumping(&self, t: u32, f: u32);
    fn emit_jumps(&self, test: String, t: u32, f: u32);
    fn to_string(&self) -> String;

    fn get_type(&self) -> &TypeBase;
}

pub struct ExprBase {
    op: Token,
    type_: TypeBase,
}

impl ExprBase {
    pub fn new(tok: Token, p: TypeBase) -> ExprBase {
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
            type_: self.type_.clone(),
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

    fn get_type(&self) -> &TypeBase {
        &self.type_
    }
}

struct Temp {
    expr_base: ExprBase,
    number: u8,
}

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
                type_: p,
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

    fn get_type(&self) -> &TypeBase {
        self.expr_base.get_type()
    }
}

pub struct Id {
    expr_base: ExprBase,
    offset: u32,
}

impl Id {
    pub fn new(id: WordBase, p: TypeBase, b: u32) -> Id {
        Id {
            expr_base: ExprBase::new(Token::Word(Word::Word(id)), p),
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

    fn get_type(&self) -> &TypeBase {
        self.expr_base.get_type()
    }
}

struct OpBase {
    expr_base: ExprBase, // TODO: refactor
    pub temp_count: Rc<RefCell<u8>>,
}

impl OpBase {
    pub fn new(tok: Token, p: TypeBase, count: Rc<RefCell<u8>>) -> OpBase {
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
                  Temp::new((*self.get_type()).clone(), self.temp_count.clone()));
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

    fn get_type(&self) -> &TypeBase {
        self.expr_base.get_type()
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

        let type1 = (*x1).get_type();
        let type2 = (*x2).get_type();
        match TypeBase::max(type1, type2) {
            Some(type_base) => {
                return Arith {
                    op_base: OpBase::new(tok, type_base, count),
                    expr1: x1,
                    expr2: x2,
                    line: line,
                };
            },
            None => Arith::error(line, "type error"),
        };
    }
}

impl ExprAble for Arith {
    fn gen(&self) -> Box<dyn ExprAble> {
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

    fn get_type(&self) -> &TypeBase {
        self.op_base.get_type()
    }
}

pub struct Unary {
    op_base: OpBase,
    expr: Box<dyn ExprAble>,
}

impl Unary {
    pub fn new(tok: Token, x: Box<dyn ExprAble>, count: Rc<RefCell<u8>>) -> Unary {
        let type_ = TypeBase::max(&type_int(), (*x).get_type());
        if type_ == None {
            panic!("type error"); // TODO: add output of line of source code
        }

        Unary {
            op_base: OpBase::new(tok, type_.unwrap(), count),
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

    fn get_type(&self) -> &TypeBase {
        self.op_base.get_type()
    }
}

pub struct Constant {
    expr_base: ExprBase,
}

impl Constant {
    pub fn new(tok: Token, p: TypeBase) -> Constant {
        Constant {
            expr_base: ExprBase::new(tok, p),
        }
    }
}

#[inline]
pub fn constant_true() -> Constant {
    Constant {
        expr_base: ExprBase::new(Token::Word(Word::Word(word_true())), type_bool()),
    }
}

#[inline]
pub fn constant_false() -> Constant {
    Constant {
        expr_base: ExprBase::new(Token::Word(Word::Word(word_false())), type_bool()),
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

    fn get_type(&self) -> &TypeBase {
        self.expr_base.get_type()
    }
}

pub fn new_label(labels: Rc<RefCell<u32>>) -> u32 {
    *labels.borrow_mut() += 1;
    *labels.borrow()
}

struct Logical {
    expr_base: ExprBase,
    pub expr1: Box<dyn ExprAble>,
    pub expr2: Box<dyn ExprAble>,
    temp_count: Rc<RefCell<u8>>,
    labels: Rc<RefCell<u32>>,
}

impl Logical {
    #[allow(dead_code)]
    fn new(tok: Token, x1: Box<dyn ExprAble>, x2: Box<dyn ExprAble>,
               count: Rc<RefCell<u8>>, labels: Rc<RefCell<u32>>) -> Logical {

        if (*(*x1).get_type()) == type_bool() &&
           (*(*x2).get_type()) == type_bool() {

            Logical {
                expr_base: ExprBase::new(tok, type_bool()),
                expr1: x1,
                expr2: x2,
                temp_count: count,
                labels: labels,
            }
        }
        else {
            panic!("type error"); // TODO: should print line
        }
    }
}

impl ExprAble for Logical {
    fn gen(&self) -> Box<dyn ExprAble> {
        let f = new_label(self.labels.clone());
        let a = new_label(self.labels.clone());
        let temp = Temp::new((*self.get_type()).clone(), self.temp_count.clone());
        self.jumping(0, f);
        emit(format!("{} = true", temp.to_string()));
        emit(format!("goto L{}", a));
        emit_label(f);
        emit(format!("{} = false", temp.to_string()));
        emit_label(a);
        Box::new(temp)
    }

    fn to_string(&self) -> String {
        format!("{} {} {}", (*self.expr1).to_string(), self.expr_base.op.to_string(),
                            (*self.expr2).to_string())
    }

    // Explicitly inherited:

    fn reduce(&self) -> Box<dyn ExprAble> {
        self.expr_base.reduce()
    }

    fn jumping(&self, t: u32, f: u32) {
        self.expr_base.jumping(t, f);
    }

    fn emit_jumps(&self, test: String, t: u32, f: u32) {
        self.expr_base.emit_jumps(test, t, f);
    }

    fn get_type(&self) -> &TypeBase {
        self.expr_base.get_type()
    }
}

pub struct And {
    logic: Logical,
}

impl And {
    #[allow(dead_code)]
    pub fn new(tok: Token, x1: Box<dyn ExprAble>, x2: Box<dyn ExprAble>, count: Rc<RefCell<u8>>,
               labels: Rc<RefCell<u32>>) -> And {
        And {
            logic: Logical::new(tok, x1, x2, count, labels),
        }
    }
}

impl ExprAble for And {
    fn jumping(&self, t: u32, f: u32) {
        let label: u32;
        if f != 0 {
            label = f;
        }
        else {
            label = new_label(self.logic.labels.clone());
        }
        self.logic.expr1.jumping(0, label);
        self.logic.expr2.jumping(t, f);
        if f == 0 {
            emit_label(label);
        }
    }

    // Explicitly inherited:

    fn gen(&self) -> Box<dyn ExprAble> {
        self.logic.gen()
    }

    fn reduce(&self) -> Box<dyn ExprAble> {
        self.logic.reduce()
    }

    fn emit_jumps(&self, test: String, t: u32, f: u32) {
        self.logic.emit_jumps(test, t, f);
    }

    fn to_string(&self) -> String {
        self.logic.to_string()
    }

    fn get_type(&self) -> &TypeBase {
        self.logic.get_type()
    }
}

// Statements:

pub trait StmtAble {
    // gen is called with labels begin and after

    fn gen(&self, b: u32, a: u32);
    fn get_after(&self) -> u32;
}

pub struct Break {
    after: u32,
    stmt: Box<dyn StmtAble>,
}

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

impl StmtAble for Break {
    fn gen(&self, _b: u32, _a: u32) {
        emit(format!("goto L{}", (*self.stmt).get_after()));
    }

    fn get_after(&self) -> u32 {
        self.after
    }
}

pub struct Seq {
    stmt1: Option<Box<dyn StmtAble>>,
    stmt2: Option<Box<dyn StmtAble>>,
}

impl Seq {
    pub fn new(s1: Option<Box<dyn StmtAble>>, s2: Option<Box<dyn StmtAble>>) -> Seq {
        Seq {
            stmt1: s1,
            stmt2: s2,
        }
    }
}

impl StmtAble for Seq {
    fn gen(&self, b: u32, a: u32) {
        if self.stmt1.is_none() {
            (*self.stmt2.as_ref().unwrap()).gen(b, a);
        }
    }

    fn get_after(&self) -> u32 {
        panic!("Unreachable code");
    }
}

pub struct Set {
    id: Box<dyn ExprAble>,
    expr: Box<dyn ExprAble>,
}

impl Set {
    pub fn new(i: Box<dyn ExprAble>, x: Box<dyn ExprAble>) -> Set {
        let p1 = (*i).get_type();
        let p2 = (*x).get_type();

        if numeric(p1) && numeric(p2) {}
        else if *p1 == type_bool() && *p2 == type_bool() {}
        else {
            panic!("type error");
        }

        Set {
            id: i,
            expr: x,
        }
    }
}

impl StmtAble for Set {
    fn gen(&self, _b: u32, _a: u32) {
        emit(format!("{} = {}", (*self.id).to_string(), (*(*self.expr).gen()).to_string()));
    }

    fn get_after(&self) -> u32 {
        panic!("Unreachable code");
    }
}

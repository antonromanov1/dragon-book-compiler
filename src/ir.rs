use std::cell::RefCell;
use std::rc::Rc;

use crate::lexer::*;

macro_rules! unreachable {
    () => {
        panic!("Unreachable code");
    };
}

fn error(s: &str, line: u32) -> ! {
    println!("near line {}: {}", line, s);
    std::process::exit(0);
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

#[derive(Clone)]
pub struct ExprBase {
    op: Token,
    type_: TypeBase,
}

impl ExprBase {
    pub fn new(tok: Token, p: TypeBase) -> ExprBase {
        ExprBase { op: tok, type_: p }
    }

    fn get_op(&self) -> &Token {
        &self.op
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
        } else if t != 0 {
            emit(format!("if {} goto L{}", test, t));
        } else if f != 0 {
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

macro_rules! gen {
    ( $self:ident, $field:ident ) => {
        fn gen(&$self) -> Box<dyn ExprAble> {
            $self.$field.gen()
        }
    }
}

macro_rules! reduce {
    ( $self:ident, $field:ident ) => {
        fn reduce(&$self) -> Box<dyn ExprAble> {
            $self.$field.gen()
        }
    }
}

macro_rules! jumping {
    ( $self:ident, $field:ident ) => {
        fn jumping(&$self, t: u32, f: u32) {
            $self.$field.jumping(t, f);
        }
    }
}

macro_rules! emit_jumps {
    ( $self:ident, $field:ident ) => {
        fn emit_jumps(&$self, test: String, t: u32, f: u32) {
            $self.$field.emit_jumps(test, t, f);
        }
    }
}

macro_rules! to_string {
    ( $self:ident, $field:ident ) => {
        fn to_string(&$self) -> String {
            $self.$field.to_string()
        }
    }
}

macro_rules! get_type {
    ( $self:ident, $field:ident ) => {
        fn get_type(&$self) -> &TypeBase {
            $self.$field.get_type()
        }
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

    gen! {self, expr_base}
    reduce! {self, expr_base}
    jumping! {self, expr_base}
    emit_jumps! {self, expr_base}
    get_type! {self, expr_base}
}

#[derive(Clone)]
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

impl ExprAble for Id {
    // All explicitly inherited

    gen! {self, expr_base}
    reduce! {self, expr_base}
    jumping! {self, expr_base}
    emit_jumps! {self, expr_base}
    to_string! {self, expr_base}
    get_type! {self, expr_base}
}

struct OpBase {
    expr_base: ExprBase,
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

macro_rules! op_reduce {
    ( $self:expr ) => {{
        let x = $self.gen();
        let t = Box::new(Temp::new(
            (*$self.get_type()).clone(),
            $self.temp_count.clone(),
        ));
        emit(format!("{} = {}", t.to_string(), x.to_string()));
        t
    }};
}

impl ExprAble for OpBase {
    fn reduce(&self) -> Box<dyn ExprAble> {
        op_reduce!(self)
    }

    // Explicitly inherited:

    gen! {self, expr_base}
    jumping! {self, expr_base}
    emit_jumps! {self, expr_base}
    to_string! {self, expr_base}
    get_type! {self, expr_base}
}

pub struct Arith {
    op_base: OpBase,
    expr1: Box<dyn ExprAble>,
    expr2: Box<dyn ExprAble>,
    line: u32,
    temp_count: Rc<RefCell<u8>>,
}

impl Arith {
    pub fn new(
        tok: Token,
        x1: Box<dyn ExprAble>,
        x2: Box<dyn ExprAble>,
        line: u32,
        count: Rc<RefCell<u8>>,
    ) -> Arith {
        let type1 = (*x1).get_type();
        let type2 = (*x2).get_type();
        match TypeBase::max(type1, type2) {
            Some(type_base) => {
                return Arith {
                    op_base: OpBase::new(tok, type_base, count.clone()),
                    expr1: x1,
                    expr2: x2,
                    line: line,
                    temp_count: count.clone(),
                };
            }
            None => error("type error", line),
        };
    }
}

impl ExprAble for Arith {
    fn gen(&self) -> Box<dyn ExprAble> {
        Box::new(Arith::new(
            self.op_base.expr_base.op.clone(),
            self.expr1.reduce(),
            self.expr2.reduce(),
            self.line,
            self.op_base.temp_count.clone(),
        ))
    }

    fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            (*self.expr1).to_string(),
            self.op_base.expr_base.op.to_string(),
            (*self.expr2).to_string()
        )
    }

    // Explicitly inherited:

    fn reduce(&self) -> Box<dyn ExprAble> {
        op_reduce!(self)
    }

    jumping! {self, op_base}
    emit_jumps! {self, op_base}
    get_type! {self, op_base}
}

pub struct Unary {
    op_base: OpBase,
    expr: Box<dyn ExprAble>,
}

impl Unary {
    pub fn new(tok: Token, x: Box<dyn ExprAble>, count: Rc<RefCell<u8>>) -> Unary {
        let type_ = TypeBase::max(&type_int(), (*x).get_type());
        if type_ == None {
            panic!("type error");
        }

        Unary {
            op_base: OpBase::new(tok, type_.unwrap(), count),
            expr: x,
        }
    }
}

impl ExprAble for Unary {
    fn gen(&self) -> Box<dyn ExprAble> {
        Box::new(Unary::new(
            self.op_base.expr_base.op.clone(),
            (*self.expr).reduce(),
            self.op_base.temp_count.clone(),
        ))
    }

    fn to_string(&self) -> String {
        self.op_base.expr_base.op.to_string().clone() + &(*self.expr).to_string()
    }

    // Explicitly inherited
    reduce! {self, op_base}
    jumping! {self, op_base}
    emit_jumps! {self, op_base}
    get_type! {self, op_base}
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
            Token::Word(word) => match word {
                Word::Word(base) => {
                    if (base.lexeme == "true".to_string()) && (t != 0) {
                        emit(format!("goto L{}", t));
                    } else if (base.lexeme == "false".to_string()) && (f != 0) {
                        emit(format!("goto L{}", f));
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    // Explicitly inherited:

    gen! {self, expr_base}
    reduce! {self, expr_base}
    emit_jumps! {self, expr_base}
    to_string! {self, expr_base}
    get_type! {self, expr_base}
}

pub fn new_label(labels: Rc<RefCell<u32>>) -> u32 {
    *labels.borrow_mut() += 1;
    *labels.borrow()
}

struct Logical {
    pub expr_base: ExprBase,
    pub expr1: Box<dyn ExprAble>,
    pub expr2: Box<dyn ExprAble>,
    temp_count: Rc<RefCell<u8>>,
    labels: Rc<RefCell<u32>>,
}

macro_rules! logical_construct {
    ( $check:expr, $tok:ident, $x1:ident, $x2:ident, $count:ident, $labels:ident ) => {{
        if $check((*$x1).get_type(), (*$x2).get_type()) {
            Logical {
                expr_base: ExprBase::new($tok, type_bool()),
                expr1: $x1,
                expr2: $x2,
                temp_count: $count,
                labels: $labels,
            }
        } else {
            panic!("type error");
        }
    }};
}

impl Logical {
    fn new(
        tok: Token,
        x1: Box<dyn ExprAble>,
        x2: Box<dyn ExprAble>,
        count: Rc<RefCell<u8>>,
        labels: Rc<RefCell<u32>>,
    ) -> Logical {
        logical_construct!(Logical::check, tok, x1, x2, count, labels)
    }

    fn check(p1: &TypeBase, p2: &TypeBase) -> bool {
        if *p1 == type_bool() && *p2 == type_bool() {
            true
        } else {
            false
        }
    }
}

macro_rules! logical_gen {
    ( $self:expr, $labels:expr, $count:expr ) => {{
        let f = new_label($labels.clone());
        let a = new_label($labels.clone());
        let temp = Temp::new((*$self.get_type()).clone(), $count.clone());
        $self.jumping(0, f);
        emit(format!("{} = true", temp.to_string()));
        emit(format!("goto L{}", a));
        emit_label(f);
        emit(format!("{} = false", temp.to_string()));
        emit_label(a);
        Box::new(temp)
    }};
}

impl ExprAble for Logical {
    fn gen(&self) -> Box<dyn ExprAble> {
        logical_gen!(self, self.labels, self.temp_count)
    }

    fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            (*self.expr1).to_string(),
            self.expr_base.op.to_string(),
            (*self.expr2).to_string()
        )
    }

    // Explicitly inherited:

    reduce! {self, expr_base}
    jumping! {self, expr_base}
    emit_jumps! {self, expr_base}
    get_type! {self, expr_base}
}

pub struct And {
    logic: Logical,
}

impl And {
    pub fn new(
        tok: Token,
        x1: Box<dyn ExprAble>,
        x2: Box<dyn ExprAble>,
        count: Rc<RefCell<u8>>,
        labels: Rc<RefCell<u32>>,
    ) -> And {
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
        } else {
            label = new_label(self.logic.labels.clone());
        }
        self.logic.expr1.jumping(0, label);
        self.logic.expr2.jumping(t, f);
        if f == 0 {
            emit_label(label);
        }
    }

    // Explicitly inherited:

    gen! {self, logic}
    reduce! {self, logic}
    emit_jumps! {self, logic}
    to_string! {self, logic}
    get_type! {self, logic}
}

pub struct Or {
    logic: Logical,
}

impl Or {
    pub fn new(
        tok: Token,
        x1: Box<dyn ExprAble>,
        x2: Box<dyn ExprAble>,
        count: Rc<RefCell<u8>>,
        labels: Rc<RefCell<u32>>,
    ) -> Or {
        Or {
            logic: Logical::new(tok, x1, x2, count, labels),
        }
    }
}

impl ExprAble for Or {
    fn jumping(&self, t: u32, f: u32) {
        let label: u32;
        if t != 0 {
            label = t;
        } else {
            label = new_label(self.logic.labels.clone());
        }
        self.logic.expr1.jumping(label, 0);
        self.logic.expr2.jumping(t, f);
        if t == 0 {
            emit_label(label);
        }
    }

    // Explicitly inherited:

    gen! {self, logic}
    reduce! {self, logic}
    emit_jumps! {self, logic}
    to_string! {self, logic}
    get_type! {self, logic}
}

pub struct Not {
    logic: Logical,
}

impl Not {
    pub fn new(
        tok: Token,
        x2: Box<dyn ExprAble>,
        count: Rc<RefCell<u8>>,
        labels: Rc<RefCell<u32>>,
    ) -> Not {
        // I use Box::new(Id::new()) as an unuseful thing cause Logical requires 2 pointers
        // TODO: rewrite it

        Not {
            logic: Logical::new(
                tok,
                Box::new(Id::new(word_true(), type_bool(), 0)),
                x2,
                count,
                labels,
            ),
        }
    }
}

impl ExprAble for Not {
    fn jumping(&self, t: u32, f: u32) {
        (*self.logic.expr2).jumping(f, t);
    }

    fn to_string(&self) -> String {
        format!(
            "{} {}",
            self.logic.expr_base.op.to_string(),
            self.logic.expr2.to_string()
        )
    }

    // Explicitly inherited:
    gen! {self, logic}
    reduce! {self, logic}
    emit_jumps! {self, logic}
    get_type! {self, logic}
}

pub struct Rel {
    logic: Logical,
}

impl Rel {
    pub fn new(
        tok: Token,
        x1: Box<dyn ExprAble>,
        x2: Box<dyn ExprAble>,
        count: Rc<RefCell<u8>>,
        labels: Rc<RefCell<u32>>,
    ) -> Rel {
        Rel {
            logic: logical_construct!(Rel::check, tok, x1, x2, count, labels),
        }
    }

    fn check(p1: &TypeBase, p2: &TypeBase) -> bool {
        if *p1 == *p2 {
            true
        } else {
            false
        }
    }
}

impl ExprAble for Rel {
    fn gen(&self) -> Box<dyn ExprAble> {
        logical_gen!(self, self.logic.labels, self.logic.temp_count)
    }

    fn jumping(&self, t: u32, f: u32) {
        let a = self.logic.expr1.reduce();
        let b = self.logic.expr2.reduce();
        let test = a.to_string()
            + " "
            + &(*self.logic.expr_base.get_op()).to_string()
            + " "
            + &b.to_string();
        self.emit_jumps(test, t, f);
    }

    // Explicitly inherited:

    reduce! {self, logic}
    emit_jumps! {self, logic}
    to_string! {self, logic}
    get_type! {self, logic}
}

// Statements:

pub trait StmtAble {
    // gen is called with labels begin, after and _gen_after which is passed by While node

    fn gen(&self, _b: u32, _a: u32, _gen_after: u32) {}

    fn is_null(&self) -> bool {
        false
    }

    fn init(&mut self, _x: Box<dyn ExprAble>, _s: Box<dyn StmtAble>) {
        unreachable!();
    }
}

pub struct Null {}

impl StmtAble for Null {
    fn is_null(&self) -> bool {
        true
    }
}

pub struct Break {}

impl StmtAble for Break {
    fn gen(&self, _b: u32, _a: u32, gen_after: u32) {
        emit(format!("goto L{}", gen_after));
    }
}

pub struct Seq {
    stmt1: Box<dyn StmtAble>,
    stmt2: Box<dyn StmtAble>,
    labels: Rc<RefCell<u32>>,
}

impl Seq {
    pub fn new(s1: Box<dyn StmtAble>, s2: Box<dyn StmtAble>, labels: Rc<RefCell<u32>>) -> Seq {
        Seq {
            stmt1: s1,
            stmt2: s2,
            labels: labels,
        }
    }
}

impl StmtAble for Seq {
    fn gen(&self, b: u32, a: u32, gen_after: u32) {
        if (*self.stmt1).is_null() {
            (*self.stmt2).gen(b, a, gen_after);
        } else if (*self.stmt2).is_null() {
            (*self.stmt1).gen(b, a, gen_after);
        } else {
            let label = new_label(self.labels.clone());
            (*self.stmt1).gen(b, label, gen_after);
            emit_label(label);
            (*self.stmt2).gen(label, a, gen_after);
        }
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

        if numeric(p1) && numeric(p2) {
        } else if *p1 == type_bool() && *p2 == type_bool() {
        } else {
            panic!("type error");
        }

        Set { id: i, expr: x }
    }
}

impl StmtAble for Set {
    fn gen(&self, _b: u32, _a: u32, _gen_after: u32) {
        emit(format!(
            "{} = {}",
            (*self.id).to_string(),
            (*(*self.expr).gen()).to_string()
        ));
    }
}

pub struct If {
    expr: Box<dyn ExprAble>,
    stmt: Box<dyn StmtAble>,
    labels: Rc<RefCell<u32>>,
}

macro_rules! bool_check {
    ( $x:ident, $line:expr ) => {
        if *(*$x).get_type() != type_bool() {
            error("boolean required in if", $line);
        }
    };
}

impl If {
    pub fn new(
        x: Box<dyn ExprAble>,
        s: Box<dyn StmtAble>,
        line: u32,
        labels: Rc<RefCell<u32>>,
    ) -> If {
        bool_check!(x, line);
        If {
            expr: x,
            stmt: s,
            labels: labels,
        }
    }
}

impl StmtAble for If {
    fn gen(&self, _b: u32, a: u32, gen_after: u32) {
        let label = new_label(self.labels.clone());
        (*self.expr).jumping(0, a);
        emit_label(label);
        (*self.stmt).gen(label, a, gen_after);
    }
}

pub struct Else {
    expr: Box<dyn ExprAble>,
    stmt1: Box<dyn StmtAble>,
    stmt2: Box<dyn StmtAble>,
    labels: Rc<RefCell<u32>>,
}

impl Else {
    pub fn new(
        x: Box<dyn ExprAble>,
        s1: Box<dyn StmtAble>,
        s2: Box<dyn StmtAble>,
        line: u32,
        labels: Rc<RefCell<u32>>,
    ) -> Else {
        bool_check!(x, line);
        Else {
            expr: x,
            stmt1: s1,
            stmt2: s2,
            labels: labels,
        }
    }
}

impl StmtAble for Else {
    fn gen(&self, _b: u32, a: u32, gen_after: u32) {
        let label1 = new_label(self.labels.clone());
        let label2 = new_label(self.labels.clone());
        self.expr.jumping(0, label2);
        emit_label(label1);
        (*self.stmt1).gen(label1, a, gen_after);
        emit(format!("goto L{}", a));
        emit_label(label2);
        (*self.stmt2).gen(label2, a, gen_after);
    }
}

pub struct While {
    expr: Option<Box<dyn ExprAble>>,
    stmt: Option<Box<dyn StmtAble>>,
    line: u32,
    labels: Rc<RefCell<u32>>,
}

impl While {
    pub fn new(line: u32, labels: Rc<RefCell<u32>>) -> While {
        While {
            expr: None,
            stmt: None,
            line: line,
            labels: labels,
        }
    }
}

impl StmtAble for While {
    fn gen(&self, b: u32, a: u32, _gen_after: u32) {
        self.expr.as_ref().unwrap().jumping(0, a);
        let label = new_label(self.labels.clone());
        emit_label(label);
        self.stmt.as_ref().unwrap().gen(label, b, a);
        emit(format!("goto L{}", b));
    }

    fn init(&mut self, x: Box<dyn ExprAble>, s: Box<dyn StmtAble>) {
        bool_check!(x, self.line);
        self.expr = Some(x);
        self.stmt = Some(s);
    }
}

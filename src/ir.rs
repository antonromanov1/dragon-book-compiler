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
fn new_label(labels: &mut Box<u32>) -> u32 {
    *(*labels) = *(*labels) + 1;
    *(*labels)
}

#[allow(dead_code)]
fn emit_label(i: u32) {
    print!("L{}:", i);
}

#[allow(dead_code)]
fn emit(s: String) {
    println!("\t{}", s);
}

// Expressions:

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

trait ExprAble {
    fn gen(&self, temp_count: Rc<RefCell<u8>>) -> Box<dyn ExprAble>;
    fn reduce(&self, temp_count: Rc<RefCell<u8>>) -> Box<dyn ExprAble>;
    fn jumping(&self, t: u32, f: u32);
    fn emit_jumps(&self, test: String, t: u32, f: u32);
    fn to_string(&self) -> String;

    fn get_type(&self) -> &Option<TypeBase>;
}

#[allow(dead_code)]
struct ExprBase {
    op: Token,
    type_: Option<TypeBase>,
}

impl ExprBase {
    #[allow(dead_code)]
    fn new(tok: Token, p: Option<TypeBase>) -> ExprBase {
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
    fn gen(&self, _temp_count: Rc<RefCell<u8>>) -> Box<dyn ExprAble> {
        Box::new(self.clone())
    }

    fn reduce(&self, _temp_count: Rc<RefCell<u8>>) -> Box<dyn ExprAble> {
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
enum Op {
    Op(OpBase),
    Arith(ArithBase),
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

    fn gen(&self, temp_count: Rc<RefCell<u8>>) -> Box<dyn ExprAble> {
        self.expr_base.gen(temp_count)
    }

    fn reduce(&self, temp_count: Rc<RefCell<u8>>) -> Box<dyn ExprAble> {
        self.expr_base.reduce(temp_count)
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

#[allow(dead_code)]
struct OpBase {
    expr_base: ExprBase,
}

impl OpBase {
    #[allow(dead_code)]
    pub fn new(tok: Token, p: Option<TypeBase>) -> OpBase {
        OpBase {
            expr_base: ExprBase::new(tok, p),
        }
    }
}

impl ExprAble for OpBase {
    fn reduce(&self, temp_count: Rc<RefCell<u8>>) -> Box<dyn ExprAble> {
        let x = self.gen(temp_count.clone());
        let t = Box::new(Temp::new((*self.get_type()).as_ref().unwrap().clone(), temp_count));
        emit(format!("{} = {}", t.to_string(), x.to_string()));
        t
    }

    // Inherited:

    fn gen(&self, temp_count: Rc<RefCell<u8>>) -> Box<dyn ExprAble> {
        self.expr_base.gen(temp_count)
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

#[allow(dead_code)]
struct ArithBase {
    op_base: OpBase,
    expr1: Box<dyn ExprAble>,
    expr2: Box<dyn ExprAble>,
    line: u32,
}

impl ArithBase {
    #[allow(dead_code)]
    fn error(line: u32, s: &str) -> ! {
        let node = Node::new(line);
        node.error(s);
    }

    #[allow(dead_code)]
    fn new(tok: Token, x1: Box<dyn ExprAble>, x2: Box<dyn ExprAble>, line: u32) -> ArithBase {
        let mut ret = ArithBase {
            op_base: OpBase::new(tok, None),
            expr1: x1,
            expr2: x2,
            line: line,
        };

        let type1 = (*ret.expr1).get_type().as_ref().unwrap();
        let type2 = (*ret.expr2).get_type().as_ref().unwrap();
        match TypeBase::max(type1, type2) {
            Some(type_base) => ret.op_base.expr_base.type_ = Some(type_base),
            None => ArithBase::error(line, "type error"),
        };
        ret
    }
}

impl ExprAble for ArithBase {
    fn gen(&self, temp_count: Rc<RefCell<u8>>) -> Box<dyn ExprAble> {
        Box::new(ArithBase {
            op_base: OpBase {
                expr_base: ExprBase {
                    op: self.op_base.expr_base.op.clone(),
                    type_: match &self.op_base.expr_base.type_ {
                        Some(type_base) => Some(type_base.clone()),
                        None => None,
                    },
                },
            },
            expr1: self.expr1.reduce(temp_count.clone()),
            expr2: self.expr2.reduce(temp_count),
            line: self.line,
        })
    }

    fn to_string(&self) -> String {
        format!("{} {} {}",
            (*self.expr1).to_string(),
            self.op_base.expr_base.op.to_string(),
            (*self.expr2).to_string()
        )
    }

    // Explicitly inherited:

    fn reduce(&self, temp_count: Rc<RefCell<u8>>) -> Box<dyn ExprAble> {
        self.op_base.reduce(temp_count)
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

// Statements:

pub trait StmtAble {
    fn gen(&self, b: u32, a: u32);
    fn get_after(&self) -> u32;
}

#[allow(dead_code)]
struct Break {
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

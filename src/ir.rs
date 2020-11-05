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

    fn error(&self, s: String) -> ! {
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
    Logical(Box<Logical>),
    // Op(),
}

trait ExprAble {
    fn gen(&self) -> *const dyn ExprAble where Self: Sized;
    fn reduce(&self) -> *const dyn ExprAble where Self: Sized;
    fn jumping(&self, t: u32, f: u32);
    fn emit_jumps(&self, test: String, t: u32, f: u32);
    fn to_string(&self) -> String;
}

#[allow(dead_code)]
struct ExprBase {
    op: Token,
    type_: Option<Box<TypeBase>>,
}

impl ExprBase {
    #[allow(dead_code)]
    fn new(tok: Token, p: Option<Box<TypeBase>>) -> ExprBase {
        ExprBase {
            op: tok,
            type_: p,
        }
    }
}

impl ExprAble for ExprBase {
    fn gen(&self) -> *const dyn ExprAble where Self: Sized {
        &(*self) as *const dyn ExprAble
    }

    fn reduce(&self) -> *const dyn ExprAble where Self: Sized {
        &(*self) as *const dyn ExprAble
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
}

#[allow(dead_code)]
enum Logical {
    Logical(LogicalBase),
    And {
        logical: LogicalBase,
    },
    Or {
        logical: LogicalBase,
    },
}

#[allow(dead_code)]
struct LogicalBase {
    expr_base: ExprBase,
    expr1: Expr,
    expr2: Expr,
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

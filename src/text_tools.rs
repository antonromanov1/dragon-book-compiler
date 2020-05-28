pub struct ExprBase {
    op: Token,
    type_: Type,
}

impl ExprBase {
    pub fn new(tok: Token, p: Type) -> ExprBase {
        ExprBase {
            op: tok,
            type_: p,
        }
    }

    pub fn gen(&self) {
        *self
    }

    pub fn reduce(&self) {
        *self
    }
}

struct Break {}
struct Do{}
struct Set{}
struct If{}
struct While{}

enum Stmt {
    Null,
    Break(Break),
    Do(Do),
    Set(Set),
    If(If),
    While(While),
}

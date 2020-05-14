extern crate text_tools;

mod lexer;
mod symbols;
use lexer::tag;
use symbols::type_mod;
use text_tools::{Expr, Stmt, Null, If, Else, While};

pub struct Parser {
    lex: lexer::Lexer,
    look: u32,
    top: symbols::Env,
    used: u32,
}

impl Parser {
    pub fn new(l: lexer::Lexer) -> Parser {
        let parser = Parser {
            lex: l,
            look: 0,
            top: symbols::Env::new(),
            used: 0,
        };
        self.move_();
        parser
    }

    fn move_(&mut self) {
        self.look = self.lex.scan();
    }

    fn match_(&mut self, t: u32) {
        if self.look == t {
            move_();
        }
        else {
            panic!("syntax error");
        }
    }

    pub fn program(&mut self) {
        let mut s = self.block();
        let mut begin = s.new_label();
        let mut after = s.new_label();
        s.emit_label(begin);
        s.gen(begin, after);
        s.emit_label(after);
    }

    fn block(&mut self) -> text_tools::Stmt {
        self.match_('{' as u32);
        let saved_env = self.top.clone();
        self.top = symbols::Env::new(self.top);
        self.decls();
        let s = self.stmts();
        self.match_('}' as u32);
        self.top = saved_env;
    }

    fn decls(&mut self) {
        while self.look == tag::BASIC {
            let p = type();
            let tok = self.look;
            self.match_(tag::ID);
            self.match_(';' as u32);
            let id = text_tools::Id::new(tok, p, self.used);
            self.top.put(tok, id);
            self.used = self.used + p.width();
        }
    }

    fn type(&mut self) -> type_mod::Type {
        let p = self.look;
        self.match_(tag::BASIC);
        if self.look != '[' {
            p as type_mod::Type
        }
        else {
            self.dims(p)
        }
    }

    fn dims(&mut self, p: type_mod::Type) {
        self.match_('[' as u32);
        let tok = self.look;
        self.match_(tag::NUM);
        self.match_(']' as u32);
        if self.look == '[' {
            p = self.dims(p);
        }
        symbols::Array::new(tok, p)
    }

    fn stmts(&mut self) -> text_tools::Stmt {
        if self.look == '}' {
            Null
        }
        else {
            Seq::new(self.stmt(), self.stmts())
        }
    }

    fn stmt(&mut self) -> text_tools::Stmt {
        let mut x: Expr;
        let mut s: Stmt;
        let mut s1: Stmt;
        let mut s2: Stmt;
        let mut saved_stmt: Stmt;

        match self.look {
            ';' as u32 => {
                self.move_();
                Null
            }
            tag::IF => {
                self.match_(tag::IF);
                self.match_('(' as u32);
                let x = self.bool_();
                self.match_(')' as u32);
                let s1 = self.stmt();
                if self.look != tag::ELSE {
                    If::new(x, s1)
                }
                self.match_(tag::ELSE);
                let s2 = self.stmt();
                Else::new(x, s1, s2)
            }
            tag::WHILE => {
                let mut while_node = While::new();
                saved_stmt = stmt_mod::enclosing;
                stmt_mod::enclosing = while_node;
                self.match_(tag::WHILE);
                self.match_('(' as u32);
                x = self.bool_();
                self.match_(')' as u32);
                s1 = stmt();
                while_node.init(x, s1);
                stmt_mod::enclosing = saved_stmt;
                while_node
            }
            tag::DO => {
                let mut do_node = Do::new();
                saved_stmt = stmt_mod::enclosing;
                stmt_mod::enclosing = do_node;
                self.match_(tag::DO);
                s1 = self.stmt();
                self.match_(tag::WHILE);
                self.match_('(' as u32);
                x = self.bool_();
                self.match_(')' as u32);
                self.match_(';' as u32);
                do_node.init(s1, x);
                stmt_mod::enclosing = saved_stmt;
                do_node
            }
            tag::BREAK => {
                self.match_(tag::BREAK);
                self.match_(';' as u32);
                Break::new();
            }
            '{' as u32 => self.block(),
            _ => self.assign(),
        }
    }

    fn assign(&mut self) -> Stmt {
        let mut stmt = Stmt::new();
        let t = self.look;
        self.match_(tag::ID);
        let id = self.top.get(t);
        if id == None {
            panic!("undeclared");
        }

        if look == '=' as u32 {
            self.move_();
            stmt = Set::new(id, self.bool_());
        }
        else {
            let x = self.offset(id);
            self.match_('=' as u32);
            stmt = SetElem::new(x, self.bool_());
        }
        self.match_(';' as u32);
        stmt
    }
}

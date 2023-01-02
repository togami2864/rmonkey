use std::fmt;

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

impl Program {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Program { stmts }
    }
}

#[derive(Debug)]
pub enum Stmt {
    LetStmt(LetStmt),
    ReturnStmt(ReturnStmt),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::LetStmt(LetStmt { name, value }) => write!(f, "let {} = {}", name, value),
            Stmt::ReturnStmt(ReturnStmt { value }) => write!(f, "return {}", value),
        }
    }
}

#[derive(Debug)]
pub struct LetStmt {
    pub name: Expr,
    pub value: Expr,
}

#[derive(Debug)]
pub struct ReturnStmt {
    pub value: Expr,
}

#[derive(Debug)]
pub enum Expr {
    Ident(String),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Ident(val) => write!(f, "{}", val),
        }
    }
}
#[cfg(test)]
mod tests {}

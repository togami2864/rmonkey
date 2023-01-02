#[derive(Debug)]
pub struct Program {
    stmts: Vec<Stmt>,
}

impl Program {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Program { stmts }
    }
}

#[derive(Debug)]
pub enum Stmt {
    LetStmt(LetStmt),
}

#[derive(Debug)]
pub struct LetStmt {
    pub name: Expr,
    pub value: Expr,
}

#[derive(Debug)]
pub enum Expr {
    Ident(String),
}

#[cfg(test)]
mod tests {}

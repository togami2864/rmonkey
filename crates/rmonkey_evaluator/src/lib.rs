use rmonkey_ast::{Expr, Program, Stmt};
use rmonkey_error::Result;
use rmonkey_object::Object;

#[derive(Debug)]
pub struct Eval {}

impl Eval {
    pub fn eval(&self, node: Program) -> Result<Object> {
        let mut result = Object::Null;
        for p in node.stmts.iter() {
            result = self.eval_stmt(p)?;
        }
        Ok(result)
    }

    fn eval_stmt(&self, node: &Stmt) -> Result<Object> {
        match node {
            Stmt::LetStmt { name, value } => todo!(),
            Stmt::ReturnStmt(expr) => Ok(self.eval_expr(expr)?),
            Stmt::ExprStmt(expr) => Ok(self.eval_expr(expr)?),
            Stmt::BlockStmt { stmts } => todo!(),
        }
    }

    fn eval_expr(&self, node: &Expr) -> Result<Object> {
        match node {
            Expr::Ident(_) => todo!(),
            Expr::IntLiteral(val) => Ok(Object::Int(*val)),
            Expr::BoolLiteral(val) => Ok(Object::Bool(*val)),
            Expr::If {
                condition,
                consequence,
                alternative,
            } => todo!(),
            Expr::PrefixExpr { op, right } => todo!(),
            Expr::InfixExpr { left, right, op } => todo!(),
            Expr::Func { params, body } => todo!(),
            Expr::Call { callee, args } => todo!(),
        }
    }
}

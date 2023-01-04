use rmonkey_ast::{operator::Prefix, Expr, Program, Stmt};
use rmonkey_error::Result;
use rmonkey_object::Object;

#[derive(Debug)]
pub struct Evaluator {}

impl Evaluator {
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
            Expr::PrefixExpr { op, right } => Ok(self.eval_prefix_expr(op, right)?),
            Expr::InfixExpr { left, right, op } => todo!(),
            Expr::Func { params, body } => todo!(),
            Expr::Call { callee, args } => todo!(),
        }
    }

    fn eval_prefix_expr(&self, op: &Prefix, right: &Expr) -> Result<Object> {
        let right = self.eval_expr(right)?;
        match op {
            Prefix::Minus => Ok(self.eval_minus_operator_expr(right)),
            Prefix::Bang => Ok(self.eval_bang_operator_expr(right)),
        }
    }

    fn eval_bang_operator_expr(&self, right: Object) -> Object {
        match right {
            Object::Bool(val) => Object::Bool(!val),
            Object::Null => Object::Bool(true),
            _ => Object::Bool(false),
        }
    }

    fn eval_minus_operator_expr(&self, right: Object) -> Object {
        if let Object::Int(val) = right {
            Object::Int(-val)
        } else {
            Object::Null
        }
    }
}

#[cfg(test)]
mod tests {
    use rmonkey_lexer::Lexer;
    use rmonkey_parser::Parser;

    use crate::Evaluator;

    #[test]
    fn test_prefix_expr() {
        let case = [
            ("!false", "true"),
            ("!5", "false"),
            ("!!true", "true"),
            ("!!false", "false"),
            ("!!5", "true"),
            ("!!-5", "true"),
            ("-5", "-5"),
            ("-10", "-10"),
            ("-true", "null"),
            ("-false", "null"),
        ];
        for (input, expected) in case.iter() {
            let e = Evaluator {};
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = e.eval(program).unwrap();
            assert_eq!(r.to_string(), *expected);
        }
    }
}

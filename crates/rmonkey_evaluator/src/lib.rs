use rmonkey_ast::{
    operator::{Infix, Prefix},
    Expr, Program, Stmt,
};
use rmonkey_error::Result;
use rmonkey_object::Object;

#[derive(Debug)]
pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }
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
            Expr::InfixExpr { left, right, op } => Ok(self.eval_infix_expr(op, left, right)?),
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

    fn eval_infix_expr(&self, op: &Infix, left: &Expr, right: &Expr) -> Result<Object> {
        let left = self.eval_expr(left)?;
        let right = self.eval_expr(right)?;
        match (left, right) {
            (Object::Int(left_val), Object::Int(right_val)) => {
                Ok(self.eval_int_infix_expr(op, left_val, right_val))
            }
            (Object::Bool(left_val), Object::Bool(right_val)) => match op {
                Infix::Eq => Ok(self.native_bool_to_bool_object(left_val == right_val)),
                Infix::NotEq => Ok(self.native_bool_to_bool_object(left_val != right_val)),
                _ => Ok(Object::Null),
            },
            _ => Ok(Object::Null),
        }
    }

    fn eval_int_infix_expr(&self, op: &Infix, left: i64, right: i64) -> Object {
        match op {
            Infix::Plus => Object::Int(left + right),
            Infix::Minus => Object::Int(left - right),
            Infix::Asterisk => Object::Int(left * right),
            Infix::Slash => Object::Int(left / right),
            Infix::Lt => self.native_bool_to_bool_object(left < right),
            Infix::Gt => self.native_bool_to_bool_object(left > right),
            Infix::Eq => self.native_bool_to_bool_object(left == right),
            Infix::NotEq => self.native_bool_to_bool_object(left != right),
            _ => Object::Null,
        }
    }

    fn native_bool_to_bool_object(&self, val: bool) -> Object {
        match val {
            true => Object::Bool(true),
            false => Object::Bool(false),
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
    #[test]
    fn test_integer_ope() {
        let case = [
            ("5", "5"),
            ("10", "10"),
            ("-5", "-5"),
            ("-10", "-10"),
            ("5 + 5 + 5 + 5 - 10", "10"),
            ("2 * 2 * 2 * 2 * 2", "32"),
            ("-50 + 100 - 50", "0"),
            ("5 * 2 + 10", "20"),
            ("5 + 2 * 10", "25"),
            ("50 / 2 * 2 + 10", "60"),
            ("2 * (5 + 10)", "30"),
            ("3 * 3 * 3 + 10", "37"),
            ("3 * (3 * 3) + 10", "37"),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", "50"),
        ];

        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = e.eval(program).unwrap();
            assert_eq!(r.to_string(), *expected)
        }
    }
    #[test]
    fn test_boolean_expr() {
        let case = [
            ("1 < 2", "true"),
            ("1 > 2", "false"),
            ("1 < 1", "false"),
            ("1 > 1", "false"),
            ("1 == 1", "true"),
            ("1 != 1", "false"),
            ("1 == 2", "false"),
            ("1 != 2", "true"),
            ("true == true", "true"),
            ("false == false", "true"),
            ("(1 < 2) == true", "true"),
            ("(1 < 2) == false", "false"),
            ("(1 > 2) == true", "false"),
            ("(1 > 2) == false", "true"),
        ];

        for (input, expected) in case {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let r = e.eval(program).unwrap();
            assert_eq!(r.to_string(), expected)
        }
    }
}

use rmonkey_ast::{
    operator::{Infix, Prefix},
    Expr, Program, Stmt,
};
use rmonkey_error::{eval_error::EvalErrorKind, RMonkeyError, Result};
use rmonkey_object::{scope::Scope, Object};

#[derive(Debug, Default)]
pub struct Evaluator {
    scope: Scope,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            scope: Default::default(),
        }
    }

    pub fn eval(&mut self, node: Program) -> Result<Object> {
        let mut result = Object::Null;
        for p in node.stmts.iter() {
            result = self.eval_stmt(p)?;
            if let Object::ReturnValue(val) = result {
                return Ok(*val);
            };
        }
        Ok(result)
    }

    fn eval_stmt(&mut self, node: &Stmt) -> Result<Object> {
        match node {
            Stmt::LetStmt { name, value } => {
                let ident = self.eval_expr(name)?;
                let value = self.eval_expr(value)?;
                self.scope.set(ident.to_string(), value);
                Ok(Object::Null)
            }
            Stmt::ReturnStmt(expr) => {
                let value = self.eval_expr(expr)?;
                Ok(Object::ReturnValue(Box::new(value)))
            }
            Stmt::ExprStmt(expr) => Ok(self.eval_expr(expr)?),
            Stmt::BlockStmt { stmts } => Ok(self.eval_block_stmt(stmts)?),
        }
    }

    fn eval_block_stmt(&mut self, stmts: &[Stmt]) -> Result<Object> {
        let mut result = Object::Null;
        for s in stmts.iter() {
            result = self.eval_stmt(s)?;
            // should not unwrap `RETURN_VALUE`
            if result.obj_type() == "RETURN_VALUE" {
                return Ok(result);
            }
        }
        Ok(result)
    }

    fn eval_expr(&mut self, node: &Expr) -> Result<Object> {
        match node {
            Expr::Ident(val) => Ok(self.eval_ident(val)?),
            Expr::IntLiteral(val) => Ok(Object::Int(*val)),
            Expr::BoolLiteral(val) => Ok(Object::Bool(*val)),
            Expr::If {
                condition,
                consequence,
                alternative,
            } => self.eval_if_expr(condition, consequence, alternative),
            Expr::PrefixExpr { op, right } => Ok(self.eval_prefix_expr(op, right)?),
            Expr::InfixExpr { left, right, op } => Ok(self.eval_infix_expr(op, left, right)?),
            Expr::Func { params, body } => todo!(),
            Expr::Call { callee, args } => todo!(),
        }
    }

    fn eval_ident(&self, ident: &String) -> Result<Object> {
        if let Some(val) = self.scope.get(ident.to_string()) {
            Ok(val)
        } else {
            Err(RMonkeyError::EvalError(EvalErrorKind::UncaughtRef {
                ident: ident.to_string(),
            }))
        }
    }

    fn eval_prefix_expr(&mut self, op: &Prefix, right: &Expr) -> Result<Object> {
        let right = self.eval_expr(right)?;
        match op {
            Prefix::Minus => Ok(self.eval_minus_operator_expr(right)?),
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

    fn eval_minus_operator_expr(&self, right: Object) -> Result<Object> {
        if let Object::Int(val) = right {
            Ok(Object::Int(-val))
        } else {
            Err(RMonkeyError::EvalError(
                rmonkey_error::eval_error::EvalErrorKind::UnknownPrefixOperator {
                    op: Prefix::Minus,
                    right,
                },
            ))
        }
    }

    fn eval_infix_expr(&mut self, op: &Infix, left: &Expr, right: &Expr) -> Result<Object> {
        let left = self.eval_expr(left)?;
        let right = self.eval_expr(right)?;
        match (&left, &right) {
            (Object::Int(left_val), Object::Int(right_val)) => {
                Ok(self.eval_int_infix_expr(op, left_val, right_val))
            }
            (Object::Bool(left_val), Object::Bool(right_val)) => match op {
                Infix::Eq => Ok(self.native_bool_to_bool_object(left_val == right_val)),
                Infix::NotEq => Ok(self.native_bool_to_bool_object(left_val != right_val)),
                _ => Err(RMonkeyError::EvalError(
                    EvalErrorKind::UnknownInfixOperator {
                        op: op.clone(),
                        left,
                        right,
                    },
                )),
            },
            _ => {
                if left.obj_type() != right.obj_type() {
                    Err(RMonkeyError::EvalError(EvalErrorKind::TypeMismatch {
                        op: op.clone(),
                        left,
                        right,
                    }))
                } else {
                    Err(RMonkeyError::EvalError(
                        EvalErrorKind::UnknownInfixOperator {
                            op: op.clone(),
                            left,
                            right,
                        },
                    ))
                }
            }
        }
    }

    fn eval_int_infix_expr(&self, op: &Infix, left: &i64, right: &i64) -> Object {
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

    fn eval_if_expr(
        &mut self,
        condition: &Expr,
        consequence: &Stmt,
        alt: &Option<Box<Stmt>>,
    ) -> Result<Object> {
        let cond = self.eval_expr(condition)?;
        if self.is_truthy(cond) {
            return self.eval_stmt(consequence);
        }
        match alt {
            Some(alt) => self.eval_stmt(alt),
            None => Ok(Object::Null),
        }
    }

    fn is_truthy(&self, obj: Object) -> bool {
        match obj {
            Object::Bool(val) => val,
            Object::Null => false,
            _ => true,
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
        ];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
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
    #[test]
    fn test_if_else_expr() {
        let case = [
            ("if(true){10}", "10"),
            ("if (false) { 10 }", "null"),
            ("if (5 * 5 + 10 > 34) { 99 } else { 100 }", "99"),
            ("if ((1000 / 2) + 250 * 2 == 1000) { 9999 }", "9999"),
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
    fn test_return_stmt() {
        let case = [
            ("return 10", "10"),
            ("return 2 * 5", "10"),
            (
                "if (10 > 1) {
                    if (10 > 1) { return 10; }
                    return 1;
                }",
                "10",
            ),
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
    fn test_let_statement() {
        let case = [
            ("let a = 5; a;", "5"),
            ("let a = 5 * 5; a;", "25"),
            ("let a = 5; let b = a; b;", "5"),
            ("let a = 5; let b = a; let c = a + b + 5; c;", "15"),
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
}

use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

use rmonkey_ast::{
    operator::{Infix, Prefix},
    Expr, Program, Stmt,
};
use rmonkey_error::{eval_error::EvalErrorKind, RMonkeyError, Result};
use rmonkey_object::builtin::builtins;
use rmonkey_object::{scope::Scope, Object};

#[derive(Debug, Default)]
pub struct Evaluator {
    env: Rc<RefCell<Scope>>,
    builtin: Rc<RefCell<HashMap<&'static str, Object>>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            env: Rc::new(RefCell::new(Scope::new())),
            builtin: Rc::new(RefCell::new(builtins())),
        }
    }

    pub fn set_val_to_env(&mut self, key: String, val: Object) {
        self.env.borrow_mut().set(key, val);
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        self.env.borrow_mut().get(key.to_string())
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
                let value = self.eval_expr(value)?;
                self.set_val_to_env(name.to_string(), value);
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
            Expr::StringLiteral(val) => Ok(Object::String(val.to_owned())),
            Expr::If {
                condition,
                consequence,
                alternative,
            } => self.eval_if_expr(condition, consequence, alternative),
            Expr::PrefixExpr { op, right } => Ok(self.eval_prefix_expr(op, right)?),
            Expr::InfixExpr { left, right, op } => Ok(self.eval_infix_expr(op, left, right)?),
            Expr::Func { params, body } => Ok(self.eval_func_literal(body, params)?),
            Expr::Call { callee, args } => Ok(self.eval_call_expr(callee, args)?),
            Expr::Array { elements } => {
                let mut elems: Vec<Object> = Vec::new();
                for e in elements.iter() {
                    elems.push(self.eval_expr(e)?);
                }
                Ok(Object::Array { elements: elems })
            }
            Expr::IndexExpr { left, index } => {
                let left = self.eval_expr(left)?;
                let index = self.eval_expr(index)?;
                self.eval_index_expr(left, index)
            }
            Expr::HashLiteral { pairs } => self.eval_hash_literal(pairs.to_vec()),
        }
    }

    fn eval_ident(&self, ident: &String) -> Result<Object> {
        if let Some(val) = self.get(&ident.to_string()) {
            return Ok(val);
        }

        if let Some(builtin) = self.builtin.borrow_mut().get(ident.as_str()) {
            return Ok(builtin.clone());
        }

        Err(RMonkeyError::EvalError(EvalErrorKind::UncaughtRef {
            ident: ident.to_string(),
        }))
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
                    right: right.obj_type().to_owned(),
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
                        left: left.obj_type().to_owned(),
                        right: right.obj_type().to_owned(),
                    },
                )),
            },
            (Object::String(left_val), Object::String(right_val)) => match op {
                Infix::Plus => Ok(Object::String(format!("{left_val}{right_val}"))),
                _ => Err(RMonkeyError::EvalError(
                    EvalErrorKind::UnknownInfixOperator {
                        op: op.clone(),
                        left: left.obj_type().to_owned(),
                        right: right.obj_type().to_owned(),
                    },
                )),
            },
            _ => {
                if left.obj_type() != right.obj_type() {
                    Err(RMonkeyError::EvalError(EvalErrorKind::TypeMismatch {
                        op: op.clone(),
                        left: left.obj_type().to_owned(),
                        right: right.obj_type().to_owned(),
                    }))
                } else {
                    Err(RMonkeyError::EvalError(
                        EvalErrorKind::UnknownInfixOperator {
                            op: op.clone(),
                            left: left.obj_type().to_owned(),
                            right: right.obj_type().to_owned(),
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

    fn eval_func_literal(&mut self, body: &Stmt, params: &[Expr]) -> Result<Object> {
        Ok(Object::Func {
            params: params.to_vec(),
            body: body.clone(),
            scope: Scope::new_enclosed_environment(Rc::clone(&self.env)),
        })
    }

    fn eval_call_expr(&mut self, callee: &Expr, args: &[Expr]) -> Result<Object> {
        let callee = self.eval_expr(callee)?;
        let args = self.eval_exprs(args)?;
        self.apply_func(callee, args)
    }

    fn eval_exprs(&mut self, exprs: &[Expr]) -> Result<Vec<Object>> {
        let mut result = Vec::new();
        for expr in exprs.iter() {
            result.push(self.eval_expr(expr)?);
        }
        Ok(result)
    }

    fn apply_func(&self, callee: Object, args: Vec<Object>) -> Result<Object> {
        if let Object::Func {
            params,
            ref body,
            scope,
        } = callee
        {
            let mut function_env = self.extend_func_env(params, args, scope);
            let result = function_env.eval_stmt(body).unwrap();
            if let Object::ReturnValue(value) = result {
                return Ok(*value);
            }
            return Ok(result);
        }

        if let Object::BuiltIn { func } = callee {
            return func(args);
        }

        Err(RMonkeyError::Custom("".to_string()))
    }

    /// create child scope and bind params name and actual given arg value.
    fn extend_func_env(
        &self,
        params: Vec<Expr>,
        args: Vec<Object>,
        parent_scope: Scope,
    ) -> Evaluator {
        let function_env = Evaluator {
            env: Rc::new(RefCell::new(parent_scope)),
            builtin: Rc::clone(&self.builtin),
        };
        // bind param and actual given arg.
        for (ident, arg) in params.iter().zip(args.iter()) {
            if let Expr::Ident(ident) = ident {
                function_env
                    .env
                    .borrow_mut()
                    .set(ident.to_string(), arg.clone());
            }
        }
        function_env
    }

    fn eval_index_expr(&mut self, left: Object, index: Object) -> Result<Object> {
        match (left, index) {
            (Object::Array { elements }, Object::Int(val)) => {
                self.eval_array_index_expr(elements, val)
            }
            (Object::Hash(pairs), ref index_obj) => self.eval_hash_index_expr(pairs, index_obj),
            _ => todo!(),
        }
    }

    fn eval_array_index_expr(&mut self, arr: Vec<Object>, index: i64) -> Result<Object> {
        let max = arr.len() as i64;

        if index < 0 || index > max {
            return Ok(Object::Null);
        }

        if let Some(array_val) = arr.get(index as usize) {
            return Ok(array_val.clone());
        }
        Ok(Object::Null)
    }

    fn eval_hash_literal(&mut self, pairs: Vec<(Expr, Expr)>) -> Result<Object> {
        let mut hash: HashMap<Object, Object> = HashMap::new();
        for (key, val) in pairs.iter() {
            let key = self.eval_expr(key)?;
            let value = self.eval_expr(val)?;

            hash.insert(key, value);
        }
        Ok(Object::Hash(hash))
    }

    fn eval_hash_index_expr(
        &mut self,
        pairs: HashMap<Object, Object>,
        index: &Object,
    ) -> Result<Object> {
        match pairs.get(index) {
            Some(val) => Ok(val.clone()),
            None => Ok(Object::Null),
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
            (r#"let hello = "world"; hello"#, "\"world\""),
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
    fn test_function_literal() {
        let case = [
            ("let identity = fn(x){ x; }; identity(5);", "5"),
            ("let identity = fn(x){ return x; }; identity(5);", "5"),
            ("let double = fn(x) { x * 2; }; double(5);", "10"),
            ("let add = fn(x, y){ x + y;}; add(5, 5);", "10"),
            ("let add = fn(x, y){ x + y;}; add(5 + 5, add(5, 5));", "20"),
            (
                "let newAdder = fn(x) { fn(y) { x + y } };let addTwo = newAdder(2);addTwo(3);",
                "5",
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
    fn test_builtin_string_len() {
        let case = [
            (r#"len("")"#, "0"),
            (r#"len("four")"#, "4"),
            (r#"len("hello world")"#, "11"),
            (
                r#"len(1)"#,
                "custom error: arg to `len` not supported, got INTEGER",
            ),
            (
                r#"len("one", "two")"#,
                "custom error: wrong number of args. got=2, want=1",
            ),
        ];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            match e.eval(program) {
                Ok(r) => assert_eq!(r.to_string(), *expected),
                Err(e) => assert_eq!(e.to_string(), *expected),
            }
        }
    }

    #[test]
    fn test_array_literal() {
        let case = [(r#"[1, 2 * 2, 3 + 3]"#, "[1, 4, 6]")];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            match e.eval(program) {
                Ok(r) => assert_eq!(r.to_string(), *expected),
                Err(e) => assert_eq!(e.to_string(), *expected),
            }
        }
    }

    #[test]
    fn test_index_literal() {
        let case = [
            (r#"[1, 2, 3][0]"#, "1"),
            (r#"[1, 2, 3][1]"#, "2"),
            (r#"[1, 2, 3][2]"#, "3"),
            ("let myArray = [1, 2, 3]; myArray[2];", "3"),
            ("[1, 2, 3][3]", "null"),
        ];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            match e.eval(program) {
                Ok(r) => assert_eq!(r.to_string(), *expected),
                Err(e) => assert_eq!(e.to_string(), *expected),
            }
        }
    }

    #[test]
    fn test_buitin() {
        let case = [
            (r#"len([1, 2, 3])"#, "3"),
            (r#"first([1, 2, 3])"#, "1"),
            (r#"last([1, 2, 3])"#, "3"),
            (r#"rest([1, 2, 3])"#, "[2, 3]"),
            (r#"push([1, 2, 3], 4)"#, "[1, 2, 3, 4]"),
            (
                r#"let map = fn(arr, f) {let iter = fn(arr, accumulated) {if (len(arr) == 0) { accumulated} else {iter(rest(arr), push(accumulated, f(first(arr)))); }};iter(arr, []); }; let a = [1, 2, 3, 4]; let double = fn(x) { x * 2 };  map(a, double); "#,
                "[2, 4, 6, 8]",
            ),
            (
                r#"let reduce = fn(arr, initial, f) { let iter = fn(arr, result) {if (len(arr) == 0) { result} else {iter(rest(arr), f(result, first(arr))); }};iter(arr, initial); };let sum = fn(arr) {reduce(arr, 0, fn(initial, el) { initial + el });}; sum([1, 2, 3, 4, 5]);"#,
                "15",
            ),
        ];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            match e.eval(program) {
                Ok(r) => assert_eq!(r.to_string(), *expected),
                Err(e) => assert_eq!(e.to_string(), *expected),
            }
        }
    }

    #[test]
    fn test_hash_string() {
        let case = [(r#"{"one": 10 - 9, "two": 5}"#, r#"{"one": 1, "two": 5}"#)];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            match e.eval(program) {
                Ok(r) => assert_eq!(r.to_string(), *expected),
                Err(e) => assert_eq!(e.to_string(), *expected),
            }
        }
    }

    #[test]
    fn test_hash_index_access() {
        let case = [
            (r#"let bob = {"one": 10 - 9, "two": 5}; bob["one"];"#, "1"),
            (r#"{"foo": 5}["foo"]"#, "5"),
            (r#"{"foo": 5}["bar"]"#, "null"),
            (r#"{}["bar"]"#, "null"),
        ];
        for (input, expected) in case.iter() {
            let mut e = Evaluator::new();
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            match e.eval(program) {
                Ok(r) => assert_eq!(r.to_string(), *expected),
                Err(e) => assert_eq!(e.to_string(), *expected),
            }
        }
    }
}

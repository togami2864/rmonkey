use rmonkey_ast::{
    operator::{Infix, Prefix},
    precedence::Precedence,
    *,
};

#[derive(Default)]
pub struct Formatter {
    indent: usize,
    column: usize,
}

impl Formatter {
    pub fn fmt(&mut self, ast: Program) -> String {
        let formatted_code = self.fmt_block_stmt(ast.stmts);
        formatted_code
    }

    fn fmt_stmt(&mut self, stmt: Stmt) -> String {
        match stmt {
            Stmt::LetStmt { name, value } => self.fmt_let_stmt(name, value),
            Stmt::ReturnStmt(_) => todo!(),
            Stmt::ExprStmt(expr) => format!("{};", self.fmt_expr(expr, Precedence::Lowest)),
            Stmt::BlockStmt { stmts } => self.fmt_block_stmt(stmts),
        }
    }
    fn fmt_block_stmt(&mut self, stmts: Vec<Stmt>) -> String {
        let mut formatted_code = String::new();

        for (i, stmt) in stmts.into_iter().enumerate() {
            formatted_code.push_str(&self.fmt_stmt(stmt).to_string());
        }
        formatted_code
    }
    fn fmt_let_stmt(&mut self, name: Expr, value: Expr) -> String {
        let name = self.fmt_ident_expr(name);
        let result = format!("let {name} = ");
        self.column += result.len();

        let expr = self.fmt_expr(value, Precedence::Lowest);
        format!("{result}{expr};")
    }

    fn fmt_expr(&mut self, expr: Expr, precedence: Precedence) -> String {
        match expr {
            Expr::Ident(_) => self.fmt_ident_expr(expr),
            Expr::IntLiteral(val) => self.fmt_int_literal(val),
            Expr::BoolLiteral(val) => self.fmt_bool_literal(val),
            Expr::StringLiteral(val) => self.fmt_string_literal(val),
            Expr::PrefixExpr { op, right } => self.fmt_prefix_expr(op, *right),
            Expr::InfixExpr { left, right, op } => {
                self.fmt_infix_expr(*left, *right, op, precedence)
            }
            Expr::If {
                condition,
                consequence,
                alternative,
            } => self.fmt_if_expr(*condition, *consequence, alternative),
            Expr::Func { params, body } => self.fmt_func_literal(params, *body),
            Expr::Call { callee, args } => self.fmt_call_expr(*callee, args),
            Expr::Array { elements } => self.fmt_array_literal(elements),
            Expr::IndexExpr { left, index } => self.fmt_index_expr_literal(left, index),
            Expr::HashLiteral { pairs } => todo!(),
        }
    }

    fn fmt_ident_expr(&mut self, name: Expr) -> String {
        if let Expr::Ident(ident) = name {
            self.column += ident.len();
            ident
        } else {
            unreachable!()
        }
    }

    fn fmt_prefix_expr(&mut self, op: Prefix, right: Expr) -> String {
        let right = self.fmt_expr(right, Precedence::Prefix);
        format!("{op}{right}")
    }

    fn fmt_infix_expr(
        &mut self,
        left: Expr,
        right: Expr,
        op: Infix,
        precedence: Precedence,
    ) -> String {
        let cur_precedence = Self::infix_to_precedence(&op);
        let l = self.fmt_expr(left, cur_precedence.clone());
        let r = self.fmt_expr(right, cur_precedence.clone());

        if precedence > cur_precedence {
            format!("({l} {op} {r})")
        } else {
            format!("{l} {op} {r}")
        }
    }

    fn infix_to_precedence(op: &Infix) -> Precedence {
        match op {
            Infix::Plus | Infix::Minus => Precedence::Sum,
            Infix::Asterisk | Infix::Slash => Precedence::Product,
            Infix::Lt => Precedence::LessGreater,
            Infix::Gt => Precedence::LessGreater,
            Infix::Eq | Infix::NotEq => Precedence::Equals,
        }
    }

    fn fmt_int_literal(&mut self, val: i64) -> String {
        let res = val.to_string();
        self.column += res.len();
        res
    }

    fn fmt_bool_literal(&mut self, val: bool) -> String {
        let res = val.to_string();
        self.column += res.len();
        res
    }

    fn fmt_string_literal(&mut self, val: String) -> String {
        let res = val.to_string();
        self.column += res.len();
        format!(r#""{res}""#)
    }

    fn fmt_func_literal(&mut self, params: Vec<Expr>, body: Stmt) -> String {
        let params = params
            .iter()
            .map(|p| self.fmt_expr(p.clone(), Precedence::Lowest))
            .collect::<Vec<String>>()
            .join(", ");
        let body = self.fmt_stmt(body);
        format!("fn({}) {{\n{body}\n}}", params.trim_end_matches(", "))
    }

    fn fmt_array_literal(&mut self, elements: Vec<Expr>) -> String {
        let elems: Vec<String> = elements
            .iter()
            .map(|e| self.fmt_expr(e.clone(), Precedence::Lowest))
            .collect();
        format!("[{}]", elems.join(", ").trim_end_matches(", "))
    }

    fn fmt_index_expr_literal(&mut self, left: Box<Expr>, index: Box<Expr>) -> String {
        let left = self.fmt_expr(*left, Precedence::Lowest);
        let index = self.fmt_expr(*index, Precedence::Lowest);
        format!("{left}[{index}]")
    }

    fn fmt_call_expr(&mut self, callee: Expr, args: Vec<Expr>) -> String {
        let callee = self.fmt_expr(callee, Precedence::Lowest);
        let args: Vec<String> = args
            .iter()
            .map(|a| self.fmt_expr(a.clone(), Precedence::Lowest))
            .collect();
        format!("{callee}({})", args.join(", ").trim_end_matches(", "))
    }

    fn fmt_if_expr(&mut self, cond: Expr, cons: Stmt, alt: Option<Box<Stmt>>) -> String {
        let cond = self.fmt_expr(cond, Precedence::Lowest);
        let cons = self.fmt_stmt(cons);

        let res = if let Some(alt) = alt {
            let alt = self.fmt_stmt(*alt);
            format!("if({cond}) {{\n{cons}\n}} else {{\n{alt}\n}}")
        } else {
            format!("if({cond}) {{\n{cons}\n}}")
        };
        res
    }
}

#[cfg(test)]
mod tests {
    use rmonkey_lexer::Lexer;
    use rmonkey_parser::Parser;

    use super::*;

    fn formatter(input: &str) -> String {
        let mut formatter = Formatter::default();
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        formatter.fmt(program)
    }

    #[test]
    fn test_literals() {
        let input = [
            (r#"40"#, r#"40;"#),
            (r#"400000"#, r#"400000;"#),
            (r#"true"#, r#"true;"#),
            (r#"false"#, r#"false;"#),
            (r#""foo""#, r#""foo";"#),
            (r#""bar    ""#, r#""bar    ";"#),
            (r#"[1,2,3,4]"#, r#"[1, 2, 3, 4];"#),
            (r#"[1,2   ,3,    4]"#, r#"[1, 2, 3, 4];"#),
            (r#"arr[0 ]"#, r#"arr[0];"#),
            (r#"some_func(left,right)"#, r#"some_func(left, right);"#),
            (
                r#"some_func( 0, 1,    3,      5, 4  )"#,
                r#"some_func(0, 1, 3, 5, 4);"#,
            ),
        ];
        for (input, expected) in input.into_iter() {
            let input = formatter(input);
            assert_eq!(input, expected);
        }
    }

    #[test]
    fn test_prefix() {
        let input = [
            ("! false ", "!false;"),
            ("!5 ", "!5;"),
            ("!! true", "!!true;"),
            (" ! ! false", "!!false;"),
            ("! !- 5", "!!-5;"),
            ("- 5", "-5;"),
            (" -   10", "-10;"),
        ];
        for (input, expected) in input.into_iter() {
            let input = formatter(input);
            assert_eq!(input, expected);
        }
    }

    #[test]
    fn test_infix() {
        let input = [
            ("5+5+5+5-10", "5 + 5 + 5 + 5 - 10;"),
            ("2*2*2*2*2", "2 * 2 * 2 * 2 * 2;"),
            ("2*(5+10)", "2 * (5 + 10);"),
            ("(5+10*2+15/3)*2+-10", "(5 + 10 * 2 + 15 / 3) * 2 + -10;"),
        ];
        for (input, expected) in input.into_iter() {
            let input = formatter(input);
            assert_eq!(input, expected);
        }
    }

    #[test]
    fn test_function_literal() {
        let input = [
            (
                "let identity=fn(x){x;}",
                "let identity = fn(x) {
x;
};",
            ),
            (
                "let double = fn(x){x*2;};",
                "let double = fn(x) {
x * 2;
};",
            ),
            (
                "let add = fn(   x,     y){x+y;};",
                "let add = fn(x, y) {
x + y;
};",
            ),
            (
                "let newAdder = fn(x){fn(y){x + y}};",
                "let newAdder = fn(x) {
fn(y) {
x + y;
};
};",
            ),
            ("let fibonacci = fn(x) {if (x == 0) {0;} else {if (x == 1) {1;}else {fibonacci(x - 1) + fibonacci(x - 2);}}};",
"let fibonacci = fn(x) {
  if (x == 0) {
    0;
  } else {
    if (x == 1) {
        1;
    } else {
fibonacci(x - 1) + fibonacci(x - 2);
    }
}
};")
        ];
        for (input, expected) in input.into_iter() {
            let input = formatter(input);
            assert_eq!(input, expected);
        }
    }

    #[test]
    fn test_if_expr() {
        let input = [
            (
                "if(true){10}",
                "if(true) {
10;
};",
            ),
            (
                "if (false) { 10 }",
                "if(false) {
10;
};",
            ),
            (
                "if (5 * 5 + 10 > 34) { 99 } else { 100 }",
                "if(5 * 5 + 10 > 34) {
99;
} else {
100;
};",
            ),
        ];
        for (input, expected) in input.into_iter() {
            let input = formatter(input);
            assert_eq!(input, expected);
        }
    }

    #[test]
    fn let_stmt() {
        let input = [
            (r#"let foo=40"#, r#"let foo = 40;"#),
            (r#"let foo=400000"#, r#"let foo = 400000;"#),
            (r#"let foo=-40"#, r#"let foo = -40;"#),
            (r#"let foo=-   40"#, r#"let foo = -40;"#),
            (r#"let foo=true"#, r#"let foo = true;"#),
            (r#"let foo=false"#, r#"let foo = false;"#),
            (r#"let foo="foo""#, r#"let foo = "foo";"#),
            (r#"let foo="bar    ""#, r#"let foo = "bar    ";"#),
        ];

        for (input, expected) in input.into_iter() {
            let input = formatter(input);
            assert_eq!(input, expected);
        }
    }
}

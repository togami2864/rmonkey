use rmonkey_ast::{precedence::Precedence, *};

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

    fn fmt_block_stmt(&mut self, stmts: Vec<Stmt>) -> String {
        let mut formatted_code = String::new();

        for (i, stmt) in stmts.into_iter().enumerate() {
            formatted_code.push_str(&self.fmt_stmt(stmt).to_string());
        }
        formatted_code
    }

    fn fmt_stmt(&mut self, stmt: Stmt) -> String {
        match stmt {
            Stmt::LetStmt { name, value } => self.fmt_let_stmt(name, value),
            Stmt::ReturnStmt(_) => todo!(),
            Stmt::ExprStmt(_) => todo!(),
            Stmt::BlockStmt { stmts } => todo!(),
        }
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
            Expr::If {
                condition,
                consequence,
                alternative,
            } => self.fmt_if_expr(condition, consequence, alternative),
            Expr::PrefixExpr { op, right } => todo!(),
            Expr::InfixExpr { left, right, op } => todo!(),
            Expr::Func { params, body } => todo!(),
            Expr::Call { callee, args } => todo!(),
            Expr::Array { elements } => todo!(),
            Expr::IndexExpr { left, index } => todo!(),
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

    fn fmt_if_expr(&mut self, cond: Box<Expr>, cons: Box<Stmt>, alt: Option<Box<Stmt>>) -> String {
        todo!()
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
    fn let_stmt() {
        let input = [
            (r#"let foo=40"#, r#"let foo = 40;"#),
            (r#"let foo=400000"#, r#"let foo = 400000;"#),
            // (r#"let foo=-40"#, r#"let foo = -40;"#),
            // (r#"let foo=-   40"#, r#"let foo = -40;"#),
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

use rmonkey_ast::{
    operator::{Infix, Prefix},
    precedence::Precedence,
    Expr, Program, Stmt,
};
use rmonkey_error::{RMonkeyError, Result};
use rmonkey_lexer::Lexer;
use rmonkey_token::Token;

#[derive(Debug)]
pub struct Parser<'a> {
    l: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(l: Lexer<'a>) -> Self {
        let mut parser = Self {
            l,
            cur_token: Token::Illegal,
            peek_token: Token::Illegal,
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn parse(&mut self) {
        match self.parse_program() {
            Ok(p) => {
                for p in p.stmts.iter() {
                    println!("{p:?}");
                }
            }
            Err(err) => eprintln!("{err}"),
        }
    }

    fn next_token(&mut self) -> Token {
        let cur = self.cur_token.clone();
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
        cur
    }

    /// check if the current token equals to given Token
    fn cur_token_is(&mut self, t: Token) -> bool {
        self.cur_token == t
    }

    /// check if the peek token equals to given Token
    fn peek_token_is(&mut self, t: Token) -> bool {
        self.peek_token == t
    }

    /// if `peek_token_is` is true, consume the token and return true.
    fn expect_peek(&mut self, t: Token) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut stmts: Vec<Stmt> = vec![];
        while self.cur_token != Token::Eof {
            stmts.push(self.parse_stmt()?);
            self.next_token();
        }
        Ok(Program::new(stmts))
    }

    fn parse_stmt(&mut self) -> Result<Stmt> {
        match self.cur_token {
            Token::Let => Ok(self.parse_let_stmt()?),
            Token::Return => Ok(self.parse_return_stmt()?),
            _ => Ok(self.parse_expr_stmt()?),
        }
    }

    fn parse_let_stmt(&mut self) -> Result<Stmt> {
        // consume `let`
        self.next_token();
        let ident = match &self.cur_token {
            Token::Ident(val) => val.to_owned(),
            tok => {
                return Err(RMonkeyError::UnexpectedToken {
                    expected: Token::Ident("Ident".to_string()),
                    got: tok.clone(),
                });
            }
        };
        if !self.expect_peek(Token::Assign) {
            return Err(RMonkeyError::UnexpectedToken {
                expected: Token::Assign,
                got: self.peek_token.clone(),
            });
        }
        // consume `=`
        self.next_token();

        let value = self.parse_expr(Precedence::Lowest)?;
        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }

        Ok(Stmt::LetStmt {
            name: Expr::Ident(ident),
            value,
        })
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt> {
        // consume `return keyword`
        self.next_token();

        let return_val = self.parse_expr(Precedence::Lowest)?;

        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }
        Ok(Stmt::ReturnStmt(return_val))
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt> {
        let expr = self.parse_expr(Precedence::Lowest)?;
        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }
        Ok(Stmt::ExprStmt(expr))
    }

    /// should call this if `self.cur_token == Token::LBrace`
    fn parse_block_stmt(&mut self) -> Result<Stmt> {
        self.next_token();
        let mut stmts: Vec<Stmt> = Vec::new();
        while !self.cur_token_is(Token::RBrace) && !self.cur_token_is(Token::Eof) {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
            self.next_token();
        }
        Ok(Stmt::BlockStmt { stmts })
    }

    /// The function begins with the token associated with the syntax parsing function set to curToken.
    /// It then proceeds until the last token of the expression being processed is set to curToken.
    fn parse_expr(&mut self, prec: Precedence) -> Result<Expr> {
        let mut left = match &self.cur_token {
            Token::Ident(val) => self.parse_identifier(val.to_owned())?,
            Token::Int(val) => self.parse_integer_literal(val.to_owned())?,
            Token::True | Token::False => self.parse_bool_literal()?,
            Token::String(val) => self.parse_string_literal(val.to_owned())?,
            Token::Bang | Token::Minus => self.parse_prefix_expr()?,
            Token::LParen => self.parse_grouped_expr()?,
            Token::If => self.parse_if_expr()?,
            Token::Function => self.parse_func_literal()?,
            Token::LBracket => self.parse_array_literal()?,
            Token::LBrace => self.parse_hash_literal()?,
            _ => {
                return Err(RMonkeyError::Custom(
                    "failed to parse expression".to_string(),
                ));
            }
        };

        while !self.peek_token_is(Token::Semicolon) && prec < self.peek_token.cur_precedence() {
            self.next_token();
            left = match self.cur_token {
                Token::LParen => self.parse_call_expr(left)?,
                Token::LBracket => self.parse_index_expr(left)?,
                _ => self.parse_infix_expr(left)?,
            }
        }
        Ok(left)
    }

    fn parse_identifier(&mut self, val: String) -> Result<Expr> {
        Ok(Expr::Ident(val))
    }

    fn parse_integer_literal(&mut self, val: i64) -> Result<Expr> {
        Ok(Expr::IntLiteral(val))
    }

    fn parse_bool_literal(&mut self) -> Result<Expr> {
        let bool = self.cur_token_is(Token::True);
        Ok(Expr::BoolLiteral(bool))
    }

    fn parse_string_literal(&mut self, val: String) -> Result<Expr> {
        Ok(Expr::StringLiteral(val))
    }

    fn parse_grouped_expr(&mut self) -> Result<Expr> {
        // consume `(`
        self.next_token();
        let expr = self.parse_expr(Precedence::Lowest);
        if !self.expect_peek(Token::RParen) {
            return Err(RMonkeyError::UnexpectedToken {
                expected: Token::RParen,
                got: self.peek_token.clone(),
            });
        }
        expr
    }

    fn parse_if_expr(&mut self) -> Result<Expr> {
        if !self.expect_peek(Token::LParen) {
            return Err(RMonkeyError::UnexpectedToken {
                expected: Token::LParen,
                got: self.peek_token.clone(),
            });
        };
        // consume `(`
        self.next_token();

        let condition = self.parse_expr(Precedence::Lowest)?;
        if !self.expect_peek(Token::RParen) {
            return Err(RMonkeyError::UnexpectedToken {
                expected: Token::RParen,
                got: self.peek_token.clone(),
            });
        };
        // consume `{`
        self.next_token();

        let consequence = self.parse_block_stmt()?;

        if self.expect_peek(Token::Else) {
            // consume `{`
            self.next_token();
            let alt = self.parse_block_stmt()?;
            return Ok(Expr::If {
                condition: Box::new(condition),
                consequence: Box::new(consequence),
                alternative: Some(Box::new(alt)),
            });
        }

        Ok(Expr::If {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative: None,
        })
    }

    fn parse_func_literal(&mut self) -> Result<Expr> {
        if !self.expect_peek(Token::LParen) {
            return Err(RMonkeyError::UnexpectedToken {
                expected: Token::LParen,
                got: self.peek_token.clone(),
            });
        }

        let params = self.parse_func_params()?;
        self.next_token();
        let body = self.parse_block_stmt()?;
        Ok(Expr::Func {
            params,
            body: Box::new(body),
        })
    }

    fn parse_func_params(&mut self) -> Result<Vec<Expr>> {
        if self.peek_token_is(Token::RParen) {
            self.next_token();
            return Ok(Vec::new());
        }

        // consume `(`
        self.next_token();
        let mut params: Vec<Expr> = Vec::new();
        let first_param = match &self.cur_token {
            Token::Ident(val) => Expr::Ident(val.to_owned()),
            _ => {
                return Err(RMonkeyError::UnexpectedToken {
                    expected: Token::Ident("Ident".to_string()),
                    got: self.cur_token.clone(),
                })
            }
        };
        params.push(first_param);

        while self.peek_token_is(Token::Comma) {
            self.next_token();
            self.next_token();
            match &self.cur_token {
                Token::Ident(val) => {
                    params.push(Expr::Ident(val.to_owned()));
                }
                _ => {
                    return Err(RMonkeyError::UnexpectedToken {
                        expected: Token::Ident("Ident".to_string()),
                        got: self.cur_token.clone(),
                    })
                }
            }
        }

        if !self.expect_peek(Token::RParen) {
            return Err(RMonkeyError::UnexpectedToken {
                expected: Token::RParen,
                got: self.peek_token.clone(),
            });
        }

        Ok(params)
    }

    fn parse_call_expr(&mut self, func: Expr) -> Result<Expr> {
        let args = self.parse_call_args()?;
        Ok(Expr::Call {
            callee: Box::new(func),
            args,
        })
    }

    fn parse_call_args(&mut self) -> Result<Vec<Expr>> {
        if self.peek_token_is(Token::RParen) {
            self.next_token();
            return Ok(Vec::new());
        }

        // consume `(`
        self.next_token();
        let mut args: Vec<Expr> = Vec::new();
        let first_arg = self.parse_expr(Precedence::Lowest)?;
        args.push(first_arg);

        while self.peek_token_is(Token::Comma) {
            self.next_token();
            self.next_token();
            let arg = self.parse_expr(Precedence::Lowest)?;
            args.push(arg);
        }

        if !self.expect_peek(Token::RParen) {
            return Err(RMonkeyError::UnexpectedToken {
                expected: Token::RParen,
                got: self.peek_token.clone(),
            });
        }

        Ok(args)
    }

    fn parse_prefix_expr(&mut self) -> Result<Expr> {
        let op = match self.cur_token {
            Token::Minus => Prefix::Minus,
            Token::Bang => Prefix::Bang,
            _ => {
                return Err(RMonkeyError::InvalidPrefix {
                    got: self.cur_token.clone(),
                })
            }
        };

        self.next_token();
        let right = self.parse_expr(Precedence::Prefix)?;
        Ok(Expr::PrefixExpr {
            op,
            right: Box::new(right),
        })
    }

    fn parse_infix_expr(&mut self, left: Expr) -> Result<Expr> {
        let op = match self.cur_token {
            Token::Eq => Infix::Eq,
            Token::NotEq => Infix::NotEq,
            Token::Lt => Infix::Lt,
            Token::Gt => Infix::Gt,
            Token::Plus => Infix::Plus,
            Token::Minus => Infix::Minus,
            Token::Slash => Infix::Slash,
            Token::Asterisk => Infix::Asterisk,
            _ => return Err(RMonkeyError::Custom("invalid for infix".to_string())),
        };

        let precedence = self.cur_token.cur_precedence();
        self.next_token();
        let right = self.parse_expr(precedence)?;
        Ok(Expr::InfixExpr {
            left: Box::new(left),
            right: Box::new(right),
            op,
        })
    }

    fn parse_array_literal(&mut self) -> Result<Expr> {
        let elements = self.parse_expr_list(Token::RBracket)?;
        Ok(Expr::Array { elements })
    }

    fn parse_expr_list(&mut self, end: Token) -> Result<Vec<Expr>> {
        if self.peek_token_is(end.clone()) {
            self.next_token();
            return Ok(Vec::new());
        }

        self.next_token();
        let mut list: Vec<Expr> = Vec::new();
        let first_expr = self.parse_expr(Precedence::Lowest)?;
        list.push(first_expr);

        while self.peek_token_is(Token::Comma) {
            self.next_token();
            self.next_token();
            list.push(self.parse_expr(Precedence::Lowest)?)
        }

        if !self.expect_peek(end.clone()) {
            return Err(RMonkeyError::UnexpectedToken {
                expected: end,
                got: self.cur_token.clone(),
            });
        }

        Ok(list)
    }

    fn parse_index_expr(&mut self, left: Expr) -> Result<Expr> {
        self.next_token();
        let index = self.parse_expr(Precedence::Lowest)?;

        if !self.expect_peek(Token::RBracket) {
            return Err(RMonkeyError::UnexpectedToken {
                expected: Token::RBracket,
                got: self.cur_token.clone(),
            });
        }
        Ok(Expr::IndexExpr {
            left: Box::new(left),
            index: Box::new(index),
        })
    }

    fn parse_hash_literal(&mut self) -> Result<Expr> {
        let mut pairs: Vec<(Expr, Expr)> = Vec::new();
        while !self.peek_token_is(Token::RBrace) {
            self.next_token();
            let key = self.parse_expr(Precedence::Lowest)?;

            if !self.expect_peek(Token::Colon) {
                return Err(RMonkeyError::UnexpectedToken {
                    expected: Token::Colon,
                    got: self.cur_token.clone(),
                });
            }

            self.next_token();
            let value = self.parse_expr(Precedence::Lowest)?;

            pairs.push((key, value));

            if !self.peek_token_is(Token::RBrace) && !self.expect_peek(Token::Comma) {
                return Err(RMonkeyError::UnexpectedToken {
                    expected: Token::Colon,
                    got: self.cur_token.clone(),
                });
            }
        }

        if !self.expect_peek(Token::RBrace) {
            return Err(RMonkeyError::UnexpectedToken {
                expected: Token::RBrace,
                got: self.cur_token.clone(),
            });
        }

        Ok(Expr::HashLiteral { pairs })
    }
}

#[cfg(test)]
mod tests {
    use rmonkey_lexer::Lexer;

    use crate::Parser;

    #[test]
    fn test_literal_expressions() {
        let input = r#"
        true;
        false;
        "#;

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();

        assert_eq!(program.stmts.len(), 2);
    }

    #[test]
    fn test_return_stmt() {
        let input = r#"
        return 5;
        return 10;
        return 993322;
        "#;

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();

        assert_eq!(program.stmts.len(), 3);
    }

    #[test]
    fn test_string_literal() {
        let input = r#"
        let foo = "foo";
        let hello = "hello world";
        "#;
        let expected = vec![r#"let foo = "foo""#, r#"let hello = "hello world""#];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, p) in program.stmts.iter().enumerate() {
            assert_eq!(p.to_string(), expected[i]);
        }
    }

    #[test]
    fn test_infix_expression() {
        let input = "5 + 5;
        5 - 5;
        5 * 5;
        5 / 5;
        5 + 5 * 5;
        5 * 5 - 5 * 5 + 1;
        -a * b;
        !-a;
        a + b * c + d / e - f;
        5 > 4 == 3 < 4;
        5 < 4 != 3 > 4;
        3 + 4 * 5 == 3 * 1 + 4 * 5;
        ";
        let expected = vec![
            "(5 + 5)",
            "(5 - 5)",
            "(5 * 5)",
            "(5 / 5)",
            "(5 + (5 * 5))",
            "(((5 * 5) - (5 * 5)) + 1)",
            "((-a) * b)",
            "(!(-a))",
            "(((a + (b * c)) + (d / e)) - f)",
            "((5 > 4) == (3 < 4))",
            "((5 < 4) != (3 > 4))",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, p) in program.stmts.iter().enumerate() {
            assert_eq!(p.to_string(), expected[i]);
        }
    }

    #[test]
    fn test_if_expression() {
        let input = r#"if(x < y){x};
        if(a<b){a}else{b};"#;
        let expected = vec!["if((x < y)){x}", "if((a < b)){a}else{b}"];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, p) in program.stmts.iter().enumerate() {
            assert_eq!(p.to_string(), expected[i]);
        }
    }

    #[test]
    fn test_function_literal() {
        let input = r#"
        fn(x){x + 1};
        fn(x,y){x+y};
        fn(){1+1};
        fn() { return foobar + barfoo }
        fn() { return fn(x, y) { return x > y; }; }
        let myFunction = fn(x, y) { return x + y; }
        "#;
        let expected = vec![
            "fn(x){(x + 1)}",
            "fn(x, y){(x + y)}",
            "fn(){(1 + 1)}",
            "fn(){return (foobar + barfoo)}",
            "fn(){return fn(x, y){return (x > y)}}",
            "let myFunction = fn(x, y){return (x + y)}",
        ];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, p) in program.stmts.iter().enumerate() {
            assert_eq!(p.to_string(), expected[i]);
        }
    }

    #[test]
    fn test_call_expr() {
        let input = r#"add(1, 2 * 3, 4 + 5);
        a + add(b * c) + d;
        add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8));
        add(a + b + c * d / f + g);
        "#;
        let expected = vec![
            "add(1, (2 * 3), (4 + 5))",
            "((a + add((b * c))) + d)",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            "add((((a + b) + ((c * d) / f)) + g))",
        ];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, p) in program.stmts.iter().enumerate() {
            assert_eq!(p.to_string(), expected[i]);
        }
    }

    #[test]
    fn test_array_literal() {
        let input = r#"
        [1, 2 * 2, 3 + 3]
        myArray[1 + 1]
        a * [1, 2, 3, 4][b * c] * d
        add(a * b[2], b[1], 2 * [1, 2][1])
        "#;
        let expected = vec![
            "[1, (2 * 2), (3 + 3)]",
            "(myArray[(1 + 1)])",
            "((a * ([1, 2, 3, 4][(b * c)])) * d)",
            "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
        ];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, p) in program.stmts.iter().enumerate() {
            assert_eq!(p.to_string(), expected[i]);
        }
    }

    #[test]
    fn test_hash_literal() {
        let input = r#"
        {"one": 1, "two": 2, "three": 3}
        {}
        {"one": 0 + 1, "two": 10 - 8, "three": 15 / 5}
        "#;
        let expected = vec![
            r#"{"one": 1, "two": 2, "three": 3}"#,
            "{}",
            r#"{"one": (0 + 1), "two": (10 - 8), "three": (15 / 5)}"#,
        ];
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.stmts.len(), expected.len());
        for (i, p) in program.stmts.iter().enumerate() {
            assert_eq!(p.to_string(), expected[i]);
        }
    }
}

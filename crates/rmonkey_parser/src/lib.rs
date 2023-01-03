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

    fn parse_program(&mut self) -> Result<Program> {
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
        let ident = match self.next_token() {
            Token::Ident(val) => val,
            _ => return Err(RMonkeyError::UnexpectedTokenError),
        };
        if self.expect_peek(Token::Assign) {
            return Err(RMonkeyError::UnexpectedTokenError);
        }

        while !self.cur_token_is(Token::Semicolon) {
            self.next_token();
        }

        Ok(Stmt::LetStmt {
            name: Expr::Ident(ident),
            value: Expr::Ident("empty".to_owned()),
        })
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt> {
        // consume `return keyword`
        self.next_token();

        while !self.cur_token_is(Token::Semicolon) {
            self.next_token();
        }
        Ok(Stmt::ReturnStmt(Expr::Ident("empty".to_owned())))
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt> {
        let expr = self.parse_expr(Precedence::Lowest)?;
        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }
        Ok(Stmt::ExprStmt(expr))
    }

    /// The function begins with the token associated with the syntax parsing function set to curToken.
    /// It then proceeds until the last token of the expression being processed is set to curToken.
    fn parse_expr(&mut self, prec: Precedence) -> Result<Expr> {
        let mut left = match &self.cur_token {
            Token::Ident(val) => self.parse_identifier(val.to_owned())?,
            Token::Int(val) => self.parse_integer_literal(val.to_owned())?,
            Token::Bang | Token::Minus => self.parse_prefix_expr()?,
            t => {
                dbg!(t);
                return Err(RMonkeyError::UnexpectedTokenError);
            }
        };

        while !self.peek_token_is(Token::Semicolon) && prec < self.peek_token.cur_precedence() {
            self.next_token();
            left = self.parse_infix_expr(left)?;
        }
        Ok(left)
    }

    fn parse_identifier(&mut self, val: String) -> Result<Expr> {
        Ok(Expr::Ident(val))
    }

    fn parse_integer_literal(&mut self, val: u64) -> Result<Expr> {
        Ok(Expr::IntLiteral(val))
    }

    fn parse_prefix_expr(&mut self) -> Result<Expr> {
        let op = match self.cur_token {
            Token::Minus => Prefix::Minus,
            Token::Bang => Prefix::Bang,
            _ => return Err(RMonkeyError::UnexpectedTokenError),
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
            _ => return Err(RMonkeyError::UnexpectedTokenError),
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
}

#[cfg(test)]
mod tests {
    use rmonkey_lexer::Lexer;

    use crate::Parser;

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
    fn test_infix_expression() {
        let input = "
        -a * b;
        !-a;
        a + b * c + d / e - f;
        5 > 4 == 3 < 4;
        5 < 4 != 3 > 4;
        3 + 4 * 5 == 3 * 1 + 4 * 5;
        ";
        let expected = vec![
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
}

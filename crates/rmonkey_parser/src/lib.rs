use rmonkey_ast::{Expr, LetStmt, Program, Stmt};
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

    fn parse_program(&mut self) -> Result<Program, ()> {
        let mut stmts: Vec<Stmt> = vec![];
        while self.cur_token != Token::Eof {
            stmts.push(self.parse_stmt().unwrap());
            self.next_token();
        }
        Ok(Program::new(stmts))
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ()> {
        match self.cur_token {
            Token::Let => Ok(self.parse_let_stmt().unwrap()),
            _ => unimplemented!(),
        }
    }

    fn parse_let_stmt(&mut self) -> Result<Stmt, ()> {
        let ident = match self.next_token() {
            Token::Ident(val) => val,
            _ => todo!(),
        };
        if self.expect_peek(Token::Assign) {
            todo!()
        }

        while !self.cur_token_is(Token::Semicolon) {
            self.next_token();
        }

        Ok(Stmt::LetStmt(LetStmt {
            name: Expr::Ident(ident),
            value: Expr::Ident("empty".to_owned()),
        }))
    }
}

#[cfg(test)]
mod tests {}

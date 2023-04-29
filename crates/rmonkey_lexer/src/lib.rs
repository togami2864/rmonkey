use std::str::Chars;

use rmonkey_token::{look_up_ident, Token};

#[derive(Debug)]
pub struct Lexer<'a> {
    pub input: Chars<'a>,
    pub cur: char,
    pub peek: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            cur: '\u{0}',
            peek: '\u{0}',
        };
        lexer.read_char();
        lexer.read_char();
        lexer
    }

    pub fn tokenize(&mut self) {
        loop {
            let tok = self.next_token();
            println!("{} -> {}", tok, tok.name());
            if tok.eq(&Token::Eof) {
                break;
            }
        }
    }

    /// read next char of the input and return current char
    pub fn read_char(&mut self) -> char {
        let cur = self.cur;
        self.cur = self.peek;
        self.peek = self.input.next().unwrap_or('\u{0}');
        cur
    }

    /// return the current token and call `read_char()`
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.cur {
            '=' => {
                if self.peek == '=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '!' => {
                if self.peek == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            '>' => Token::Gt,
            '<' => Token::Lt,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            ':' => Token::Colon,
            '"' => self.read_string(),
            '\u{0}' => Token::Eof,
            c => {
                if is_letter(c) {
                    return self.read_identifier();
                } else if c.is_ascii_digit() {
                    return self.read_number();
                } else {
                    Token::Illegal
                }
            }
        };
        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while self.cur.is_whitespace() || self.cur == '\n' || self.cur == '\t' || self.cur == '\r' {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();
        while is_letter(self.cur) {
            ident.push(self.read_char());
        }
        if let Some(token) = look_up_ident(&ident) {
            return token;
        }
        Token::Ident(ident)
    }

    fn read_number(&mut self) -> Token {
        let mut value = String::new();
        while self.cur.is_ascii_digit() {
            value.push(self.read_char());
        }
        let value_i64 = value.parse::<i64>().unwrap();
        Token::Int(value_i64)
    }

    fn read_string(&mut self) -> Token {
        self.read_char();
        let mut value = String::new();
        while self.cur != '"' {
            let ch = self.read_char();
            if ch == '\u{0}' {
                return Token::Illegal;
            }
            value.push(ch);
        }
        Token::String(value)
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmonkey_token::Token;

    #[test]
    fn test_next_token() {
        let input = r#"=+(){},;"#;
        let tests = [
            (Token::Assign, "="),
            (Token::Plus, "+"),
            (Token::LParen, "("),
            (Token::RParen, ")"),
            (Token::LBrace, "{"),
            (Token::RBrace, "}"),
            (Token::Comma, ","),
            (Token::Semicolon, ";"),
            (Token::Eof, "Eof"),
        ];

        let mut l = Lexer::new(input);

        for (exp, exp_literal) in tests.iter() {
            let token = l.next_token();
            assert!(token.eq(exp));
            assert_eq!(token.to_string(), *exp_literal);
        }
    }

    #[test]
    fn test_variable_binding() {
        let input = r#"
        let five = 5;
        let ten = 10;
        let add = fn(x, y){
            x + y
        };
        let result = add(five, ten);"#;
        let tests = [
            (Token::Let, "let"),
            (Token::Ident("five".to_string()), "five"),
            (Token::Assign, "="),
            (Token::Int(5), "5"),
            (Token::Semicolon, ";"),
            (Token::Let, "let"),
            (Token::Ident("ten".to_string()), "ten"),
            (Token::Assign, "="),
            (Token::Int(10), "10"),
            (Token::Semicolon, ";"),
            (Token::Let, "let"),
            (Token::Ident("add".to_string()), "add"),
            (Token::Assign, "="),
            (Token::Function, "fn"),
            (Token::LParen, "("),
            (Token::Ident("x".to_string()), "x"),
            (Token::Comma, ","),
            (Token::Ident("y".to_string()), "y"),
            (Token::RParen, ")"),
            (Token::LBrace, "{"),
            (Token::Ident("x".to_string()), "x"),
            (Token::Plus, "+"),
            (Token::Ident("y".to_string()), "y"),
            (Token::RBrace, "}"),
            (Token::Semicolon, ";"),
            (Token::Let, "let"),
            (Token::Ident("result".to_string()), "result"),
            (Token::Assign, "="),
            (Token::Ident("add".to_string()), "add"),
            (Token::LParen, "("),
            (Token::Ident("five".to_string()), "five"),
            (Token::Comma, ","),
            (Token::Ident("ten".to_string()), "ten"),
            (Token::RParen, ")"),
            (Token::Semicolon, ";"),
            (Token::Eof, "Eof"),
        ];

        let mut l = Lexer::new(input);

        for (exp, exp_literal) in tests.iter() {
            let token = l.next_token();
            if token != *exp {
                panic!(
                    "assertion failed at {} => left: {}, right: {}",
                    l.cur, token, exp
                );
            }
            assert_eq!(token.to_string(), *exp_literal);
        }
    }
    #[test]
    fn test_op() {
        let input = r#"
!-/*5;
5 < 10 > 5;
        "#;

        let tests = [
            (Token::Bang, "!"),
            (Token::Minus, "-"),
            (Token::Slash, "/"),
            (Token::Asterisk, "*"),
            (Token::Int(5), "5"),
            (Token::Semicolon, ";"),
            (Token::Int(5), "5"),
            (Token::Lt, "<"),
            (Token::Int(10), "10"),
            (Token::Gt, ">"),
            (Token::Int(5), "5"),
        ];

        let mut l = Lexer::new(input);
        for (exp, exp_literal) in tests.iter() {
            let token = l.next_token();
            if token != *exp {
                panic!(
                    "assertion failed at {} => left: {}, right: {}",
                    l.cur, token, exp
                );
            }
            assert_eq!(token.to_string(), *exp_literal);
        }
    }
    #[test]
    fn test_if_statement() {
        let input = r#"if(5 < 10){
            return true;
        } else {
            return false;
        }"#;

        let tests = [
            (Token::If, "if"),
            (Token::LParen, "("),
            (Token::Int(5), "5"),
            (Token::Lt, "<"),
            (Token::Int(10), "10"),
            (Token::RParen, ")"),
            (Token::LBrace, "{"),
            (Token::Return, "return"),
            (Token::True, "true"),
            (Token::Semicolon, ";"),
            (Token::RBrace, "}"),
            (Token::Else, "else"),
            (Token::LBrace, "{"),
            (Token::Return, "return"),
            (Token::False, "false"),
            (Token::Semicolon, ";"),
            (Token::RBrace, "}"),
            (Token::Eof, "Eof"),
        ];

        let mut l = Lexer::new(input);
        for (exp, exp_literal) in tests.iter() {
            let token = l.next_token();
            if token != *exp {
                panic!(
                    "assertion failed at {} => left: {}, right: {}",
                    l.cur, token, exp
                );
            }
            assert_eq!(token.to_string(), *exp_literal);
        }
    }

    #[test]
    fn test_multiple() {
        let input = r#"
        10 == 10;
        10 != 9;
        "#;
        let tests = [
            (Token::Int(10), "10"),
            (Token::Eq, "=="),
            (Token::Int(10), "10"),
            (Token::Semicolon, ";"),
            (Token::Int(10), "10"),
            (Token::NotEq, "!="),
            (Token::Int(9), "9"),
            (Token::Semicolon, ";"),
            (Token::Eof, "Eof"),
        ];

        let mut l = Lexer::new(input);
        for (exp, exp_literal) in tests.iter() {
            let token = l.next_token();
            if token != *exp {
                panic!(
                    "assertion failed at {} => left: {}, right: {}",
                    l.cur, token, exp
                );
            }
            assert_eq!(token.to_string(), *exp_literal);
        }
    }

    #[test]
    fn test_string() {
        let input = r#"
        "foobar"
        "foo bar"
        "#;
        let tests = [
            (Token::String("foobar".to_owned()), r#"foobar"#),
            (Token::String("foo bar".to_owned()), r#"foo bar"#),
            (Token::Eof, "Eof"),
        ];

        let mut l = Lexer::new(input);
        for (exp, exp_literal) in tests.iter() {
            let token = l.next_token();
            if token != *exp {
                panic!(
                    "assertion failed at {} => left: {}, right: {}",
                    l.cur, token, exp
                );
            }
            assert_eq!(token.to_string(), *exp_literal);
        }
    }

    #[test]
    fn test_array_literal() {
        let input = r#"
        [1, 2];
        "#;
        let tests = [
            (Token::LBracket, "["),
            (Token::Int(1), "1"),
            (Token::Comma, ","),
            (Token::Int(2), "2"),
            (Token::RBracket, "]"),
            (Token::Semicolon, ";"),
            (Token::Eof, "Eof"),
        ];

        let mut l = Lexer::new(input);
        for (exp, exp_literal) in tests.iter() {
            let token = l.next_token();
            if token != *exp {
                panic!(
                    "assertion failed at {} => left: {}, right: {}",
                    l.cur, token, exp
                );
            }
            assert_eq!(token.to_string(), *exp_literal);
        }
    }

    #[test]
    fn test_hash_literal() {
        let input = r#"
        {"foo": "bar"}
        "#;
        let tests = [
            (Token::LBrace, "{"),
            (Token::String("foo".to_owned()), "foo"),
            (Token::Colon, ":"),
            (Token::String("bar".to_owned()), "bar"),
            (Token::RBrace, "}"),
            (Token::Eof, "Eof"),
        ];

        let mut l = Lexer::new(input);
        for (exp, exp_literal) in tests.iter() {
            let token = l.next_token();
            if token != *exp {
                panic!(
                    "assertion failed at {} => left: {}, right: {}",
                    l.cur, token, exp
                );
            }
            assert_eq!(token.to_string(), *exp_literal);
        }
    }

    #[test]
    fn test_illegal_unclosed_quote() {
        let input = r#"let foo = ""#;
        let tests = [
            (Token::Let, "let"),
            (Token::Ident("foo".to_owned()), "foo"),
            (Token::Assign, "="),
            (Token::Illegal, "Illegal"),
        ];

        let mut l = Lexer::new(input);
        for (exp, exp_literal) in tests.iter() {
            let token = l.next_token();
            if token != *exp {
                panic!(
                    "assertion failed at {} => left: {}, right: {}",
                    l.cur, token, exp
                );
            }
            assert_eq!(token.to_string(), *exp_literal);
        }
    }
}

use std::str::Chars;

use rmonkey_token::Token;

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

    /// read next char of the input and return current char
    pub fn read_char(&mut self) -> char {
        let cur = self.cur;
        self.cur = self.peek;
        self.peek = self.input.next().unwrap_or('\u{0}');
        cur
    }

    /// return the current token and call `read_char()`
    pub fn next_token(&mut self) -> Token {
        let token = match self.cur {
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '\u{0}' => Token::Eof,
            _ => Token::Illegal,
        };
        self.read_char();
        token
    }
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
}

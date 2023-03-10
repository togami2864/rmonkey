use phf::phf_map;
use rmonkey_ast::precedence::Precedence;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(i64),
    String(String),
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Function,
    Let,
    Bang,
    Lt,
    Gt,
    True,
    False,
    If,
    Else,
    Return,
    Eq,
    NotEq,
    Colon,
}

impl Token {
    pub fn cur_precedence(&self) -> Precedence {
        match self {
            Token::Eq => Precedence::Equals,
            Token::NotEq => Precedence::Equals,
            Token::Lt => Precedence::LessGreater,
            Token::Gt => Precedence::LessGreater,
            Token::Plus => Precedence::Sum,
            Token::Minus => Precedence::Sum,
            Token::Slash => Precedence::Product,
            Token::Asterisk => Precedence::Product,
            Token::LParen => Precedence::Call,
            Token::LBracket => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Token::Illegal => "Illegal",
            Token::Eof => "Eof",
            Token::Ident(_) => "Ident",
            Token::Int(_) => "Int",
            Token::Assign => "Assign",
            Token::Plus => "Plus",
            Token::Minus => "Minus",
            Token::Asterisk => "Asterisk",
            Token::Slash => "Slash",
            Token::Comma => "Comma",
            Token::Semicolon => "Semicolon",
            Token::LParen => "LParen",
            Token::RParen => "RParen",
            Token::LBrace => "LBrace",
            Token::RBrace => "RBrace",
            Token::LBracket => "LBracket",
            Token::RBracket => "RBracket",
            Token::Function => "Function",
            Token::Let => "Let",
            Token::Bang => "Bang",
            Token::Lt => "Lt",
            Token::Gt => "Gt",
            Token::True => "True",
            Token::False => "False",
            Token::If => "If",
            Token::Else => "Else",
            Token::Return => "Return",
            Token::Eq => "Eq",
            Token::NotEq => "NotEq",
            Token::String(_) => "STRING",
            Token::Colon => "Colon",
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Illegal => write!(f, "Illegal"),
            Token::Eof => write!(f, "Eof"),
            Token::Ident(val) => write!(f, "{val}"),
            Token::Int(val) => write!(f, "{val}"),
            Token::String(val) => write!(f, "{val}"),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::Function => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::Minus => write!(f, "-"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Bang => write!(f, "!"),
            Token::Lt => write!(f, "<"),
            Token::Gt => write!(f, ">"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
            Token::Eq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::Colon => write!(f, ":"),
        }
    }
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "let" => Token::Let,
    "fn" => Token::Function,
    "true" => Token::True,
    "false" => Token::False,
    "if" => Token::If,
    "else" => Token::Else,
    "return" => Token::Return
};

pub fn look_up_ident(ident: &str) -> Option<Token> {
    if let Some(tok) = KEYWORDS.get(ident) {
        return Some(tok.clone());
    }
    None
}

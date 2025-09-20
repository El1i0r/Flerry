// SPDX-License-Identifier: MIT
// Copyright (C) 2025 Affan Ahmad <st_iaffan@Outlook.com>

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub ttype: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    RSqBrace,
    LSqBrace,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    AmprSand,
    AmprAmprSand,
    Pipe,
    PipePipe,
    FatArrow,
    ThinArrow,

    // Literals
    Identifier,
    Strings,
    Integer,
    Float,
    Boolean,

    // Reserved Keywords
    Type,
    Struct,
    Enum,
    If,
    Else,
    Elsif,
    Return,
    While,
    For,
    Match,
    Func,
    End,

    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct TokenError {
    pub message: String,
    pub line: u32,
}

#[derive(Debug)]
pub enum TokenResult<'a> {
    Token(Token<'a>),
    Error(TokenError),
}

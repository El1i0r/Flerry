// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2025 Affan Ahmad <st_iaffan@Outlook.com>

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub ttype: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    RsqBrck,
    LsqBrck,
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
    Hash,

    // One or two character tokens
    Bang,
    BgEqual,
    Equal,
    EqEqual,
    Greater,
    GrEqual,
    Less,
    LsEqual,
    And,
    Or,
    AmprSand,
    AmprAmprSand,
    Pipe,
    PipePipe,

    // Literals
    Ident,
    String,
    Int,
    Float,
    Bool,
    Char,

    // Reserved Keywords
    If,
    Else,
    Elsif,
    Return,
    //Import,  (will implement modules later)
    While,
    For,
    Func,
    End,
    Print,
    Mut,
    Type,
    Enum,
    Struct,
    Is,

    Error,
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

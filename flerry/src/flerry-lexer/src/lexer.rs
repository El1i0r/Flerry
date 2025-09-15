// SPDX-License-Identifier: MIT
// Copyright (C) 2025 Affan Ahmad <st_iaffan@Outlook.com>
use flerry_core::{Token, TokenError, TokenResult, TokenType};

pub struct Lexer<'a> {
    start: usize,
    current: usize,
    line: u32,
    source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            start: 0,
            current: 0,
            line: 1,
            source,
        }
    }

    pub fn lex(&mut self) -> TokenResult {
        self.start = self.current;

        macro_rules! token {
            ($type:ident) => {
                TokenResult::Token(self.token(TokenType::$type))
            };
        }
        macro_rules! error {
            ($msg:expr) => {
                TokenResult::Error(self.error_token($msg.to_owned()))
            };
        }

        if self.is_at_end() {
            return token!(Eof);
        }

        let character = self.advance();

        match character {
            '(' => return token!(LParen),
            ')' => return token!(RParen),
            '{' => return token!(LBrace),
            '}' => return token!(RBrace),
            '[' => return token!(LSqBrace),
            ']' => return token!(RSqBrace),
            ',' => return token!(Comma),
            '.' => return token!(Dot),
            '+' => return token!(Plus),
            '/' => return token!(Slash),
            '*' => return token!(Star),
            '-' => return token!(Minus),
            '=' => return token!(Equal),
            _ => return error!("Error: unrecognized character."),
        }
    }

    // Token Functions
    pub fn token(&self, ttype: TokenType) -> Token<'a> {
        Token {
            ttype,
            lexeme: &self.source[self.start..self.current],
            line: self.line as usize,
        }
    }

    pub fn error_token(&self, msg: String) -> TokenError {
        TokenError {
            message: msg,
            line: self.line,
        }
    }

    // Check Functions
    // Helper Functions
    pub fn is_at_end(&self) -> bool {
        self.source.len() == self.current
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source.as_bytes()[self.current - 1] as char
    }

    pub fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        return Some(self.source.as_bytes()[self.current] as char);
    }

    pub fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            return None;
        }

        return Some(self.source.as_bytes()[self.current] as char);
    }

    pub fn cur_char(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.as_bytes()[self.current] as char;
    }

    pub fn skip_whitespaces(&mut self) {
        loop {
            match self.cur_char() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == Some('/') {
                        while self.cur_char() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }
}

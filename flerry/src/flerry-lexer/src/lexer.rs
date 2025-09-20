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
            '(' => token!(LParen),
            ')' => token!(RParen),
            '{' => token!(LBrace),
            '}' => token!(RBrace),
            '[' => token!(LSqBrace),
            ']' => token!(RSqBrace),
            ',' => token!(Comma),
            '.' => token!(Dot),
            '+' => token!(Plus),
            '/' => token!(Slash),
            '*' => token!(Star),
            '-' => {
                if self.peek() == Some('>') {
                    self.advance();
                    token!(ThinArrow)
                } else {
                    token!(Minus)
                }
            }
            '=' => {
                if self.peek() == Some('>') {
                    self.advance();
                    token!(FatArrow)
                } else {
                    self.check_next('=', TokenType::Equal, TokenType::EqualEqual)
                }
            }
            '!' => self.check_next('=', TokenType::Bang, TokenType::BangEqual),
            '<' => self.check_next('=', TokenType::Less, TokenType::LessEqual),
            '>' => self.check_next('=', TokenType::Greater, TokenType::GreaterEqual),
            '&' => self.check_next('&', TokenType::AmprSand, TokenType::AmprAmprSand),
            '|' => self.check_next('|', TokenType::Pipe, TokenType::PipePipe),
            '"' => self.handle_strings(),
            _ if self.is_digit(Some(character)) => self.handle_numbers(),
            _ if self.is_alpha(Some(character)) => self.handle_identifier(),
            _ => error!(format!("Error: unrecognized character {}", character)),
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

    // Literal handlers
    fn handle_strings(&mut self) -> TokenResult {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                let msg = format!("unterminated string at line {}.", self.line);
                return TokenResult::Error(self.error_token(msg));
            }
            self.advance();
        }

        if self.is_at_end() {
            let msg = format!("unterminated string at line {}.", self.line);
            return TokenResult::Error(self.error_token(msg));
        }

        // The closing quote.
        self.advance();
        return TokenResult::Token(self.token(TokenType::Strings));
    }

    fn handle_numbers(&mut self) -> TokenResult {
        let mut ttype: TokenType = TokenType::Integer;

        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == Some('.') && self.is_digit(self.peek_next()) {
            // Consume the ".".
            self.advance();
            ttype = TokenType::Float;
        }

        while self.is_digit(self.peek()) {
            self.advance();
        }

        return TokenResult::Token(self.token(ttype));
    }

    fn handle_identifier(&mut self) -> TokenResult {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let ttype = match text {
            "type" => TokenType::Type,
            "struct" => TokenType::Struct,
            "enum" => TokenType::Enum,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "elsif" => TokenType::Elsif,
            "return" => TokenType::Return,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "match" => TokenType::Match,
            "func" => TokenType::Func,
            "end" => TokenType::End,
            _ => TokenType::Identifier,
        };

        return TokenResult::Token(self.token(ttype));
    }
    // Check Functions
    fn is_digit(&self, c: Option<char>) -> bool {
        if c >= Some('0') && c <= Some('9') {
            return true;
        } else {
            return false;
        }
    }

    fn is_alpha(&self, c: Option<char>) -> bool {
        if c >= Some('a') && c <= Some('z') || c >= Some('A') && c <= Some('Z') || c == Some('_') {
            return true;
        } else {
            return false;
        }
    }

    fn is_alphanumeric(&self, c: Option<char>) -> bool {
        return self.is_digit(c) || self.is_alpha(c);
    }

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

        return Some(self.source.as_bytes()[self.current + 1] as char);
    }

    pub fn cur_char(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.as_bytes()[self.current] as char;
    }

    fn check_next(&mut self, expected: char, single: TokenType, double: TokenType) -> TokenResult {
        if self.peek() == Some(expected) {
            self.advance();
            TokenResult::Token(self.token(double))
        } else {
            TokenResult::Token(self.token(single))
        }
    }

    pub fn skip_whitespaces(&mut self) -> Option<TokenResult> {
        loop {
            match self.peek() {
                // Use peek() here to check without advancing
                Some(' ') | Some('\r') | Some('\t') => {
                    self.advance();
                }
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                }
                Some('#') => {
                    self.advance(); // Consume '#'
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                }
                Some('(') => {
                    if self.peek_next() == Some('*') {
                        self.advance(); // Consume '('
                        self.advance(); // Consume '*'
                        let mut comment_depth = 1;
                        while comment_depth > 0 && !self.is_at_end() {
                            if self.peek() == Some('(') && self.peek_next() == Some('*') {
                                self.advance(); // Consume '('
                                self.advance(); // Consume '*'
                                comment_depth += 1;
                            } else if self.peek() == Some('*') && self.peek_next() == Some(')') {
                                self.advance(); // Consume '*'
                                self.advance(); // Consume ')'
                                comment_depth -= 1;
                            } else if self.peek() == Some('\n') {
                                self.line += 1;
                                self.advance();
                            } else {
                                self.advance();
                            }
                        }
                        if comment_depth > 0 {
                            return Some(TokenResult::Error(
                                self.error_token("Unterminated multi-line comment".to_owned()),
                            ));
                        }
                        // Continue the loop to skip any further whitespaces or comments
                        continue;
                    } else {
                        return None; // Not a multi-line comment, let lex() handle it
                    }
                }
                _ => return None, // Not a whitespace or comment start, let lex() handle it
            }
        }
    }
}

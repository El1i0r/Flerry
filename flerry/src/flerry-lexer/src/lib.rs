// SPDX-License-Identifier: MIT
// Copyright (C) 2025 Affan Ahmad <st_iaffan@Outlook.com>

pub mod lexer;

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use flerry_core::{TokenResult, TokenType};

    #[test]
    fn test_single_char_tokens() -> Result<(), Box<dyn std::error::Error>> {
        let source = "()[]{} -";
        let mut lexer = Lexer::new(source);

        let expected_tokens = vec![
            (TokenType::LParen, "("),
            (TokenType::RParen, ")"),
            (TokenType::LSqBrace, "["),
            (TokenType::RSqBrace, "]"),
            (TokenType::LBrace, "{"),
            (TokenType::RBrace, "}"),
            (TokenType::Minus, "-"),
        ];

        for (expected_type, expected_lexeme) in expected_tokens {
            lexer.skip_whitespaces();
            match lexer.lex() {
                TokenResult::Token(token) => {
                    assert_eq!(token.ttype, expected_type);
                    assert_eq!(token.lexeme, expected_lexeme);
                }
                TokenResult::Error(err) => {
                    panic!("Lexer returned an error: {:?}", err);
                }
            }
        }
        Ok(())
    }

    #[test]
    fn test_identifiers_and_keywords() -> Result<(), Box<dyn std::error::Error>> {
        let source = "type struct enum if else elsif return while for match func end identifier_name anotherIdentifier";
        let mut lexer = Lexer::new(source);

        let expected_tokens = vec![
            (TokenType::Type, "type"),
            (TokenType::Struct, "struct"),
            (TokenType::Enum, "enum"),
            (TokenType::If, "if"),
            (TokenType::Else, "else"),
            (TokenType::Elsif, "elsif"),
            (TokenType::Return, "return"),
            (TokenType::While, "while"),
            (TokenType::For, "for"),
            (TokenType::Match, "match"),
            (TokenType::Func, "func"),
            (TokenType::End, "end"),
            (TokenType::Identifier, "identifier_name"),
            (TokenType::Identifier, "anotherIdentifier"),
        ];

        for (expected_type, expected_lexeme) in expected_tokens {
            lexer.skip_whitespaces();
            match lexer.lex() {
                TokenResult::Token(token) => {
                    assert_eq!(token.ttype, expected_type);
                    assert_eq!(token.lexeme, expected_lexeme);
                }
                TokenResult::Error(err) => {
                    panic!("Lexer returned an error: {:?}", err);
                }
            }
        }
        Ok(())
    }

    #[test]
    fn test_literals_and_multi_char_operators() -> Result<(), Box<dyn std::error::Error>> {
        let source = "123 45.67 \"hello world\" == != <= >= & && | ||";
        let mut lexer = Lexer::new(source);

        let expected_tokens = vec![
            (TokenType::Integer, "123"),
            (TokenType::Float, "45.67"),
            (TokenType::Strings, "\"hello world\""),
            (TokenType::EqualEqual, "=="),
            (TokenType::BangEqual, "!="),
            (TokenType::LessEqual, "<="),
            (TokenType::GreaterEqual, ">="),
            (TokenType::AmprSand, "&"),
            (TokenType::AmprAmprSand, "&&"),
            (TokenType::Pipe, "|"),
            (TokenType::PipePipe, "||"),
        ];

        for (expected_type, expected_lexeme) in expected_tokens {
            lexer.skip_whitespaces();
            match lexer.lex() {
                TokenResult::Token(token) => {
                    assert_eq!(token.ttype, expected_type);
                    assert_eq!(token.lexeme, expected_lexeme);
                }
                TokenResult::Error(err) => {
                    panic!("Lexer returned an error: {:?}", err);
                }
            }
        }
        Ok(())
    }

    #[test]
    fn test_eof_and_errors() -> Result<(), Box<dyn std::error::Error>> {
        let source = "\"unterminated string\n @";
        let mut lexer = Lexer::new(source);

        // Test unterminated string error
        match lexer.lex() {
            TokenResult::Error(err) => {
                assert!(err.message.contains("unterminated string"));
            }
            _ => panic!("Expected an error for unterminated string"),
        }

        // Test unrecognized character error
        lexer.advance(); // Move past the newline from the previous error
        lexer.skip_whitespaces();
        match lexer.lex() {
            TokenResult::Error(err) => {
                assert!(err.message.contains("unrecognized character"));
            }
            _ => panic!("Expected an error for unrecognized character"),
        }

        // Test EOF
        let source_eof = "";
        let mut lexer_eof = Lexer::new(source_eof);
        match lexer_eof.lex() {
            TokenResult::Token(token) => {
                assert_eq!(token.ttype, TokenType::Eof);
            }
            _ => panic!("Expected EOF token"),
        }

        Ok(())
    }

    #[test]
    fn test_arrows() -> Result<(), Box<dyn std::error::Error>> {
        let source = "=> ->";
        let mut lexer = Lexer::new(source);

        let expected_tokens = vec![(TokenType::FatArrow, "=>"), (TokenType::ThinArrow, "->")];

        for (expected_type, expected_lexeme) in expected_tokens {
            lexer.skip_whitespaces();
            match lexer.lex() {
                TokenResult::Token(token) => {
                    assert_eq!(token.ttype, expected_type);
                    assert_eq!(token.lexeme, expected_lexeme);
                }
                TokenResult::Error(err) => {
                    panic!("Lexer returned an error: {:?}", err);
                }
            }
        }
        Ok(())
    }

    #[test]
    fn test_comments() -> Result<(), Box<dyn std::error::Error>> {
        // Single line comment
        let source_single_line = "# This is a comment\n123";
        let mut lexer_single_line = Lexer::new(source_single_line);
        lexer_single_line.skip_whitespaces();
        match lexer_single_line.lex() {
            TokenResult::Token(token) => {
                assert_eq!(token.ttype, TokenType::Integer);
                assert_eq!(token.lexeme, "123");
            }
            _ => panic!("Expected integer after single line comment"),
        }

        // Multi-line comment
        let source_multi_line = "(* This is a multi-line comment *)\n456";
        let mut lexer_multi_line = Lexer::new(source_multi_line);
        lexer_multi_line.skip_whitespaces();
        match lexer_multi_line.lex() {
            TokenResult::Token(token) => {
                assert_eq!(token.ttype, TokenType::Integer);
                assert_eq!(token.lexeme, "456");
            }
            _ => panic!("Expected integer after multi-line comment"),
        }

        // Nested multi-line comment
        let source_nested_multi_line = "(* This is (* a nested *) multi-line comment *)\n789";
        let mut lexer_nested_multi_line = Lexer::new(source_nested_multi_line);
        lexer_nested_multi_line.skip_whitespaces();
        match lexer_nested_multi_line.lex() {
            TokenResult::Token(token) => {
                assert_eq!(token.ttype, TokenType::Integer);
                assert_eq!(token.lexeme, "789");
            }
            _ => panic!("Expected integer after nested multi-line comment"),
        }

        // Unterminated multi-line comment
        let source_unterminated = "(* This is an unterminated comment";
        let mut lexer_unterminated = Lexer::new(source_unterminated);
        match lexer_unterminated.skip_whitespaces() {
            Some(TokenResult::Error(err)) => {
                assert!(err.message.contains("Unterminated multi-line comment"));
            }
            _ => panic!("Expected error for unterminated multi-line comment"),
        }

        Ok(())
    }

    #[test]
    fn test_flerry_example() -> Result<(), Box<dyn std::error::Error>> {
        use std::fs; // Add this import

        let source = fs::read_to_string("../flerry-lexer/src/test.flerry")?;
        let mut lexer = Lexer::new(&source);

        let expected_tokens = vec![
            (TokenType::Func, "func"),
            (TokenType::Identifier, "hello"),
            (TokenType::LParen, "("),
            (TokenType::RParen, ")"),
            (TokenType::Equal, "="),
            (TokenType::Identifier, "print"),
            (TokenType::LParen, "("),
            (TokenType::Strings, "\"Hello, World\""),
            (TokenType::RParen, ")"),
            (TokenType::End, "end"),
            (TokenType::Func, "func"),
            (TokenType::Identifier, "add"),
            (TokenType::LParen, "("),
            (TokenType::Identifier, "x"),
            (TokenType::Comma, ","),
            (TokenType::Identifier, "y"),
            (TokenType::RParen, ")"),
            (TokenType::Equal, "="),
            (TokenType::Return, "return"),
            (TokenType::Identifier, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Identifier, "y"),
            (TokenType::End, "end"),
            (TokenType::Func, "func"),
            (TokenType::Identifier, "main"),
            (TokenType::LParen, "("),
            (TokenType::RParen, ")"),
            (TokenType::Equal, "="),
            (TokenType::Identifier, "hello"),
            (TokenType::LParen, "("),
            (TokenType::RParen, ")"),
            (TokenType::Identifier, "sum"),
            (TokenType::Equal, "="),
            (TokenType::Identifier, "add"),
            (TokenType::LParen, "("),
            (TokenType::Integer, "2"),
            (TokenType::Comma, ","),
            (TokenType::Integer, "4"),
            (TokenType::RParen, ")"),
            // The comment "(* result = 6 *)" should be skipped by skip_whitespaces()
            (TokenType::End, "end"),
            (TokenType::Eof, ""),
        ];

        for (expected_type, expected_lexeme) in expected_tokens {
            lexer.skip_whitespaces();
            match lexer.lex() {
                TokenResult::Token(token) => {
                    assert_eq!(
                        token.ttype, expected_type,
                        "Mismatch for token type. Expected {:?}, got {:?} for lexeme \"{}\"",
                        expected_type, token.ttype, token.lexeme
                    );
                    assert_eq!(
                        token.lexeme, expected_lexeme,
                        "Mismatch for lexeme. Expected \"{}\", got \"{}\" for token type {:?}",
                        expected_lexeme, token.lexeme, token.ttype
                    );
                }
                TokenResult::Error(err) => {
                    panic!("Lexer returned an error: {:?}", err);
                }
            }
        }
        Ok(())
    }
}

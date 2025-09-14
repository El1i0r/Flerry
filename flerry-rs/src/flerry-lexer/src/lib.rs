"""// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2025 Affan Ahmad <st_iaffan@Outlook.com>

pub mod lexer;
pub mod token;

#[cfg(test)]
mod tests {
    use colored::Colorize;

    use crate::lexer::Lexer;
    use crate::token::TokenType;

    #[test]
    fn test_single_char_tokens() -> Result<(), Box<dyn std::error::Error>> {
        println!("{} {}", "Debug:".magenta(), "flerry-lexer-0.0.1dev1
");
        let source = std::fs::read_to_string(
            "src/test.flerry",
        )?;
        let mut lexer = Lexer::new(source.as_str());

        let expected_tokens = vec![
            (TokenType::LParen, "("),
            (TokenType::RParen, ")"),
            (TokenType::LsqBrck, "["),
            (TokenType::RsqBrck, "]"),
            (TokenType::LBrace, "{"),
            (TokenType::RBrace, "}"),
            (TokenType::Minus, "-"),
        ];

        for (expected_type, expected_lexeme) in expected_tokens {
            lexer.skip_whitespaces();
            match lexer.lex() {
                crate::token::TokenResult::Token(token) => {
                    assert_eq!(token.ttype, expected_type);
                    println!(
                        "{} "{}" | {} "{}"
",
                        "ttype:".yellow(),
                        token.ttype,
                        "expttype:".green(),
                        expected_type,
                    );
                    assert_eq!(token.lexeme, expected_lexeme);
                    println!(
                        "{} "{}" | {} "{}"
",
                        "lexeme:".yellow(),
                        token.lexeme.magenta(),
                        "exptlexeme:".green(),
                        expected_lexeme.magenta(),
                    );
                }
                crate::token::TokenResult::Error(err) => {
                    panic!("Lexer returned an error: {:?}", err);
                }
            }
        }
        Ok(())
    }
}""

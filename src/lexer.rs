use crate::{Token, TokenType};
use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    src: Peekable<Chars<'a>>,
    current_pos: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer {
            src: src.chars().peekable(),
            current_pos: 0,
        }
    }

    // TODO: fix no whistepace problem (5+2 != 5 + 2)
    pub fn lex(mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(&ch) = self.src.peek() {
            let token_pos = self.current_pos;
            if ch.is_whitespace() {
                self.advance_src();
            } else if ch == '+' {
                self.advance_src();
                tokens.push(Token::new(TokenType::Plus, ch.to_string(), token_pos));
            } else if ch == '*' {
                self.advance_src();
                tokens.push(Token::new(TokenType::Times, ch.to_string(), token_pos));
            } else if ch == '-' {
                self.advance_src();
                tokens.push(Token::new(TokenType::Minus, ch.to_string(), token_pos));
            } else if ch.is_digit(10) {
                let number = self.scan(&|ch| ch.is_digit(10));
                tokens.push(Token::new(TokenType::Num, number, token_pos));
            } else if ch.is_alphabetic() {
                let text = self.scan(&|ch| ch.is_alphabetic());
                let token_type = match text.as_str() {
                    "false" => TokenType::False,
                    "true" => TokenType::True,
                    _ => TokenType::Identifier,
                };
                tokens.push(Token::new(token_type, text, token_pos));
            } else {
                return Err(format!(
                    "Unexpected character: {}, at position: {}",
                    ch, self.current_pos
                ));
            }
        }
        tokens.push(Token::new(
            TokenType::EOF,
            "<EOF>".to_string(),
            self.current_pos,
        ));
        Ok(tokens)
    }

    fn advance_src(&mut self) {
        self.current_pos += 1;
        self.src.next();
    }

    fn scan(&mut self, predicate: &dyn Fn(char) -> bool) -> String {
        let mut x = String::new();

        while let Some(ch) = self.src.next() {
            self.current_pos += 1;
            if !predicate(ch) {
                break;
            }
            x.push(ch);
        }
        x
    }
}

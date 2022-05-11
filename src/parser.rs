use crate::{Expr, ExprOpt, Term, TermOpt, Token, TokenType};
use std::iter::Peekable;

pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(mut self) -> Result<Expr, String> {
        let expr = self.expr()?;
        match self.tokens.next() {
            Some(token) => {
                if token.tok_type != TokenType::EOF {
                    return Err(format!(
                        "Unknown token `{}` at position: {}",
                        token.text, token.start_pos,
                    ));
                }
            }
            None => panic!("EOF not found"),
        }
        Ok(expr)
    }

    fn expr(&mut self) -> Result<Expr, String> {
        Ok(Expr::new(self.term()?, self.expr_opt()?))
    }

    fn expr_opt(&mut self) -> Result<Vec<ExprOpt>, String> {
        let mut exprs_opt = Vec::new();
        while let Some(token) = self.tokens.peek() {
            if token.tok_type != TokenType::Plus && token.tok_type != TokenType::Minus {
                break;
            }
            let op = if token.tok_type == TokenType::Plus {
                self.eat(TokenType::Plus)?.tok_type
            } else {
                self.eat(TokenType::Minus)?.tok_type
            };
            exprs_opt.push(ExprOpt::new(op, self.term()?));
        }
        Ok(exprs_opt)
    }

    fn term(&mut self) -> Result<Term, String> {
        let num_token = self.eat(TokenType::Num)?;
        let num = num_token
            .text
            .parse::<i32>()
            .map_err(|err| format!("unable to parse `{}`. parse error: {}", num_token.text, err))?;
        Ok(Term::new(num, self.term_opts()?))
    }

    fn term_opts(&mut self) -> Result<Vec<TermOpt>, String> {
        let mut opts = Vec::new();
        while let Some(token) = self.tokens.peek() {
            if token.tok_type != TokenType::Times {
                break;
            }
            self.eat(TokenType::Times)?;
            let num_token = self.eat(TokenType::Num)?;
            let num = num_token.text.parse::<i32>().map_err(|err| {
                format!("unable to parse `{}`. parse error: {}", num_token.text, err)
            })?;
            opts.push(TermOpt::new(num));
        }
        Ok(opts)
    }

    fn eat(&mut self, tok_type: TokenType) -> Result<Token, String> {
        match self.tokens.next() {
            Some(token) => {
                if token.tok_type != tok_type {
                    return Err(format!(
                        "Expected: `{:?}`, got: `{:?}` at position: {}",
                        tok_type, token.tok_type, token.start_pos,
                    ));
                }
                Ok(token)
            }
            None => panic!("Expected a token, none was found"),
        }
    }
}

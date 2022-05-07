use std::{
    io::{self, Write},
    iter::Peekable,
    str::Chars,
};

fn main() {
    let mut input = String::new();

    print!(" > ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let mut lexer = Lexer::new(&input);
    // let lex_result = lexer.lex().unwrap();
    let tokens = lexer.lex().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let mut interpreter = Interpreter::new(ast);
    println!("[interpreter]:\nresult: {}", interpreter.eval());
}

struct Lexer<'a> {
    src: Peekable<Chars<'a>>,
    current_pos: u32,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Lexer {
            src: src.chars().peekable(),
            current_pos: 0,
        }
    }

    fn lex(mut self) -> Result<Vec<Token>, String> {
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
            if predicate(ch) {
                x.push(ch);
            } else {
                break;
            }
        }
        x
    }
}

struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn parse(mut self) -> Result<Expr, String> {
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
            self.eat(TokenType::Times);
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

struct Interpreter {
    ast: Expr,
}

impl Interpreter {
    fn new(ast: Expr) -> Self {
        Interpreter { ast }
    }

    // TODO: fix the borrowing problem
    fn eval(mut self) -> i32 {
        let mut tmp = self.eval_term(&self.ast.term);
        let iterator = self.ast.expr_opt.into_iter();
        iterator.for_each(|expr_opt| {
            if expr_opt.op == TokenType::Plus {
                tmp += self.eval_term(&expr_opt.term);
            } else {
                tmp -= self.eval_term(&expr_opt.term);
            }
        });
        tmp
    }

    fn eval_term(&mut self, term: &Term) -> i32 {
        unimplemented!();
    }
}

#[derive(Debug)]
struct Expr {
    term: Term,
    expr_opt: Vec<ExprOpt>,
}

impl Expr {
    fn new(term: Term, expr_opt: Vec<ExprOpt>) -> Self {
        Expr { term, expr_opt }
    }
}

#[derive(Debug)]
struct ExprOpt {
    op: TokenType,
    term: Term,
}

impl ExprOpt {
    fn new(op: TokenType, term: Term) -> Self {
        ExprOpt { op, term }
    }
}

#[derive(Debug)]
struct Term {
    num: i32,
    term_opts: Vec<TermOpt>,
}

impl Term {
    fn new(num: i32, term_opts: Vec<TermOpt>) -> Self {
        Term { num, term_opts }
    }
}

#[derive(Debug)]
struct TermOpt {
    num: i32,
}

impl TermOpt {
    fn new(num: i32) -> Self {
        TermOpt { num }
    }
}

#[derive(Debug)]
struct Token {
    tok_type: TokenType,
    text: String,
    start_pos: u32,
}

impl Token {
    fn new(tok_type: TokenType, text: String, start_pos: u32) -> Self {
        Token {
            tok_type,
            text,
            start_pos,
        }
    }
}

#[derive(Debug, PartialEq)]
enum TokenType {
    Num,
    Plus,
    Minus,
    Times,
    Identifier,
    True,
    False,
    EOF,
}

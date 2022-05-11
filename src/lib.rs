pub mod interpreter;
pub mod lexer;
pub mod parser;

// ============
//   GRAMMAR
// ============

#[derive(Debug)]
pub struct Expr {
    term: Term,
    expr_opt: Vec<ExprOpt>,
}

impl Expr {
    pub fn new(term: Term, expr_opt: Vec<ExprOpt>) -> Self {
        Expr { term, expr_opt }
    }
}

#[derive(Debug)]
pub struct ExprOpt {
    op: TokenType,
    term: Term,
}

impl ExprOpt {
    pub fn new(op: TokenType, term: Term) -> Self {
        ExprOpt { op, term }
    }
}

#[derive(Debug)]
pub struct Term {
    num: i32,
    term_opts: Vec<TermOpt>,
}

impl Term {
    pub fn new(num: i32, term_opts: Vec<TermOpt>) -> Self {
        Term { num, term_opts }
    }
}

#[derive(Debug)]
pub struct TermOpt {
    num: i32,
}

impl TermOpt {
    pub fn new(num: i32) -> Self {
        TermOpt { num }
    }
}

// =================
//  TOKEN AND TYPES
// =================

#[derive(Debug)]
pub struct Token {
    tok_type: TokenType,
    text: String,
    start_pos: u32,
}

impl Token {
    pub fn new(tok_type: TokenType, text: String, start_pos: u32) -> Self {
        Token {
            tok_type,
            text,
            start_pos,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Num,
    Plus,
    Minus,
    Times,
    Identifier,
    True,
    False,
    EOF,
}

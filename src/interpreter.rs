use crate::{Expr, Term, TokenType};

pub struct Interpreter {
    ast: Expr,
}

impl Interpreter {
    pub fn new(ast: Expr) -> Self {
        Interpreter { ast }
    }

    pub fn eval(self) -> i32 {
        let mut tmp = self.eval_term(&self.ast.term);
        self.ast.expr_opt.iter().for_each(|expr_opt| {
            if expr_opt.op == TokenType::Plus {
                tmp += self.eval_term(&expr_opt.term);
            } else {
                tmp -= self.eval_term(&expr_opt.term);
            }
        });
        tmp
    }

    fn eval_term(&self, term: &Term) -> i32 {
        let mut tmp = term.num;
        term.term_opts
            .iter()
            .for_each(|term_opt| tmp *= term_opt.num);
        tmp
    }
}

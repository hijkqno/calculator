use thiserror::Error;
use crate::{
    parser::{Expr, ExprItem},
    lexer::{Token, Operator}
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Div zero")]
    DivZero
}

struct Storage {
    num: i64,
    last_operator: Option<Operator>
}

impl Storage {
    fn init() -> Self {
        Self {
            num: 0,
            last_operator: None
        }
    }
}

pub fn eval(expr: &Expr) -> Result<i64, Error> {
    let mut items = expr.items.iter().peekable();
    let mut storage: Storage = Storage::init();

    if let Some(ExprItem::Token(Token::Number(n))) = items.peek() {
        storage.num = *n;   
        items.next();
    }

    while let Some(&item) = items.peek() {
        match item {
            ExprItem::Token(token) => {
                match token {
                    Token::Number(n) => {
                        if let Some(o) = storage.last_operator {
                            match o {
                                Operator::Add => storage.num += n,
                                Operator::Sub => storage.num -= n,
                                Operator::Mul => storage.num *= n,
                                Operator::Div => {
                                    if *n == 0 {
                                        return Err(Error::DivZero);
                                    }
                                    storage.num /= n
                                }
                            }
                        } else { storage.num *= n; }
                        storage.last_operator = None;
                    },
                    Token::Operator(o) => {
                        storage.last_operator = Some(*o);
                    },
                    Token::LParen | Token::RParen => {}
                }
            },
            ExprItem::SubExpr(s) => {
                let sub_expr_result: i64 = eval(s)?;
                if let Some(o) = storage.last_operator {
                    match o {
                        Operator::Add => storage.num += sub_expr_result,
                        Operator::Sub => storage.num -= sub_expr_result,
                        Operator::Mul => storage.num *= sub_expr_result,
                        Operator::Div => {
                            if sub_expr_result == 0 {
                                return Err(Error::DivZero);
                            }
                            storage.num /= sub_expr_result
                        }
                    }
                } else { storage.num *= sub_expr_result; }
                storage.last_operator = None;
            }
        }
        items.next();
    }

    Ok(storage.num)
}

use std::{iter::Peekable, slice::Iter};
use thiserror::Error;
use crate::lexer::{Token, Operator};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Parentheses counter limit exceeded")]
    ParenthesesCounterLimitExceeded,

    #[error("Unexpected closing paranthesis")]
    UnexpectedClosingParanthesis,

    #[error("Not all parentheses were closed")]
    NotAllParenthesesWereClosed,

    // expressions
    #[error("An expression wasn't properly terminated")]
    UnterminatedExpression,

    #[error("Unexpected operator")]
    UnexpectedOperator,

    #[error("Unexpected number")]
    UnexpectedNumber,

}

fn check_parentheses(toks: &Vec<Token>) -> Result<(), Error> {
    let mut count: u8 = 0;

    for t in toks {
        match t {
            Token::LParen => {
                if count == (u8::MAX - 1) {
                    return Err(Error::ParenthesesCounterLimitExceeded);
                } count += 1;
            },
            Token::RParen => {
                if count == u8::MIN {
                    return Err(Error::UnexpectedClosingParanthesis);
                } count -= 1;
            }, _ => {}
        }
    }
    
    if count == 0 { Ok(()) }
    else { Err(Error::NotAllParenthesesWereClosed) }
}

#[derive(Debug, Clone)]
pub enum ExprItem {
    Token(Token),
    SubExpr(Box<Expr>)
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub items: Vec<ExprItem>,
    pub level: u8
}

enum ExpectedExprItem {
    Number,
    Operator,
    SubExpr
}

fn check_expression(expr: &Expr) -> Result<(), Error> {
    let mut last_item: Option<ExpectedExprItem> = None;
    let mut is_valid_end: bool = true;

    for i in &expr.items {
        match i {
            ExprItem::Token(t) => {
                match t {
                    Token::Number(_) => {
                        if let Some(ExpectedExprItem::Number) = last_item {
                            return Err(Error::UnexpectedNumber);
                        } 
                        last_item = Some(ExpectedExprItem::Number);
                        is_valid_end = true;
                    },
                    Token::Operator(op) => {
                        if let Some(ExpectedExprItem::Operator) = last_item {
                            return Err(Error::UnexpectedOperator);
                        }
                        
                        if let None = last_item && *op != Operator::Sub {
                            return Err(Error::UnexpectedOperator);
                        }

                        last_item = Some(ExpectedExprItem::Operator);
                        is_valid_end = false;
                    },
                    Token::LParen | Token::RParen => {}
                }
            },
            ExprItem::SubExpr(_) => {
                last_item = Some(ExpectedExprItem::SubExpr);
                is_valid_end = true;
            }
        }
    }
    
    if is_valid_end { Ok(()) }
    else {
        Err(Error::UnterminatedExpression)
    }
}

fn parse_expression(level: u8, toks: &mut Peekable<Iter<'_, Token>>) -> Result<Expr, Error> {
    let mut output: Expr = Expr {
        items: Vec::new(),
        level: level
    };

    while let Some(&tk) = toks.peek() {
        match tk {
            Token::LParen => {
                let _ = toks.next();
                let sub_expr: Expr = parse_expression(level + 1, toks)?;
                check_expression(&sub_expr)?;
                output.items.push(ExprItem::SubExpr(Box::new(sub_expr)));
            },
            Token::RParen => {
                toks.next();
                return Ok(output);
            },
            other => {
                output.items.push(ExprItem::Token(*other));
                toks.next();
            }
        }
    }

    Ok(output)

}

pub fn get_root_expression(toks: &Vec<Token>) -> Result<Expr, Error> {
    check_parentheses(toks)?;

    let mut iter: Peekable<Iter<'_, Token>> = toks.iter().peekable();
    let root_expression: Expr = parse_expression(0, &mut iter)?;
    check_expression(&root_expression)?;

    Ok(root_expression)
}

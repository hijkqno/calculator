use std::{iter::Peekable, slice::Iter};
use thiserror::Error;
use crate::lexer::Token;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Parentheses counter limit exceeded")]
    ParenthesesCounterLimitExceeded,

    #[error("Invalid closing paranthesis")]
    UnexpectedClosingParanthesis,

    #[error("Not all parentheses were closed")]
    NotAllParenthesesWereClosed,
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

fn parse_expression(level: u8, toks: &mut Peekable<Iter<'_, Token>>) -> Expr {
    let mut output: Expr = Expr {
        items: Vec::new(),
        level: level
    };

    while let Some(&tk) = toks.peek() {
        match tk {
            Token::LParen => {
                let _ = toks.next();
                let new_target: Expr = parse_expression(level + 1, toks);
                output.items.push(ExprItem::SubExpr(Box::new(new_target)));
            },
            Token::RParen => {
                toks.next();
                return output;
            },
            other => {
                output.items.push(ExprItem::Token(*other));
                toks.next();
            }
        }
    }

    output
}

fn parse_root_expression(toks: &Vec<Token>) -> Expr {
    let mut tokens: Peekable<Iter<'_, Token>> = toks.iter().peekable();

    let root: Expr = parse_expression(0, &mut tokens);

    root
}

pub fn get_root_expression(toks: &Vec<Token>) -> Result<Expr, Error> {
    check_parentheses(toks)?;

    let root: Expr = parse_root_expression(toks);

    Ok(root)
}


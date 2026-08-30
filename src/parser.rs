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

fn check_brackets(toks: &Vec<Token>) -> Result<(), Error> {
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
pub enum Element {
    Token(Token),
    Target(Box<Target>)
}

#[derive(Debug, Clone)]
pub struct Target {
    pub elements: Vec<Element>,
    pub level: u8
}

fn parse_target(level: u8, toks: &mut Peekable<Iter<'_, Token>>) -> Target {
    let mut output: Target = Target {
        elements: Vec::new(),
        level: level
    };

    while let Some(&tk) = toks.peek() {
        match tk {
            Token::LParen => {
                let _ = toks.next();
                let new_target: Target = parse_target(level + 1, toks);
                output.elements.push(Element::Target(Box::new(new_target)));
            },
            Token::RParen => {
                toks.next();
                return output;
            },
            other => {
                output.elements.push(Element::Token(*other));
                toks.next();
            }
        }
    }

    output
}

fn parse_root_target(toks: &Vec<Token>) -> Target {
    let mut tokens: Peekable<Iter<'_, Token>> = toks.iter().peekable();

    let root: Target = parse_target(0, &mut tokens);

    root
}

pub fn get_target(toks: &Vec<Token>) -> Result<Target, Error> {
    check_brackets(toks)?;

    let root: Target = parse_root_target(toks);

    Ok(root)
}


use thiserror::Error;
use crate::lexer::Token;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Bracket counter limit exceeded")]
    BracketCounterLimitExceeded,

    #[error("Invalid closed bracket")]
    InvalidClosedBracket,

    #[error("Not all brackets was closed")]
    NotAllBracketsWasClosed
}

fn check_brackets(toks: &Vec<Token>) -> Result<(), Error> {
    let mut count: u8 = 0;

    for t in toks {
        match t {
            Token::LeftBracket => {
                if count == (u8::MAX - 1) {
                    return Err(Error::BracketCounterLimitExceeded);
                } count += 1;
            },
            Token::RightBracket => {
                if count == u8::MIN {
                    return Err(Error::InvalidClosedBracket);
                } count -= 1;
            }, _ => {}
        }
    }
    
    if count == 0 { Ok(()) }
    else { Err(Error::NotAllBracketsWasClosed) }
}

#[derive(Debug, Clone)]
pub enum Element {
    Token(Token),
    Target(Box<Target>)
}

#[derive(Debug, Clone)]
pub struct Target {
    elements: Vec<Element>,
    level: u8
}

pub fn get_targets(toks: &Vec<Token>) -> Result<(/* Top Target of the tree*/), Error> {

    Ok(())
}


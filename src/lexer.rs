use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid character \'{0}\'")]
    InvalidChar(char),

    #[error("Failed to parse number: {0}")]
    NumberParse(ParseIntError),

}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add, Sub,
    Mul, Div
}

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(i64),
    Operator(Operator),
    LParen,
    RParen
}

pub fn get_tokens(str: &str) -> Result<Vec<Token>, Error> {
    let mut output: Vec<Token> = Vec::new();

    let mut chars = str.chars().peekable();
    while let Some(&char) = chars.peek() {
        if char.is_ascii_digit() {
            let mut numchrs: String = String::new();

            while let Some(&char) = chars.peek() {
                if !char.is_ascii_digit() { break; }
                numchrs.push(char);
                chars.next();
            }

            let num: i64 = numchrs
                .parse()
                .map_err(|e| Error::NumberParse(e))?;
            output.push(Token::Number(num));
            continue;
        }

        match char {
            ' ' | '\n' | '\t' => {},

            '+' => output.push(Token::Operator(Operator::Add)),
            '-' => output.push(Token::Operator(Operator::Sub)),
            '*' => output.push(Token::Operator(Operator::Mul)),
            '/' => output.push(Token::Operator(Operator::Div)),

            '(' => output.push(Token::LParen),
            ')' => output.push(Token::RParen),
        
            other => {
                return Err(Error::InvalidChar(other));
            }
        }
        chars.next();
    }

    Ok(output)
}

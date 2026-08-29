use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid character {0}")]
    InvalidChar(char),

    #[error("Failed to parse number: {0}")]
    NumberParseError(ParseIntError)

}

#[derive(Debug)]
pub enum Operator {
    Add, Sub,
    Mul, Div
}

#[derive(Debug)]
pub enum Token {
    Number(i64),
    Operator(Operator),
    LeftBracket,
    RightBracket
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
                .map_err(|e| Error::NumberParseError(e))?;
            output.push(Token::Number(num));
            continue;
        }

        match char {
            ' ' | '\n' | '\t' => {},

            '+' => output.push(Token::Operator(Operator::Add)),
            '-' => output.push(Token::Operator(Operator::Sub)),
            '*' => output.push(Token::Operator(Operator::Mul)),
            '/' => output.push(Token::Operator(Operator::Div)),

            '(' => output.push(Token::LeftBracket),
            ')' => output.push(Token::RightBracket),
        
            other => {
                return Err(Error::InvalidChar(other));
            }
        }
        chars.next();
    }

    Ok(output)
}

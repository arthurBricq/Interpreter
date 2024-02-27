use crate::error::TokenError;
use crate::error::TokenError::UnknownChar;
use crate::token::Op::{Div, Minus, Plus, Times};
use crate::token::Token::{Constant, Equal, Ident, LBracket, LPar, RBracket, Return, RPar, SemiColon, TokenOp};

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Op {
    Plus,
    Minus,
    Times,
    Div,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    TokenOp(Op),
    LPar, RPar,
    LBracket, RBracket,
    Ident(String),
    Return,
    Constant(i64),
    SemiColon,
    Equal,
}

pub fn tokenize(input: &String) -> Result<Vec<Token>, TokenError> {
    let mut tokens = vec![];

    let mut chars = input.chars();
    let mut ch = chars.next();

    loop {
        if ch.is_none() {
            break;
        }

        // Parse a number
        if let Some(mut num) = ch.unwrap().to_digit(10) {
            // if char is a digit, accumulate it
            ch = chars.next();
            while let Some(next_ch) = ch {
                if let Some(next_num) = next_ch.to_digit(10) {
                    num = 10 * num + next_num;
                    ch = chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Constant(num as i64));
            continue;
        }

        // Parse an word
        if ch.unwrap().is_alphabetic() {
            let mut tmp: String = ch.unwrap().to_string();
            ch = chars.next();
            while let Some(next_ch) = ch {
                if next_ch.is_alphanumeric() {
                    tmp.push(next_ch);
                    ch = chars.next();
                } else {
                    break;
                }
            }
            tokens.push(match tmp.as_str() {
                "return" => Return,
                &_ => Ident(tmp)
            });
            continue;
        }

        match ch.unwrap() {
            '+' => tokens.push(TokenOp(Plus)),
            '-' => tokens.push(TokenOp(Minus)),
            '/' => tokens.push(TokenOp(Div)),
            '*' => tokens.push(TokenOp(Times)),
            '(' => tokens.push(LPar),
            ')' => tokens.push(RPar),
            '{' => tokens.push(LBracket),
            '}' => tokens.push(RBracket),
            '=' => tokens.push(Equal),
            ';' => tokens.push(SemiColon),
            ' ' | '\r' | '\t' | '\n' => {}
            _ => {
                return Err(UnknownChar(ch.unwrap()))
            }
        }

        ch = chars.next();
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use crate::token::Op::{Div, Minus, Plus, Times};
    use crate::token::{tokenize, Token};

    use crate::token::Token::{Constant, Equal, Ident, LBracket, LPar, RBracket, Return, RPar, SemiColon, TokenOp};

    fn assert_tokens(text: &str, tokens: Vec<Token>) {
        let computed = tokenize(&text.to_string()).unwrap();
        assert_eq!(computed, tokens)
    }

    #[test]
    fn test_tokenizer() {
        assert_tokens(
            "+-*/",
            vec![TokenOp(Plus), TokenOp(Minus), TokenOp(Times), TokenOp(Div)],
        );
        assert_tokens(
            " + -      */    ",
            vec![TokenOp(Plus), TokenOp(Minus), TokenOp(Times), TokenOp(Div)],
        );
        assert_tokens(
            " (+) -      */    ",
            vec![
                LPar,
                TokenOp(Plus),
                RPar,
                TokenOp(Minus),
                TokenOp(Times),
                TokenOp(Div),
            ],
        );
        assert_tokens(
            "1+2-31",
            vec![
                Constant(1),
                TokenOp(Plus),
                Constant(2),
                TokenOp(Minus),
                Constant(31),
            ],
        );
        assert_tokens(
            "a = 1;",
            vec![Ident("a".to_string()), Equal, Constant(1), SemiColon],
        );
        assert_tokens("1+1", vec![Constant(1), TokenOp(Plus), Constant(1)]);
        assert_tokens(
            "1+1;",
            vec![Constant(1), TokenOp(Plus), Constant(1), SemiColon],
        );
        assert_tokens(
            "{1+1;}",
            vec![LBracket, Constant(1), TokenOp(Plus), Constant(1), SemiColon, RBracket],
        );
        assert_tokens(
            "return 1;",
            vec![Return, Constant(1), SemiColon],
        );

    }
}

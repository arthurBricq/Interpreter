use crate::error::TokenError;
use crate::error::TokenError::UnknownChar;
use crate::token::Op::{Div, Minus, Plus, Times};
use crate::token::Token::{Comma, Integer, Else, Equal, False, Fn, Ident, If, LBracket, LPar, RBracket, Return, RPar, SemiColon, TokenOp, True};

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Op {
    Plus,
    Minus,
    Times,
    Div,
    /// Comparison
    Equal,
    Lower,
    LowerEq,
    Higher,
    HigherEq
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    TokenOp(Op),
    Ident(String),
    Integer(i64),
    Equal,
    /// Symbols
    LPar, RPar,
    LBracket, RBracket,
    SemiColon,
    Comma,
    /// Keywords
    Return,
    Fn,
    If,
    Else,
    True,
    False
}

pub fn tokenize(input: &String) -> Result<Vec<Token>, TokenError> {
    let mut tokens = vec![];

    let mut chars = input.chars().peekable();
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
            tokens.push(Integer(num as i64));
            continue;
        }

        // Parse an word
        if ch.unwrap().is_alphabetic() || ch.unwrap() == '_' {
            let mut tmp: String = ch.unwrap().to_string();
            ch = chars.next();
            while let Some(next_ch) = ch {
                if next_ch.is_alphanumeric() || ch.unwrap() == '_' {
                    tmp.push(next_ch);
                    ch = chars.next();
                } else {
                    break;
                }
            }
            tokens.push(match tmp.as_str() {
                "return" => Return,
                "fn" => Fn,
                "if" => If,
                "else" => Else,
                "true" => True,
                "false" => False,
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
            '=' => {
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(TokenOp(Op::Equal))
                } else {
                    tokens.push(Equal)
                }
            },
            '<' => {
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(TokenOp(Op::LowerEq))
                } else {
                    tokens.push(TokenOp(Op::Lower))
                }
            }
            '>' => {
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(TokenOp(Op::HigherEq))
                } else {
                    tokens.push(TokenOp(Op::Higher))
                }
            }
            ';' => tokens.push(SemiColon),
            ',' => tokens.push(Comma),
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
    use crate::token::{tokenize, Token, Op};

    use crate::token::Token::{Integer, Equal, Ident, If, LBracket, LPar, RBracket, Return, RPar, SemiColon, TokenOp};

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
                Integer(1),
                TokenOp(Plus),
                Integer(2),
                TokenOp(Minus),
                Integer(31),
            ],
        );
        assert_tokens(
            "a = 1;",
            vec![Ident("a".to_string()), Equal, Integer(1), SemiColon],
        );
        assert_tokens("1+1", vec![Integer(1), TokenOp(Plus), Integer(1)]);
        assert_tokens(
            "1+1;",
            vec![Integer(1), TokenOp(Plus), Integer(1), SemiColon],
        );
        assert_tokens(
            "{1+1;}",
            vec![LBracket, Integer(1), TokenOp(Plus), Integer(1), SemiColon, RBracket],
        );
        assert_tokens(
            "return 1;",
            vec![Return, Integer(1), SemiColon],
        );
        assert_tokens(
            "if (1) { return 1; }",
            vec![If, LPar, Integer(1), RPar, LBracket, Return, Integer(1), SemiColon, RBracket],
        );
    }
    
    #[test]
    fn test_parse_double_char_operrators() {
        assert_tokens(
            "==",
            vec![TokenOp(Op::Equal)],
        );
        
        assert_tokens(
            "1 == 2",
            vec![Integer(1), TokenOp(Op::Equal), Integer(2)],
        );
        
        assert_tokens(
            "1 = 2",
            vec![Integer(1), Equal, Integer(2)],
        );
        
        assert_tokens(
            "1 < 2",
            vec![Integer(1), TokenOp(Op::Lower), Integer(2)],
        );
        
        assert_tokens(
            "1 <= 2",
            vec![Integer(1), TokenOp(Op::LowerEq), Integer(2)],
        );
    }
}

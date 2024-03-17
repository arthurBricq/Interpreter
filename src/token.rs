use crate::error::TokenError;
use crate::error::TokenError::UnknownChar;
use crate::token::Op::{Div, Minus, Plus, Times};
use crate::token::Token::{Break, Comma, Else, Equal, False, Fn, Ident, If, Integer, LBrace, LBracket, Loop, LPar, RBrace, RBracket, Return, RPar, SemiColon, TokenComp, TokenOp, True};

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Op {
    Plus,
    Minus,
    Times,
    Div,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Comp {
    Equal,
    Lower,
    LowerEq,
    Higher,
    HigherEq
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    TokenOp(Op),
    TokenComp(Comp),
    Ident(String),
    Integer(i64),
    String(String),
    Equal,
    /// Symbols
    LPar, RPar,
    LBrace, RBrace,
    LBracket, RBracket,
    SemiColon,
    Comma,
    /// Keywords
    Return,
    Fn,
    If,
    Else,
    True,
    False,
    Loop,
    Break
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
        
        // Parse a word
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
                "loop" => Loop,
                "break" => Break,
                &_ => Ident(tmp)
            });
            continue
        }
        
        // Parse a string
        if ch.unwrap() == '"' {
            let mut chars_in_string = vec![];
            while let Some(next_ch) = chars.next() {
                match next_ch { 
                    '"' => {
                        tokens.push(Token::String(chars_in_string.iter().collect()));
                        break
                    }
                    _ => chars_in_string.push(next_ch),
                }
            }
            ch = chars.next();
            continue
        }

        // Parse specific character
        match ch.unwrap() {
            '+' => tokens.push(TokenOp(Plus)),
            '-' => tokens.push(TokenOp(Minus)),
            '/' => {
                if let Some(&'/') = chars.peek() {
                    chars.next();
                    // If `//` is read, then skip until a break
                    while let Some(char) = chars.next() {
                        if char == '\n' {
                            break
                        }
                    }
                } else {
                    tokens.push(TokenOp(Div))
                }
            },
            '*' => tokens.push(TokenOp(Times)),
            '(' => tokens.push(LPar),
            ')' => tokens.push(RPar),
            '{' => tokens.push(LBrace),
            '}' => tokens.push(RBrace),
            '[' => tokens.push(LBracket),
            ']' => tokens.push(RBracket),
            '=' => {
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(TokenComp(Comp::Equal))
                } else {
                    tokens.push(Equal)
                }
            },
            '<' => {
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(TokenComp(Comp::LowerEq))
                } else {
                    tokens.push(TokenComp(Comp::Lower))
                }
            }
            '>' => {
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(TokenComp(Comp::HigherEq))
                } else {
                    tokens.push(TokenComp(Comp::Higher))
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
    use crate::token::{Comp, Token, tokenize};
    use crate::token::Op::{Div, Minus, Plus, Times};
    use crate::token::Token::{Equal, Ident, If, Integer, LBrace, LPar, RBrace, Return, RPar, SemiColon, TokenComp, TokenOp};

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
            vec![LBrace, Integer(1), TokenOp(Plus), Integer(1), SemiColon, RBrace],
        );
        assert_tokens(
            "return 1;",
            vec![Return, Integer(1), SemiColon],
        );
        assert_tokens(
            "if (1) { return 1; }",
            vec![If, LPar, Integer(1), RPar, LBrace, Return, Integer(1), SemiColon, RBrace],
        );
    }
    
    #[test]
    fn test_parse_double_char_operators() {
        assert_tokens(
            "==",
            vec![TokenComp(Comp::Equal)],
        );
        
        assert_tokens(
            "1 == 2",
            vec![Integer(1), TokenComp(Comp::Equal), Integer(2)],
        );
        
        assert_tokens(
            "1 = 2",
            vec![Integer(1), Equal, Integer(2)],
        );
        
        assert_tokens(
            "1 < 2",
            vec![Integer(1), TokenComp(Comp::Lower), Integer(2)],
        );
        
        assert_tokens(
            "1 <= 2",
            vec![Integer(1), TokenComp(Comp::LowerEq), Integer(2)],
        );
    }

    #[test]
    fn test_comments() {

        assert_tokens(
            "\
1 // Something
// Hello
2
// Bla-Bla
3
            ",
            vec![Integer(1), Integer(2), Integer(3)],
        );

    }
    #[test]
    fn test_string() {

        assert_tokens(
            "\"Hello world\"",
            vec![Token::String("Hello world".to_string())],
        );
        
        assert_tokens(
            "1 = \"Hello world\"",
            vec![Integer(1), Equal, Token::String("Hello world".to_string())],
        );

    }
}

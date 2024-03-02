pub mod expression;
pub mod statement;
pub mod declaration;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::parser::parse_expression;
    use crate::token::*;

    fn assert_ast_eval(text: &str, expected: i64) {
        let tokens = tokenize(&text.to_string());
        if let Ok(ast) = parse_expression(&tokens.unwrap()) {
            match ast.eval(&mut HashMap::new()) {
                Ok(value) => assert_eq!(value, expected),
                Err(_) => assert!(false),
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_ast_eval() {
        assert_ast_eval("1", 1);
        assert_ast_eval("1 + 1 + 1", 3);
        assert_ast_eval("1 * 1 * 1", 1);
        assert_ast_eval("1 + 2 * 3", 7);
        assert_ast_eval("2 * 3 + 1", 7);
        assert_ast_eval("2 * (3 + 1)", 8);
        assert_ast_eval("(2 * 3) + 1", 7);
        assert_ast_eval("1 + 1 + 1 + 1 + 1 + 1", 6);

        assert_ast_eval("-1", -1);
        assert_ast_eval("-1 + 1", 0);
        assert_ast_eval("-1 + 2 * 2", 3);
        assert_ast_eval("2 * 2 - 1", 3);
    }
}

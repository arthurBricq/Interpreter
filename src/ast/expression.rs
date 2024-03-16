use std::collections::HashMap;

use crate::ast::declaration::Declaration;
use crate::ast::expression::Expr::{AssignmentExpr, BinaryExpr, CompareExpr, ConstExpr, FunctionCall, IdentExpr, List, ListAccess, ParenthesisExpr};
use crate::ast::expression::Value::{BoolValue, IntValue};
use crate::ast::statement::StatementEval;
use crate::error::EvalError;
use crate::error::EvalError::{Error, MultipleError, UnknownVariable};
use crate::module::Module;
use crate::token::{Comp, Op};

/// A value is the result of an evaluation
/// It can be None, if there is no value
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Value {
    IntValue(i64),
    BoolValue(bool),
    List(Vec<Value>),
    None
}

/// An expression is something that evaluates to something
#[derive(Debug, Eq, PartialEq)]
pub enum Expr {
    ConstExpr(Value),
    NegExpr(Box<Expr>),
    ParenthesisExpr(Box<Expr>),
    BinaryExpr(Box<Expr>, Op, Box<Expr>),
    CompareExpr(Box<Expr>, Comp, Box<Expr>),
    AssignmentExpr(String, Box<Expr>),
    IdentExpr(String),
    FunctionCall(String, Vec<Expr>),
    List(Vec<Expr>),
    ListAccess(String, Box<Expr>),
}

impl Expr {
    /// Evaluates the expression
    /// buf: local variables (at the current scope)
    /// module: current evaluation module
    pub fn eval(&self, buf: &mut HashMap<String, Value>, module: Option<&Module>) -> Result<Value, EvalError> {
        match self {
            ConstExpr(value) => Ok(value.clone()),
            Expr::NegExpr(expr) => match expr.eval(buf, module) {
                Ok(IntValue(value)) => Ok(IntValue(-value)),
                Err(e) => Err(e),
                _ => Err(EvalError::Error("A negative express only applies to type Int and Float"))
            }
            ParenthesisExpr(expr) => expr.eval(buf, module),
            BinaryExpr(l, op, r) => match (l.eval(buf, module), r.eval(buf, module)) {
                (Ok(IntValue(l)), Ok(IntValue(r))) => Ok(IntValue(match op {
                    Op::Plus => l + r,
                    Op::Minus => l - r,
                    Op::Times => l * r,
                    Op::Div => l / r,
                })),
                (Ok(Value::List(values1)), Ok(Value::List(values2))) => {
                    if let Op::Plus = op {
                        let mut new_values = values1.clone();
                        for v in &values2 {
                            new_values.push(v.clone());
                        }
                        Ok(Value::List(new_values))
                    } else {
                        Err(Error("Only addition is supported for list"))
                    }
                }
                (Err(r), Ok(_)) => Err(r),
                (Ok(_), Err(err)) => Err(err),
                (Err(err1), Err(err2)) => Err(MultipleError(vec![Box::new(err1), Box::new(err2)])),
                _ => panic!("Binary operation not supported")
            }
            CompareExpr(l, cmp, r) => {
                match (l.eval(buf, module), r.eval(buf, module)) {
                    (Ok(left), Ok(right)) => Ok(Self::eval_compare_expr(&left, cmp, &right)),
                    (Err(r), _) => Err(r),
                    (_, Err(r)) => Err(r),
                }
            }
            AssignmentExpr(name, value) => {
                let eval = value.eval(buf, module);
                match eval {
                    Ok(value) => buf.insert(name.clone(), value.clone()),
                    _ => None
                };
                Ok(Value::None)
            }
            IdentExpr(name) => match buf.get(name) {
                Some(value) => Ok(value.clone()),
                None => Err(UnknownVariable(name.clone())),
            }
            FunctionCall(name, inputs) => {
                if module.is_none() {
                    return Err(EvalError::Error("Module not found"))
                }
                if let Some(Declaration::Function(_name, args, function_body)) =  module.unwrap().get_function(name) {
                    // We don't provide the function call with all the variables, but just with the provided arguments
                    // i. evaluate the inputs
                    let mut function_inputs = HashMap::new();
                    for i in 0..args.len() {
                        let arg_name = &args[i];
                        let arg_expr = &inputs[i];
                        if let Ok(value) = arg_expr.eval(buf, module) {
                            function_inputs.insert(arg_name.0.clone(), value);
                        }
                    }
                    
                    match function_body.eval(&mut function_inputs, module) {
                        Ok(StatementEval::Return(result)) => Ok(result),
                        Err(err) => Err(err),
                        _ => Ok(Value::None),
                    }
                } else {
                    Err(EvalError::Error("Function not found"))
                }
            }
            List(values) => {
                let mut to_return  = vec![];
                for value in values {
                    match value.eval(buf, module) {
                        Ok(result) => to_return.push(result),
                        Err(err) => return Err(err)
                    }
                }
                Ok(Value::List(to_return))
            }
            ListAccess(name, index) => {
                // Find the index where to look up
                let pos = match index.eval(buf, module) {
                    Ok(IntValue(pos)) => {
                        pos as usize
                    }
                    Err(err) => return Err(err),
                    _ => return Err(EvalError::Error("When accessing a list, the index must be of type int"))
                };
                
                // Find the value at this index
                match buf.get(name) {
                    Some(value) => {
                        match value {
                            Value::List(values) => {
                                let n = values.len();
                                Ok(values[pos].clone())
                            }
                            _ => Err(EvalError::Error("Only list can be accessed"))
                        }
                    }
                    None => Err(EvalError::UnknownVariable(name.clone()))
                }
            }
        }
    }

    fn eval_compare_expr(left: &Value, op: &Comp, right: &Value) -> Value {
        match op {
            Comp::Equal => BoolValue(left == right),
            Comp::Lower => BoolValue(left < right),
            Comp::LowerEq => BoolValue(left <= right),
            Comp::Higher => BoolValue(left > right),
            Comp::HigherEq => BoolValue(left >= right)
        }
    }

}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::ast::expression::{Expr, Value};
    use crate::ast::expression::Value::{BoolValue, IntValue, List};
    use crate::ast::statement::StatementEval;
    use crate::error::EvalError;
    use crate::parser::Parser;
    use crate::token::tokenize;

    fn assert_expression_evaluation(text: &str, expected: Result<Value, EvalError>) {
        let tokens = tokenize(&text.to_string()).unwrap();
        let mut parser = Parser::new(&tokens);
        let ast = parser.parse_expression().unwrap();
        let result = ast.eval(&mut HashMap::new(), None);
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_simple_bool_eval() {
        // test ==
        assert_expression_evaluation("1 == 1", Ok(BoolValue(true)));
        assert_expression_evaluation("1 == 2", Ok(BoolValue(false)));
        assert_expression_evaluation("2 == 1", Ok(BoolValue(false)));
        assert_expression_evaluation("2 == 2", Ok(BoolValue(true)));
        
        // test relational
        assert_expression_evaluation("1 > 1", Ok(BoolValue(false)));
        assert_expression_evaluation("1 < 1", Ok(BoolValue(false)));
        assert_expression_evaluation("2 > 1", Ok(BoolValue(true)));
        assert_expression_evaluation("2 < 1", Ok(BoolValue(false)));
        assert_expression_evaluation("1 >= 1", Ok(BoolValue(true)));
        assert_expression_evaluation("1 <= 1", Ok(BoolValue(true)));
        
    }
    
    #[test]
    fn test_bool_eval() {
        assert_expression_evaluation("1 + 1 == 2", Ok(BoolValue(true)));
        assert_expression_evaluation("(1 + 1) == 2", Ok(BoolValue(true)));
        assert_expression_evaluation("(1 + 1) == 2 * 1", Ok(BoolValue(true)));
        assert_expression_evaluation("(1 + 1) * 2 == 2 * 2", Ok(BoolValue(true)));
        assert_expression_evaluation("(1 + 1) * 2 + 2 == 6", Ok(BoolValue(true)));
    }
    
    #[test]
    fn test_list_eval() {
        let text = "[1,2,3]";
        let tokens = tokenize(&text.to_string()).unwrap();
        let mut parser = Parser::new(&tokens);
        let ast = parser.parse_expression().unwrap();
        let result = ast.eval(&mut HashMap::new(), None);
        println!("{result:?}");
        assert_eq!(result, Ok(List(vec![IntValue(1), IntValue(2), IntValue(3)])))
    }

    #[test]
    fn test_list_access_eval() {
        fn get_list_access_ast(at: usize) -> Expr {
            let text = format!("my_list[{at}]");
            let tokens = tokenize(&text.to_string()).unwrap();
            let mut parser = Parser::new(&tokens);
            let ast = parser.parse_expression().unwrap();
            ast
        }
        
        let mut data = HashMap::new();
        let my_list = List(vec![IntValue(1), IntValue(2), IntValue(3)]);
        data.insert("my_list".to_string(), my_list);
        
        assert_eq!(Ok(IntValue(1)), get_list_access_ast(0).eval(&mut data, None));
        assert_eq!(Ok(IntValue(2)), get_list_access_ast(1).eval(&mut data, None));
        assert_eq!(Ok(IntValue(3)), get_list_access_ast(2).eval(&mut data, None));
    }

    #[test]
    fn test_sum_of_list() {
        let text = "\
fn main() {
    a = [3];
    b = a + [1];
    return b
}
        ";
        let tokens = tokenize(&text.to_string()).unwrap();
        let mut parser = Parser::new(&tokens);
        let module = parser.parse_module();
        println!("{module:?}");
        let result = module.run();
        println!("{result:?}");
        assert_eq!(result, Ok(StatementEval::Return(List(vec![IntValue(3), IntValue(1)]))));
    }
    
}

//! # Evaluator Module
//!
//! This module implements the evaluation engine for the expression parser.
//! The evaluator takes an abstract syntax tree (AST) produced by the parser
//! and recursively evaluates it to produce a numeric result.
//!
//! The evaluator handles all arithmetic operations with proper error handling,
//! including division by zero detection.

use crate::error::EvalError;
use crate::expr::Expr;
use crate::token::Token;

/// Evaluates an expression and returns the result or an evaluation error.
///
/// This function recursively evaluates the abstract syntax tree (AST) produced
/// by the parser. It handles basic arithmetic operations and checks for errors
/// like division by zero.
///
/// # Arguments
///
/// * `expr` - A reference to the expression to evaluate.
///
/// # Returns
///
/// Returns `Ok(i64)` with the computed result, or `Err(EvalError)` if division
/// by zero is encountered.
///
/// # Example
///
/// ```
/// use crate::expr::Expr;
/// use crate::token::Token;
/// // eval(&Expr::BinaryOp { left: Box::new(...), op: Token::Add, right: Box::new(...) })
/// ```

pub fn eval(expr: &Expr) -> Result<i64, EvalError> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::BinaryOp { left, op, right } => {
            // eval left, eval right, apply op

            let left_val = eval(left)?;
            let right_val = eval(right)?;

            match op {
                Token::Add => Ok(left_val + right_val),
                Token::Sub => Ok(left_val - right_val),
                Token::Mul => Ok(left_val * right_val),
                Token::Div => {
                    if right_val == 0 {
                        Err(EvalError::DivisionByZero)
                    } else {
                        Ok(left_val / right_val)
                    }
                }
                _ => unreachable!(), // The parser should ensure that only valid operators are present
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn calculate(input: &str) -> Result<i64, EvalError> {
        let tokens = Lexer::new(input).tokenize();
        let expr = Parser::new(tokens).parse().unwrap();
        eval(&expr)
    }

    // --- Single Numbers ---

    #[test]
    fn test_single_zero() {
        assert_eq!(calculate("0"), Ok(0));
    }

    #[test]
    fn test_single_nine() {
        assert_eq!(calculate("9"), Ok(9));
    }

    // --- Addition ---

    #[test]
    fn test_simple_addition() {
        assert_eq!(calculate("3+4"), Ok(7));
    }

    #[test]
    fn test_addition_with_zero() {
        assert_eq!(calculate("5+0"), Ok(5));
    }

    #[test]
    fn test_addition_chain() {
        assert_eq!(calculate("1+2+3"), Ok(6));
    }

    // --- Subtraction ---

    #[test]
    fn test_simple_subtraction() {
        assert_eq!(calculate("5-3"), Ok(2));
    }

    #[test]
    fn test_subtraction_to_zero() {
        assert_eq!(calculate("5-5"), Ok(0));
    }

    #[test]
    fn test_subtraction_to_negative() {
        assert_eq!(calculate("3-9"), Ok(-6));
    }

    #[test]
    fn test_subtraction_chain() {
        // 9-3-2 = Sub(Sub(9,3), 2) = 4
        assert_eq!(calculate("9-3-2"), Ok(4));
    }

    // --- Multiplication ---

    #[test]
    fn test_simple_multiplication() {
        assert_eq!(calculate("3*4"), Ok(12));
    }

    #[test]
    fn test_multiply_by_zero() {
        assert_eq!(calculate("5*0"), Ok(0));
    }

    #[test]
    fn test_multiply_by_one() {
        assert_eq!(calculate("7*1"), Ok(7));
    }

    #[test]
    fn test_multiplication_chain() {
        // 2*3*4 = Mul(Mul(2,3), 4) = 24
        assert_eq!(calculate("2*3*4"), Ok(24));
    }

    // --- Division ---

    #[test]
    fn test_simple_division() {
        assert_eq!(calculate("8/4"), Ok(2));
    }

    #[test]
    fn test_division_by_one() {
        assert_eq!(calculate("7/1"), Ok(7));
    }

    #[test]
    fn test_integer_division_truncates() {
        // i64 division truncates toward zero
        assert_eq!(calculate("7/2"), Ok(3));
    }

    #[test]
    fn test_division_chain() {
        // 8/4/2 = Div(Div(8,4), 2) = 1
        assert_eq!(calculate("8/4/2"), Ok(1));
    }

    // --- Division by Zero ---

    #[test]
    fn test_division_by_zero() {
        assert!(matches!(calculate("8/0"), Err(EvalError::DivisionByZero)));
    }

    #[test]
    fn test_division_by_zero_in_chain() {
        // 8/0+1 — division by zero should propagate up
        assert!(matches!(calculate("8/0+1"), Err(EvalError::DivisionByZero)));
    }

    #[test]
    fn test_division_by_zero_on_right() {
        // 1+8/0 — error on right side still propagates
        assert!(matches!(calculate("1+8/0"), Err(EvalError::DivisionByZero)));
    }

    // --- Operator Precedence ---

    #[test]
    fn test_mul_before_add() {
        // 3+4*2 = 3+(4*2) = 11, not (3+4)*2 = 14
        assert_eq!(calculate("3+4*2"), Ok(11));
    }

    #[test]
    fn test_mul_before_sub() {
        // 5*2-1 = (5*2)-1 = 9
        assert_eq!(calculate("5*2-1"), Ok(9));
    }

    #[test]
    fn test_div_before_add() {
        // 8/4+1 = (8/4)+1 = 3
        assert_eq!(calculate("8/4+1"), Ok(3));
    }

    #[test]
    fn test_div_before_sub() {
        // 9/3-1 = (9/3)-1 = 2
        assert_eq!(calculate("9/3-1"), Ok(2));
    }

    #[test]
    fn test_mixed_precedence_complex() {
        // 2+3*4-1 = 2+(3*4)-1 = 13
        assert_eq!(calculate("2+3*4-1"), Ok(13));
    }

    // --- Left Associativity ---

    #[test]
    fn test_sub_left_associative() {
        // 9-3-2 = (9-3)-2 = 4, not 9-(3-2) = 8
        assert_eq!(calculate("9-3-2"), Ok(4));
    }

    #[test]
    fn test_div_left_associative() {
        // 8/4/2 = (8/4)/2 = 1, not 8/(4/2) = 4
        assert_eq!(calculate("8/4/2"), Ok(1));
    }

    // --- Whitespace ---

    #[test]
    fn test_spaces_ignored() {
        assert_eq!(calculate("3 + 4"), Ok(7));
    }

    #[test]
    fn test_tabs_ignored() {
        assert_eq!(calculate("3\t*\t4"), Ok(12));
    }
}

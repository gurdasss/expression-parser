//! # Error Module
//!
//! This module defines error types used throughout the expression parser.
//! It includes both parsing errors that occur during tokenization and parsing,
//! as well as evaluation errors that occur during AST evaluation.

use crate::token::Token;
use std::fmt;

/// Represents errors that can occur during parsing of expressions.
///
/// Parse errors indicate issues with the structure or content of the input
/// expression that prevent successful parsing.
#[derive(Debug)]
pub enum ParseError {
    /// An unexpected token was encountered during parsing.
    UnexpectedToken(Token),
    /// The input ended unexpectedly (premature EOF).
    UnexpectedEOF,
}

/// Represents errors that can occur during evaluation of expressions.
///
/// Evaluation errors indicate issues encountered while computing the result
/// of a valid AST, such as division by zero.
#[derive(Debug, PartialEq)]
pub enum EvalError {
    /// Attempted to divide by zero.
    DivisionByZero,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(token) => {
                write!(f, "unexpected token: {}", token)
            }
            ParseError::UnexpectedEOF => {
                write!(f, "unexpected end of input")
            }
        }
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::DivisionByZero => {
                write!(f, "division by zero")
            }
        }
    }
}

//! # Error Module
//!
//! This module defines error types used throughout the expression parser.
//! Currently, it focuses on parse errors that can occur during parsing.

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

#[derive(Debug, PartialEq)]
pub enum EvalError {
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

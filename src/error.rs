//! # Error Module
//!
//! This module defines error types used throughout the expression parser.
//! Currently, it focuses on parse errors that can occur during parsing.

use crate::token::Token;

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

#[derive(Debug)]
pub enum EvalError {
    DivisionByZero,
}

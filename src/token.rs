//! # Token Module
//!
//! This module defines the `Token` enum, which represents the lexical tokens
//! produced by the lexer. Each variant corresponds to a type of token that can
//! appear in a mathematical expression, including integers, operators, and
//! special tokens like EOF (end of file) and error tokens.

/// Represents a lexical token in the expression parser.
///
/// Tokens are the basic building blocks of the input expression, produced by
/// the lexer and consumed by the parser. This enum includes variants for
/// numeric literals, arithmetic operators, and control tokens.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
    /// An integer literal (currently single-digit, 0-9).
    Int(i64),
    /// The addition operator `+`.
    Add,
    /// The subtraction operator `-`.
    Sub,
    /// The multiplication operator `*`.
    Mul,
    /// The division operator `/`.
    Div,
    /// End of file/input marker.
    EOF,
    /// An error token for unrecognized characters.
    Err(char),
}

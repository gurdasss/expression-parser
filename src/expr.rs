//! # Expression Module
//!
//! This module defines the abstract syntax tree (AST) for mathematical expressions.
//! The `Expr` enum represents the structure of parsed expressions, allowing for
//! evaluation and manipulation of the expression tree.

use crate::token::Token;

/// Represents an expression in the abstract syntax tree (AST).
///
/// Expressions can be either numeric literals or binary operations combining
/// two sub-expressions with an operator. This recursive structure allows
/// representing complex mathematical expressions.
pub(crate) enum Expr {
    /// A numeric literal value.
    Number(f64),
    /// A binary operation with left operand, operator, and right operand.
    BinaryOp {
        /// The left-hand side expression.
        left: Box<Expr>,
        /// The binary operator token.
        op: Token,
        /// The right-hand side expression.
        right: Box<Expr>,
    },
}

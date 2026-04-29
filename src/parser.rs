//! # Parser Module
//!
//! This module implements the parser for mathematical expressions using a Pratt parser
//! (top-down operator precedence parsing). The parser converts a sequence of tokens
//! into an abstract syntax tree (AST) represented by `Expr` nodes.
//!
//! Currently implements basic token navigation; full parsing logic is under development.

use crate::error::ParseError;
use crate::token::Token;

/// The parser that converts tokens into an abstract syntax tree.
///
/// The parser uses a Pratt parsing algorithm to handle operator precedence
/// and associativity. It maintains a position in the token stream and builds
/// the AST incrementally.
pub(crate) struct Parser {
    /// The sequence of tokens to parse.
    tokens: Vec<Token>,
    /// The current position in the token stream.
    pos: usize,
}

impl Parser {
    /// Creates a new parser with the given token sequence.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The vector of tokens to parse.
    ///
    /// # Examples
    ///
    /// ```
    /// let tokens = vec![Token::Int(3), Token::Add, Token::Int(4)];
    /// let parser = Parser::new(tokens);
    /// ```
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    /// Peeks at the current token without advancing the position.
    ///
    /// Returns `None` if at the end of tokens.
    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.pos).cloned()
    }

    /// Advances the position and returns the current token.
    ///
    /// Returns `None` if at the end of tokens.
    fn advance(&mut self) -> Option<Token> {
        let current_token = self.peek();
        self.pos += 1;
        current_token
    }
}

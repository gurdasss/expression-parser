//! # Parser Module
//!
//! This module implements the parser for mathematical expressions using a Pratt parser
//! (top-down operator precedence parsing). The parser converts a sequence of tokens
//! into an abstract syntax tree (AST) represented by `Expr` nodes.
//!
//! Currently implements basic token navigation; full parsing logic is under development.

use crate::error::ParseError;
use crate::expr::Expr;
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

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_expr(0)?;

        // ensure nothing is left unconsumed
        match self.peek() {
            Some(Token::EOF) | None => Ok(expr),
            Some(other) => Err(ParseError::UnexpectedToken(other)),
        }
    }

    fn parse_expr(&mut self, min_power: u8) -> Result<Expr, ParseError> {
        // 1. parse the left side — must be a number
        let mut left = match self.advance() {
            Some(Token::Int(n)) => Expr::Number(n),
            Some(Token::EOF) => return Err(ParseError::UnexpectedEOF), // ← add this
            Some(other) => return Err(ParseError::UnexpectedToken(other)),
            None => return Err(ParseError::UnexpectedEOF),
        };
        // 2. loop — peek at next token, decide whether to consume it
        loop {
            let op = match self.peek() {
                // fill this in
                Some(Token::EOF) | None => break, // end of input, stop
                Some(token) => token,             // any other token, try to parse as operator
            };

            // 3. get binding power — if too weak or not an op, stop
            let lbp = match binding_power(&op) {
                Some(p) if p > min_power => p,
                _ => break,
            };

            // 4. consume the operator
            self.advance();

            // 5. recursively parse the right side
            let right = self.parse_expr(lbp)?;

            // 6. wrap into a BinaryOp node, becomes new left
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }
}

fn binding_power(token: &Token) -> Option<u8> {
    match token {
        Token::Add | Token::Sub => Some(1),
        Token::Mul | Token::Div => Some(2),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(input: &str) -> Result<Expr, ParseError> {
        let tokens = Lexer::new(input).tokenize();
        Parser::new(tokens).parse()
    }

    // --- Single Numbers ---

    #[test]
    fn test_single_number() {
        assert!(matches!(parse("3"), Ok(Expr::Number(3))));
    }

    #[test]
    fn test_single_zero() {
        assert!(matches!(parse("0"), Ok(Expr::Number(0))));
    }

    #[test]
    fn test_single_nine() {
        assert!(matches!(parse("9"), Ok(Expr::Number(9))));
    }

    // --- Basic Binary Operations ---

    #[test]
    fn test_addition() {
        assert!(matches!(
            parse("3+4"),
            Ok(Expr::BinaryOp { op: Token::Add, .. })
        ));
    }

    #[test]
    fn test_subtraction() {
        assert!(matches!(
            parse("3-4"),
            Ok(Expr::BinaryOp { op: Token::Sub, .. })
        ));
    }

    #[test]
    fn test_multiplication() {
        assert!(matches!(
            parse("3*4"),
            Ok(Expr::BinaryOp { op: Token::Mul, .. })
        ));
    }

    #[test]
    fn test_division() {
        assert!(matches!(
            parse("3/4"),
            Ok(Expr::BinaryOp { op: Token::Div, .. })
        ));
    }

    // --- Operator Precedence ---

    #[test]
    fn test_mul_before_add() {
        // 3+4*2 should be Add(3, Mul(4,2)) not Mul(Add(3,4), 2)
        assert!(matches!(
            parse("3+4*2"),
            Ok(Expr::BinaryOp {
                op: Token::Add,
                right,
                ..
            }) if matches!(*right, Expr::BinaryOp { op: Token::Mul, .. })
        ));
    }

    #[test]
    fn test_mul_before_sub() {
        // 5*2-1 should be Sub(Mul(5,2), 1)
        assert!(matches!(
            parse("5*2-1"),
            Ok(Expr::BinaryOp {
                op: Token::Sub,
                left,
                ..
            }) if matches!(*left, Expr::BinaryOp { op: Token::Mul, .. })
        ));
    }

    #[test]
    fn test_div_before_add() {
        // 8/4+1 should be Add(Div(8,4), 1)
        assert!(matches!(
            parse("8/4+1"),
            Ok(Expr::BinaryOp {
                op: Token::Add,
                left,
                ..
            }) if matches!(*left, Expr::BinaryOp { op: Token::Div, .. })
        ));
    }

    #[test]
    fn test_div_before_sub() {
        // 9/3-1 should be Sub(Div(9,3), 1)
        assert!(matches!(
            parse("9/3-1"),
            Ok(Expr::BinaryOp {
                op: Token::Sub,
                left,
                ..
            }) if matches!(*left, Expr::BinaryOp { op: Token::Div, .. })
        ));
    }

    // --- Left Associativity ---

    #[test]
    fn test_add_left_associative() {
        // 3+4+2 should be Add(Add(3,4), 2)
        assert!(matches!(
            parse("3+4+2"),
            Ok(Expr::BinaryOp {
                op: Token::Add,
                left,
                ..
            }) if matches!(*left, Expr::BinaryOp { op: Token::Add, .. })
        ));
    }

    #[test]
    fn test_sub_left_associative() {
        // 9-3-2 should be Sub(Sub(9,3), 2)
        assert!(matches!(
            parse("9-3-2"),
            Ok(Expr::BinaryOp {
                op: Token::Sub,
                left,
                ..
            }) if matches!(*left, Expr::BinaryOp { op: Token::Sub, .. })
        ));
    }

    #[test]
    fn test_mul_left_associative() {
        // 2*3*4 should be Mul(Mul(2,3), 4)
        assert!(matches!(
            parse("2*3*4"),
            Ok(Expr::BinaryOp {
                op: Token::Mul,
                left,
                ..
            }) if matches!(*left, Expr::BinaryOp { op: Token::Mul, .. })
        ));
    }

    #[test]
    fn test_div_left_associative() {
        // 8/4/2 should be Div(Div(8,4), 2)
        assert!(matches!(
            parse("8/4/2"),
            Ok(Expr::BinaryOp {
                op: Token::Div,
                left,
                ..
            }) if matches!(*left, Expr::BinaryOp { op: Token::Div, .. })
        ));
    }

    // --- Whitespace ---

    #[test]
    fn test_spaces_ignored() {
        assert!(matches!(
            parse("3 + 4"),
            Ok(Expr::BinaryOp { op: Token::Add, .. })
        ));
    }

    #[test]
    fn test_tabs_ignored() {
        assert!(matches!(
            parse("3\t*\t4"),
            Ok(Expr::BinaryOp { op: Token::Mul, .. })
        ));
    }

    // --- Error Cases ---

    #[test]
    fn test_empty_input() {
        assert!(matches!(parse(""), Err(ParseError::UnexpectedEOF)));
    }

    #[test]
    fn test_only_operator() {
        assert!(matches!(
            parse("+"),
            Err(ParseError::UnexpectedToken(Token::Add))
        ));
    }

    #[test]
    fn test_leading_operator() {
        assert!(matches!(
            parse("*3"),
            Err(ParseError::UnexpectedToken(Token::Mul))
        ));
    }

    #[test]
    fn test_trailing_operator() {
        // "3+" — parser reads 3, then sees + as operator,
        // recurses for right side, gets EOF
        assert!(matches!(parse("3+"), Err(ParseError::UnexpectedEOF)));
    }

    #[test]
    fn test_double_operator() {
        // "3++4" — parser reads 3, consumes first +,
        // recurses, sees second + where a number is expected
        assert!(matches!(
            parse("3++4"),
            Err(ParseError::UnexpectedToken(Token::Add))
        ));
    }

    #[test]
    fn test_unknown_character() {
        // "3@4" — after parsing 3, parser sees Token::Err('@')
        // which is not EOF, so it's now an error
        assert!(matches!(
            parse("3@4"),
            Err(ParseError::UnexpectedToken(Token::Err('@')))
        ));
    }

    #[test]
    fn test_only_unknown_character() {
        assert!(matches!(
            parse("@"),
            Err(ParseError::UnexpectedToken(Token::Err('@')))
        ));
    }
}

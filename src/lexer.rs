//! # Lexer Module
//!
//! This module implements the lexical analyzer (lexer) for the expression parser.
//! The lexer converts a string input into a sequence of tokens that can be
//! processed by the parser. It handles basic arithmetic operators and single-digit
//! integers, skipping whitespace and reporting errors for invalid characters.

use crate::token::Token;

/// The lexical analyzer that tokenizes input strings into tokens.
///
/// The lexer processes the input character by character, recognizing patterns
/// for numbers, operators, and other tokens. It maintains an internal position
/// in the input and produces a vector of tokens.
pub(crate) struct Lexer {
    /// The input characters to be tokenized.
    input: Vec<char>,
    /// The current position in the input.
    pos: usize,
}

impl Lexer {
    /// Creates a new lexer with the given input string.
    ///
    /// # Arguments
    ///
    /// * `input` - The string to be tokenized.
    ///
    /// # Examples
    ///
    /// ```
    /// let lexer = Lexer::new("3 + 4");
    /// ```
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    /// Peeks at the current character without advancing the position.
    ///
    /// Returns `None` if at the end of input.
    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    /// Advances the position and returns the current character.
    ///
    /// Returns `None` if at the end of input.
    fn advance(&mut self) -> Option<char> {
        let current_char = self.peek();
        self.pos += 1;
        current_char
    }

    /// Tokenizes the entire input string into a vector of tokens.
    ///
    /// Processes the input from start to finish, recognizing operators,
    /// numbers, and skipping whitespace. Ends with an EOF token.
    ///
    /// # Returns
    ///
    /// A vector of `Token`s representing the lexical structure of the input.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lexer = Lexer::new("3 + 4");
    /// let tokens = lexer.tokenize();
    /// // tokens: [Int(3), Add, Int(4), EOF]
    /// ```
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(current_char) = self.advance() {
            // match ch to a token and push
            match current_char {
                '+' => tokens.push(Token::Add),
                '-' => tokens.push(Token::Sub),
                '*' => tokens.push(Token::Mul),
                '/' => tokens.push(Token::Div),
                '0'..='9' => tokens.push(Token::Int(current_char.to_digit(10).unwrap() as i64)),
                ' ' | '\t' | '\n' => {} // skip, push nothing
                _ => tokens.push(Token::Err(current_char)),
            }
        }

        tokens.push(Token::EOF);
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Basic Single Tokens ---

    #[test]
    fn test_single_integer() {
        let tokens = Lexer::new("3").tokenize();
        assert_eq!(tokens, vec![Token::Int(3), Token::EOF]);
    }

    #[test]
    fn test_single_add() {
        let tokens = Lexer::new("+").tokenize();
        assert_eq!(tokens, vec![Token::Add, Token::EOF]);
    }

    #[test]
    fn test_single_sub() {
        let tokens = Lexer::new("-").tokenize();
        assert_eq!(tokens, vec![Token::Sub, Token::EOF]);
    }

    #[test]
    fn test_single_mul() {
        let tokens = Lexer::new("*").tokenize();
        assert_eq!(tokens, vec![Token::Mul, Token::EOF]);
    }

    #[test]
    fn test_single_div() {
        let tokens = Lexer::new("/").tokenize();
        assert_eq!(tokens, vec![Token::Div, Token::EOF]);
    }

    // --- Basic Expressions ---

    #[test]
    fn test_simple_addition() {
        let tokens = Lexer::new("3+4").tokenize();
        assert_eq!(
            tokens,
            vec![Token::Int(3), Token::Add, Token::Int(4), Token::EOF]
        );
    }

    #[test]
    fn test_simple_subtraction() {
        let tokens = Lexer::new("3-4").tokenize();
        assert_eq!(
            tokens,
            vec![Token::Int(3), Token::Sub, Token::Int(4), Token::EOF]
        );
    }

    #[test]
    fn test_simple_multiplication() {
        let tokens = Lexer::new("3*4").tokenize();
        assert_eq!(
            tokens,
            vec![Token::Int(3), Token::Mul, Token::Int(4), Token::EOF]
        );
    }

    #[test]
    fn test_simple_division() {
        let tokens = Lexer::new("3/4").tokenize();
        assert_eq!(
            tokens,
            vec![Token::Int(3), Token::Div, Token::Int(4), Token::EOF]
        );
    }

    // --- Whitespace Handling ---

    #[test]
    fn test_spaces_between_tokens() {
        let tokens = Lexer::new("3 + 4").tokenize();
        assert_eq!(
            tokens,
            vec![Token::Int(3), Token::Add, Token::Int(4), Token::EOF]
        );
    }

    #[test]
    fn test_tabs_between_tokens() {
        let tokens = Lexer::new("3\t+\t4").tokenize();
        assert_eq!(
            tokens,
            vec![Token::Int(3), Token::Add, Token::Int(4), Token::EOF]
        );
    }

    #[test]
    fn test_multiple_spaces() {
        let tokens = Lexer::new("3   +   4").tokenize();
        assert_eq!(
            tokens,
            vec![Token::Int(3), Token::Add, Token::Int(4), Token::EOF]
        );
    }

    #[test]
    fn test_leading_whitespace() {
        let tokens = Lexer::new("   3+4").tokenize();
        assert_eq!(
            tokens,
            vec![Token::Int(3), Token::Add, Token::Int(4), Token::EOF]
        );
    }

    #[test]
    fn test_trailing_whitespace() {
        let tokens = Lexer::new("3+4   ").tokenize();
        assert_eq!(
            tokens,
            vec![Token::Int(3), Token::Add, Token::Int(4), Token::EOF]
        );
    }

    // --- Precedence Expressions ---

    #[test]
    fn test_mixed_precedence() {
        let tokens = Lexer::new("3+4*2").tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Int(3),
                Token::Add,
                Token::Int(4),
                Token::Mul,
                Token::Int(2),
                Token::EOF
            ]
        );
    }

    #[test]
    fn test_all_operators() {
        let tokens = Lexer::new("1+2-3*4/5").tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Int(1),
                Token::Add,
                Token::Int(2),
                Token::Sub,
                Token::Int(3),
                Token::Mul,
                Token::Int(4),
                Token::Div,
                Token::Int(5),
                Token::EOF,
            ]
        );
    }

    // --- Edge Cases ---

    #[test]
    fn test_empty_input() {
        let tokens = Lexer::new("").tokenize();
        assert_eq!(tokens, vec![Token::EOF]);
    }

    #[test]
    fn test_only_whitespace() {
        let tokens = Lexer::new("   ").tokenize();
        assert_eq!(tokens, vec![Token::EOF]);
    }

    #[test]
    fn test_all_single_digits() {
        let tokens = Lexer::new("0123456789").tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Int(0),
                Token::Int(1),
                Token::Int(2),
                Token::Int(3),
                Token::Int(4),
                Token::Int(5),
                Token::Int(6),
                Token::Int(7),
                Token::Int(8),
                Token::Int(9),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_single_zero() {
        let tokens = Lexer::new("0").tokenize();
        assert_eq!(tokens, vec![Token::Int(0), Token::EOF]);
    }

    #[test]
    fn test_single_nine() {
        let tokens = Lexer::new("9").tokenize();
        assert_eq!(tokens, vec![Token::Int(9), Token::EOF]);
    }

    // --- Unknown Characters ---

    #[test]
    fn test_unknown_character() {
        let tokens = Lexer::new("3@4").tokenize();
        assert_eq!(
            tokens,
            vec![Token::Int(3), Token::Err('@'), Token::Int(4), Token::EOF]
        );
    }

    #[test]
    fn test_unknown_letter() {
        let tokens = Lexer::new("3x4").tokenize();
        assert_eq!(
            tokens,
            vec![Token::Int(3), Token::Err('x'), Token::Int(4), Token::EOF]
        );
    }

    #[test]
    fn test_only_unknown() {
        let tokens = Lexer::new("@").tokenize();
        assert_eq!(tokens, vec![Token::Err('@'), Token::EOF]);
    }

    #[test]
    fn test_multiple_unknown() {
        let tokens = Lexer::new("@#$").tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Err('@'),
                Token::Err('#'),
                Token::Err('$'),
                Token::EOF
            ]
        );
    }
}

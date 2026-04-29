#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
    Int(i64),
    Add,
    Sub,
    Mul,
    Div,
    EOF,
    Err(char),
}

pub(crate) struct Lexer {
    input: Vec<char>, // the characters to lex
    pos: usize,       // current position
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    // Peek at the current character without advancing the position. If we are at the end of the input, return None.
    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    // Advance the position and return the current character. If we are at the end of the input, return None.
    fn advance(&mut self) -> Option<char> {
        let current_char = self.peek();
        self.pos += 1;
        current_char
    }

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

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

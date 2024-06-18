use crate::lexer::TokenType;

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
    Not,
}

#[derive(Debug)]
pub enum BinaryOperator {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,

    // Boolean
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug)]
pub enum Declaration {
    Variable(String, Expression),
    Statement(Statement),
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
    Block(Vec<Declaration>),
    If(Expression, Box<Statement>),
}

#[derive(Debug)]
pub enum Expression {
    Number(f64),
    Boolean(bool),
    Void,
    String(String),
    Variable(String),
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Logical(LogicalOperator, Box<Expression>, Box<Expression>),
}

impl UnaryOperator {
    pub fn from_tokentype(kind: TokenType) -> Self {
        match kind {
            TokenType::Bang => Self::Not,
            TokenType::Minus => Self::Negate,
            _ => unreachable!("Invalid unary operator"),
        }
    }
}

impl BinaryOperator {
    pub fn from_tokentype(kind: TokenType) -> Self {
        match kind {
            TokenType::Plus => Self::Add,
            TokenType::Minus => Self::Subtract,
            TokenType::Star => Self::Multiply,
            TokenType::Slash => Self::Divide,
            TokenType::EqualEqual => Self::Equal,
            TokenType::BangEqual => Self::NotEqual,
            TokenType::Greater => Self::Greater,
            TokenType::GreaterEqual => Self::GreaterEqual,
            TokenType::Less => Self::Less,
            TokenType::LessEqual => Self::LessEqual,
            _ => unreachable!("Invalid binary operator"),
        }
    }
}

impl LogicalOperator {
    pub fn from_tokentype(kind: TokenType) -> Self {
        match kind {
            TokenType::And => Self::And,
            TokenType::Or => Self::Or,
            _ => unreachable!("Invalid logical operator"),
        }
    }
}

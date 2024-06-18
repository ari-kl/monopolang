use crate::{
    ast::{
        self, BinaryOperator, Declaration, Expression, LogicalOperator, Statement, UnaryOperator,
    },
    lexer::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::new();

        while !self.is_at_end() {
            declarations.push(self.declaration());
        }

        declarations
    }

    pub fn declaration(&mut self) -> Declaration {
        if self.match_token(TokenType::Let) {
            self.variable_declaration()
        } else {
            Declaration::Statement(self.statement())
        }
    }

    pub fn variable_declaration(&mut self) -> Declaration {
        let name = self
            .consume(TokenType::Identifier, "Expected variable name")
            .lexeme;
        self.consume(TokenType::Equal, "Expected '=' after variable name");

        let initializer = self.expression();

        Declaration::Variable(name, initializer)
    }

    pub fn statement(&mut self) -> Statement {
        match self.peek().kind {
            TokenType::Print => self.print_statement(),
            TokenType::If => self.if_statement(),
            _ => Statement::Expression(self.expression()),
        }
    }

    pub fn print_statement(&mut self) -> Statement {
        self.advance();
        let value = self.expression();

        Statement::Print(value)
    }

    pub fn if_statement(&mut self) -> Statement {
        self.advance();

        let condition = self.expression();

        self.consume(TokenType::Then, "Expected 'then' after if condition");

        let then_branch = Box::new(Statement::Block(self.block()));

        Statement::If(condition, then_branch)
    }

    pub fn block(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::new();

        while !self.check(TokenType::End) && !self.is_at_end() {
            declarations.push(self.declaration());
        }

        self.consume(TokenType::End, "Expected 'end' after block");

        declarations
    }

    pub fn expression(&mut self) -> Expression {
        self.or_expression()
    }

    pub fn or_expression(&mut self) -> Expression {
        let mut expr = self.and_expression();

        while self.match_token(TokenType::Or) {
            let operator = self.previous().kind;
            let right = self.and_expression();
            expr = Expression::Logical(
                LogicalOperator::from_tokentype(operator),
                Box::new(expr),
                Box::new(right),
            );
        }

        expr
    }

    pub fn and_expression(&mut self) -> Expression {
        let mut expr = self.equality();

        while self.match_token(TokenType::And) {
            let operator = self.previous().kind;
            let right = self.equality();
            expr = Expression::Logical(
                LogicalOperator::from_tokentype(operator),
                Box::new(expr),
                Box::new(right),
            );
        }

        expr
    }

    pub fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();

        while self.match_token(TokenType::EqualEqual) || self.match_token(TokenType::BangEqual) {
            let operator = self.previous().kind;
            let right = self.comparison();
            expr = Expression::Binary(
                BinaryOperator::from_tokentype(operator),
                Box::new(expr),
                Box::new(right),
            );
        }

        expr
    }

    pub fn comparison(&mut self) -> Expression {
        let mut expr = self.term();

        while self.match_token(TokenType::Greater)
            || self.match_token(TokenType::GreaterEqual)
            || self.match_token(TokenType::Less)
            || self.match_token(TokenType::LessEqual)
        {
            let operator = self.previous().kind;
            let right = self.term();
            expr = Expression::Binary(
                BinaryOperator::from_tokentype(operator),
                Box::new(expr),
                Box::new(right),
            );
        }

        expr
    }

    pub fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        while self.match_token(TokenType::Minus) || self.match_token(TokenType::Plus) {
            let operator = self.previous().kind;
            let right = self.factor();
            expr = Expression::Binary(
                BinaryOperator::from_tokentype(operator),
                Box::new(expr),
                Box::new(right),
            );
        }

        expr
    }

    pub fn factor(&mut self) -> Expression {
        let mut expr = self.unary();

        while self.match_token(TokenType::Slash) || self.match_token(TokenType::Star) {
            let operator = self.previous().kind;
            let right = self.unary();
            expr = Expression::Binary(
                BinaryOperator::from_tokentype(operator),
                Box::new(expr),
                Box::new(right),
            );
        }

        expr
    }

    pub fn unary(&mut self) -> Expression {
        if self.match_token(TokenType::Bang) || self.match_token(TokenType::Minus) {
            let operator = self.previous().kind;
            let right = self.unary();
            Expression::Unary(UnaryOperator::from_tokentype(operator), Box::new(right))
        } else {
            self.primary()
        }
    }

    pub fn primary(&mut self) -> Expression {
        if self.match_token(TokenType::False) {
            Expression::Boolean(false)
        } else if self.match_token(TokenType::True) {
            Expression::Boolean(true)
        } else if self.match_token(TokenType::Number) {
            Expression::Number(self.previous().lexeme.parse().unwrap())
        } else if self.match_token(TokenType::String) {
            Expression::String(self.previous().lexeme.clone())
        } else if self.match_token(TokenType::Identifier) {
            Expression::Variable(self.previous().lexeme.clone())
        } else if self.match_token(TokenType::LeftParen) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expected ')' after expression");
            expr
        } else {
            panic!(
                "Expected expression at {}:{}",
                self.peek().line,
                self.peek().column
            )
        }
    }

    pub fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    pub fn check(&self, kind: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.tokens[self.current].kind == kind
        }
    }

    pub fn consume(&mut self, kind: TokenType, message: &str) -> Token {
        if self.check(kind) {
            self.advance()
        } else {
            panic!("{}", message)
        }
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn match_token(&mut self, kind: TokenType) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.tokens[self.current - 1].clone()
    }

    pub fn is_at_end(&self) -> bool {
        self.tokens[self.current].kind == TokenType::Eof
    }
}

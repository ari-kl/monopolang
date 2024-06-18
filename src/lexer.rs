#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    // Grouping
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,

    // Arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,

    // Logical operators
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,
    Boolean,

    // Keywords
    And,
    Or,
    If,
    Else,
    End,
    While,
    For,
    In,
    Do,
    Function,
    Return,
    Let,
    Print,

    // Special
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    pub line: u32,
    pub column: u32,
}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    line: u32,
    column: u32,
    start: usize,
    current: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            line: 1,
            column: 0,
            start: 0,
            current: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            kind: TokenType::Eof,
            lexeme: String::new(),
            line: self.line,
            column: self.column,
        });

        return self.tokens.clone();
    }

    fn scan_token(&mut self) {
        match self.advance() {
            ' ' | '\r' | '\t' => (),
            '\n' => {
                self.line += 1;
                self.column = 0; // Reset column to 0 because we increment it each iteration
            }
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),
            '/' => self.add_token(TokenType::Slash),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => {}
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha() {
            self.advance();
        }

        let kind = match self.source[self.start..self.current].to_string().as_str() {
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "end" => TokenType::End,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "do" => TokenType::Do,
            "function" => TokenType::Function,
            "return" => TokenType::Return,
            "let" => TokenType::Let,
            "print" => TokenType::Print,
            "true" => TokenType::Boolean,
            "false" => TokenType::Boolean,
            _ => TokenType::Identifier,
        };

        self.add_token(kind);
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            println!(
                "Unterminated string at line {} and column {}",
                self.line, self.column
            );
            return;
        }
        self.advance();
        self.add_token(TokenType::String);
    }

    fn number(&mut self) {
        while self.is_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit() {
            self.advance();
            while self.is_digit() {
                self.advance();
            }
        }
        self.add_token(TokenType::Number);
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.column += 1;

        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current as usize - 1).unwrap()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != expected {
            return false;
        }

        self.advance();
        true
    }

    fn add_token(&mut self, kind: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        let length = text.len() as u32;

        self.tokens.push(Token {
            kind,
            lexeme: text,
            line: self.line,
            column: self.column - length + 1, // Subtract length to get the start of the token
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_digit(&self) -> bool {
        self.peek().is_digit(10)
    }

    fn is_alpha(&self) -> bool {
        self.peek().is_alphabetic()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum TokenKind {
    Plus,
    Minus,
    Star,
    Slash,
    Equal,

    LCurly,
    RCurly,
    Semicolon,

    Number,
    Let,
    Identifier,

    Error,
    Eof,
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: Option<&'a str>,
}

impl<'a> Token<'a> {
    fn char(kind: TokenKind) -> Self {
        Self { kind, lexeme: None }
    }

    fn keyword(kind: TokenKind) -> Self {
        Self { kind, lexeme: None }
    }

    fn identifier(lexeme: &'a str) -> Self {
        Self {
            kind: TokenKind::Identifier,
            lexeme: Some(lexeme),
        }
    }

    fn number(lexeme: &'a str) -> Self {
        Self {
            kind: TokenKind::Number,
            lexeme: Some(lexeme),
        }
    }
}

pub struct Lexer<'a> {
    source: &'a str,
    ip: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, ip: 0 }
    }

    pub fn next(&mut self) -> Option<Token<'a>> {
        self.skip_whitespace();

        match self.peek() as u8 {
            b'+' => Some(self.char(TokenKind::Plus)),
            b'-' => Some(self.char(TokenKind::Minus)),
            b'*' => Some(self.char(TokenKind::Star)),
            b'/' => Some(self.char(TokenKind::Slash)),
            b'=' => Some(self.char(TokenKind::Equal)),

            b'{' => Some(self.char(TokenKind::LCurly)),
            b'}' => Some(self.char(TokenKind::RCurly)),
            b';' => Some(self.char(TokenKind::Semicolon)),

            b'a'..=b'z' | b'A'..=b'Z' | b'_' => Some(self.identifier()),
            b'0'..=b'9' => Some(self.number()),
            _ => None,
        }
    }

    fn at_end(&self) -> bool {
        self.ip >= self.source.len()
    }

    fn advance(&mut self) {
        self.ip += 1;
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return 0 as char;
        }

        self.source.as_bytes()[self.ip] as char
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() as u8 {
                b' ' | b'\t' | b'\n' => self.advance(),
                _ => break,
            }
        }
    }

    fn char(&mut self, kind: TokenKind) -> Token<'a> {
        self.advance();
        Token::char(kind)
    }

    fn identifier(&mut self) -> Token<'a> {
        let start_ip = self.ip;
        self.advance();

        loop {
            match self.peek() as u8 {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'0'..=b'9' => self.advance(),
                _ => break,
            }
        }

        let identifier = &self.source[start_ip..self.ip];
        match identifier {
            "let" => Token::keyword(TokenKind::Let),
            _ => Token::identifier(identifier),
        }
    }

    fn number(&mut self) -> Token<'a> {
        let start_ip = self.ip;
        self.advance();

        loop {
            match self.peek() as u8 {
                b'0'..=b'9' => self.advance(),
                _ => break,
            }
        }

        Token::number(&self.source[start_ip..self.ip])
    }
}

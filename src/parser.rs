use crate::lexer::{Lexer, Token, TokenKind};

#[derive(Debug)]
pub struct Binding<'a> {
    pub identifier: Token<'a>,
    pub expr: Box<Ast<'a>>,
}

#[derive(Debug)]
pub struct Binary<'a> {
    pub op: Token<'a>,
    pub lhs: Box<Ast<'a>>,
    pub rhs: Box<Ast<'a>>,
}

#[derive(Debug)]
pub enum Ast<'a> {
    NoOp,

    Literal(Token<'a>),
    Identifier(Token<'a>),

    Binding(Binding<'a>),

    Binary(Binary<'a>),

    Body(Vec<Box<Ast<'a>>>),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(source);
        let current = lexer.next();

        Self { lexer, current }
    }

    pub fn parse(&mut self) -> Box<Ast<'a>> {
        if let None = self.current {
            return Box::new(Ast::NoOp);
        }

        self.declaration_list(TokenKind::Eof)
    }

    fn current_kind(&self) -> TokenKind {
        self.current.map(|t| t.kind).unwrap_or(TokenKind::Eof)
    }

    fn get_consume(&mut self) -> Token<'a> {
        let old = self.current.unwrap();
        self.consume_here();
        old
    }

    fn consume_here(&mut self) {
        self.current = self.lexer.next();
    }

    fn consume<'b>(&mut self, expected: TokenKind, msg: &'b str) {
        if self.current_kind() == expected {
            self.current = self.lexer.next();
            return;
        }

        panic!("{msg}")
    }

    fn matches_any(&mut self, expected: &[TokenKind]) -> Option<Token<'a>> {
        for ex in expected {
            if self.current_kind() == *ex {
                let current = self.current.unwrap();
                self.consume_here();

                return Some(current);
            }
        }

        None
    }

    fn identifier(&mut self, msg: &'static str) -> Token<'a> {
        if self.current_kind() == TokenKind::Identifier {
            let old = self.current;
            self.current = self.lexer.next();
            return old.unwrap();
        }

        panic!("{msg}")
    }

    fn declaration_list(&mut self, end: TokenKind) -> Box<Ast<'a>> {
        let mut items = Vec::new();

        while self.current_kind() != end {
            let item = match self.current_kind() {
                TokenKind::Let => {
                    let b = self.let_binding();
                    self.consume(TokenKind::Semicolon, "Missing ';' after let binding");
                    b
                }
                TokenKind::LCurly => self.body(),
                t => panic!("Unknown type {t:?}"),
            };

            items.push(item);
        }
        self.consume(end, &format!("Missing end token '{end:?}'"));

        Box::new(Ast::Body(items))
    }

    fn let_binding(&mut self) -> Box<Ast<'a>> {
        self.consume_here();
        let identifier = self.identifier("Expect identifier after let");

        self.consume(
            TokenKind::Equal,
            "Expect '=' after identifier in let binding",
        );

        Box::new(Ast::Binding(Binding {
            identifier,
            expr: self.expr(),
        }))
    }

    fn body(&mut self) -> Box<Ast<'a>> {
        self.consume_here();
        self.declaration_list(TokenKind::RCurly)
    }

    fn expr(&mut self) -> Box<Ast<'a>> {
        self.term()
    }

    fn term(&mut self) -> Box<Ast<'a>> {
        let mut n = self.factor();

        while let Some(op) = self.matches_any(&[TokenKind::Plus, TokenKind::Minus]) {
            n = Box::new(Ast::Binary(Binary {
                op,
                lhs: n,
                rhs: self.factor(),
            }));
        }

        n
    }

    fn factor(&mut self) -> Box<Ast<'a>> {
        let mut n = self.primary();

        while let Some(op) = self.matches_any(&[TokenKind::Star, TokenKind::Slash]) {
            n = Box::new(Ast::Binary(Binary {
                op,
                lhs: n,
                rhs: self.primary(),
            }));
        }

        n
    }

    fn primary(&mut self) -> Box<Ast<'a>> {
        match self.current_kind() {
            TokenKind::Number => Box::new(Ast::Literal(self.get_consume())),
            TokenKind::Identifier => Box::new(Ast::Identifier(self.get_consume())),
            t => panic!("Unknown primary expr '{t:?}'"),
        }
    }
}

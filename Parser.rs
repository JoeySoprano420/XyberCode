#[derive(Debug)]
enum Expr {
    Number(i32),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Token::EOF,
        };
        parser.advance();
        parser
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn parse(&mut self) -> Expr {
        self.expr()
    }

    fn expr(&mut self) -> Expr {
        let mut result = self.term();

        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let op = self.current_token.clone();
            self.advance();
            let right = self.term();
            result = match op {
                Token::Plus => Expr::Add(Box::new(result), Box::new(right)),
                Token::Minus => Expr::Sub(Box::new(result), Box::new(right)),
                _ => result,
            };
        }

        result
    }

    fn term(&mut self) -> Expr {
        let mut result = self.factor();

        while matches!(self.current_token, Token::Multiply | Token::Divide) {
            let op = self.current_token.clone();
            self.advance();
            let right = self.factor();
            result = match op {
                Token::Multiply => Expr::Mul(Box::new(result), Box::new(right)),
                Token::Divide => Expr::Div(Box::new(result), Box::new(right)),
                _ => result,
            };
        }

        result
    }

    fn factor(&mut self) -> Expr {
        match self.current_token {
            Token::Number(n) => {
                self.advance();
                Expr::Number(n)
            }
            Token::LParen => {
                self.advance();
                let expr = self.expr();
                if self.current_token == Token::RParen {
                    self.advance();
                }
                expr
            }
            Token::Ident(ref ident) => {
                self.advance();
                Expr::Variable(ident.clone())
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    Ident(String),
    EOF,
}

struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn next_token(&mut self) -> Token {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance();
                continue;
            }
            if c.is_digit(10) {
                return self.number();
            }
            match c {
                '+' => return self.advance_and_return(Token::Plus),
                '-' => return self.advance_and_return(Token::Minus),
                '*' => return self.advance_and_return(Token::Multiply),
                '/' => return self.advance_and_return(Token::Divide),
                '(' => return self.advance_and_return(Token::LParen),
                ')' => return self.advance_and_return(Token::RParen),
                _ => {
                    if c.is_alphabetic() {
                        return self.identifier();
                    } else {
                        panic!("Unexpected character: {}", c);
                    }
                }
            }
        }
        Token::EOF
    }

    fn current_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self) {
        self.pos += self.current_char().unwrap_or_default().len_utf8();
    }

    fn advance_and_return(&mut self, token: Token) -> Token {
        self.advance();
        token
    }

    fn number(&mut self) -> Token {
        let start = self.pos;
        while let Some(c) = self.current_char() {
            if !c.is_digit(10) {
                break;
            }
            self.advance();
        }
        Token::Number(self.input[start..self.pos].parse().unwrap())
    }

    fn identifier(&mut self) -> Token {
        let start = self.pos;
        while let Some(c) = self.current_char() {
            if !c.is_alphanumeric() {
                break;
            }
            self.advance();
        }
        Token::Ident(self.input[start..self.pos].to_string())
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Print,
    Read,
    Loop(Vec<Token>),
}

pub struct Parser {
    src: String,
    pos: usize,
}

impl Parser {
    pub fn new(src: String) -> Self {
        Self { src, pos: 0 }
    }

    fn next(&mut self) {
        self.pos += 1;
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.src.chars().nth(self.pos) {

            // TODO: check for matching loop brackets
            let token = match c {
                '>' => Token::MoveRight,
                '<' => Token::MoveLeft,
                '+' => Token::Increment,
                '-' => Token::Decrement,
                '.' => Token::Print,
                ',' => Token::Read,
                '[' => {
                    self.next();
                    let tokens = self.parse();
                    Token::Loop(tokens)
                }
                ']' => return tokens,
                _ => {
                    self.next();
                    continue
                }
            };

            self.next();
            tokens.push(token);

        }

        tokens
    }

}

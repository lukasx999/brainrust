use crate::parser::Token;



pub struct Interpreter {
    dp: usize, // data pointer
    tape: [u8; 30_000],
    tokens: Vec<Token>,
}

impl Interpreter {

    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            dp: 0,
            tape: [0; 30_000],
            tokens,
        }
    }

    fn read(&mut self) {
        use std::io::Read;

        let mut buf: [u8; 1] = [0; 1];
        std::io::stdin().read_exact(&mut buf).unwrap();
        self.tape[self.dp] = buf[0];
    }

    fn print(&self) {
        print!("{}", self.tape[self.dp] as char);
    }

    fn exec(&mut self, tokens: Vec<Token>) {
        for token in tokens {
            match token {
                Token::MoveRight => self.dp += 1,
                Token::MoveLeft  => self.dp -= 1,
                Token::Increment => self.tape[self.dp] += 1,
                Token::Decrement => self.tape[self.dp] -= 1,
                Token::Print     => self.print(),
                Token::Read      => self.read(),
                Token::Loop(t)   => {
                    while self.tape[self.dp] != 0 {
                        self.exec(t.clone());
                    }
                }
            }
        }
    }

    pub fn run(&mut self) {
        self.exec(self.tokens.clone());
    }
}

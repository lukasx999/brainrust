use std::io::Write;
use crate::parser::Token;



pub struct Compiler {
    dp: usize, // data pointer
    tokens: Vec<Token>,
    output: String,
    filename: String,
    label_counter: u32,
}

impl Compiler {

    pub fn new(tokens: Vec<Token>, filename: String) -> Self {
        Self {
            dp: 1,
            tokens,
            output: String::new(),
            filename,
            label_counter: 0,
        }
    }

    fn asm<T>(&mut self, code: T) where T: AsRef<str> {
        self.output.push_str(format!("{}\n", code.as_ref()).as_str());
    }

    fn read_byte(&mut self) {
        self.asm("mov rax, 0");
        self.asm("mov rdi, 0");
        self.asm(format!("lea rsi, [rbp-{}]", self.dp));
        self.asm("mov rdx, 1");
        self.asm("syscall");
    }

    fn print_byte(&mut self) {
        self.asm("mov rax, 1");
        self.asm("mov rdi, 1");
        // mov cannot calculate memory address, hence using lea
        self.asm(format!("lea rsi, [rbp-{}]", self.dp));
        self.asm("mov rdx, 1");
        self.asm("syscall");
    }

    fn prelude(&mut self) {
        self.asm("global _start");
        self.asm("section .text");
        self.asm("_start:");
        self.asm("push rbp");
        self.asm("mov rbp, rsp");
    }

    fn epilogue(&mut self) {
        self.asm("pop rbp");
        self.asm("mov rax, 60");
        self.asm("mov rdi, 0");
        self.asm("syscall");
    }

    fn inc(&mut self) {
        self.asm(format!("inc BYTE [rbp-{}]", self.dp));
    }

    fn dec(&mut self) {
        self.asm(format!("dec BYTE [rbp-{}]", self.dp));
    }

    fn loop_(&mut self, tokens: Vec<Token>) {
        let count = self.label_counter;
        self.label_counter += 1;

        /* check condition first */
        self.asm(format!("jmp .loop_cond{}", count));

        /* loop body */
        self.asm(format!(".loop{}:", count));
        self.compile_instructions(tokens);

        /* condition */
        self.asm(format!(".loop_cond{}:", count));
        self.asm(format!("cmp BYTE [rbp-{}], 0", self.dp));
        self.asm(format!("jne .loop{}", count));

    }

    fn compile_instructions(&mut self, tokens: Vec<Token>) {
        for token in tokens {
            match token {
                Token::MoveRight  => self.dp += 1,
                Token::MoveLeft   => self.dp -= 1,
                Token::Increment  => self.inc(),
                Token::Decrement  => self.dec(),
                Token::Print      => self.print_byte(),
                Token::Read       => self.read_byte(),
                Token::Loop(toks) => self.loop_(toks),
            }
        }
    }

    pub fn compile(&mut self) {
        self.prelude();
        self.compile_instructions(self.tokens.clone());
        self.epilogue();
        self.write_output();
    }

    fn write_output(&self) {
        let mut file = std::fs::File::create(&self.filename).unwrap();
        file.write_all(self.output.as_bytes()).unwrap();
    }

}

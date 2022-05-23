#![allow(unused)]

#[derive(Debug, Clone, Copy)]
enum Literal {
    Int(i32),
    Str(&'static str),
}

#[derive(Debug, Clone, Copy)]
enum Instruction {}

struct Machine {
    stack: Vec<Literal>,
    code: Vec<Instruction>,
    pc: usize,
}

impl Machine {
    fn new(code: Vec<Instruction>) -> Machine {
        Machine {
            stack: Vec::new(),
            code,
            pc: 0,
        }
    }

    fn execute(&mut self) {
        while self.pc < self.code.len() {
            let opcode = self.code[self.pc];
            self.pc += 1;
            self.dispatch(opcode);
        }
    }

    fn dispatch(&mut self, opcode: Instruction) {
        match opcode {
            _ => todo!(),
        };
    }
}

fn main() {}

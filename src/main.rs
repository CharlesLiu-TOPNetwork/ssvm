#![allow(unused)]
use std::{
    collections::HashMap,
    io::{self, Write},
};

#[derive(Debug, Clone, Copy)]
enum Literal {
    Int(i32),
    Float(f32),
    Str(&'static str),
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Add,
    Sub,
    Mul,
    Div,
    Push(Literal),
    Display,
    Mstore((u32)),
    Mload((u32)),
}

struct Machine {
    stack: Vec<Literal>,
    code: Vec<Instruction>,
    pc: usize,
    mem: HashMap<u32, Literal>,
}

impl Machine {
    fn new(code: Vec<Instruction>) -> Machine {
        Machine {
            stack: Vec::new(),
            code,
            pc: 0,
            mem: HashMap::new(),
        }
    }

    fn push(&mut self, x: Literal) {
        self.stack.push(x);
    }

    fn pop(&mut self) -> Literal {
        self.stack.pop().expect("stack popped while empty")
    }

    fn execute(&mut self) {
        while self.pc < self.code.len() {
            let opcode = self.code[self.pc];
            self.pc += 1;
            self.dispatch(opcode);
        }
    }

    fn mem_store(&mut self, addr: u32, value: Literal) {
        self.mem.insert(addr, value);
    }

    fn mem_load(&mut self, addr: &u32) -> Option<&Literal> {
        self.mem.get(addr)
    }

    fn dispatch(&mut self, opcode: Instruction) {
        match opcode {
            Instruction::Push(lit) => self.push(lit),
            Instruction::Add => {
                let pair = (self.pop(), self.pop());
                self.push(match pair {
                    (Literal::Int(x), Literal::Int(y)) => Literal::Int(x + y),
                    (Literal::Float(x), Literal::Float(y)) => Literal::Float(x + y),
                    (Literal::Int(x), Literal::Float(y)) => Literal::Float(x as f32 + y),
                    (Literal::Float(x), Literal::Int(y)) => Literal::Float(x + y as f32),
                    _ => panic!("error in code-- expected int/float for binary operations"),
                });
            }
            Instruction::Sub => {
                let pair = (self.pop(), self.pop());
                self.push(match pair {
                    (Literal::Int(x), Literal::Int(y)) => Literal::Int(x - y),
                    (Literal::Float(x), Literal::Float(y)) => Literal::Float(x - y),
                    (Literal::Int(x), Literal::Float(y)) => Literal::Float(x as f32 - y),
                    (Literal::Float(x), Literal::Int(y)) => Literal::Float(x - y as f32),
                    _ => panic!("error in code-- expected int/float for binary operations"),
                });
            }
            Instruction::Mul => {
                let pair = (self.pop(), self.pop());
                self.push(match pair {
                    (Literal::Int(x), Literal::Int(y)) => Literal::Int(x * y),
                    (Literal::Float(x), Literal::Float(y)) => Literal::Float(x * y),
                    (Literal::Int(x), Literal::Float(y)) => Literal::Float(x as f32 * y),
                    (Literal::Float(x), Literal::Int(y)) => Literal::Float(x * y as f32),
                    _ => panic!("error in code-- expected int/float for binary operations"),
                });
            }
            Instruction::Div => {
                let pair = (self.pop(), self.pop());
                self.push(match pair {
                    (Literal::Int(x), Literal::Int(y)) => Literal::Int(x / y),
                    (Literal::Float(x), Literal::Float(y)) => Literal::Float(x / y),
                    (Literal::Int(x), Literal::Float(y)) => Literal::Float(x as f32 / y),
                    (Literal::Float(x), Literal::Int(y)) => Literal::Float(x / y as f32),
                    _ => panic!("error in code-- expected int/float for binary operations"),
                });
            }
            Instruction::Display => {
                match self.pop() {
                    Literal::Int(x) => print!("{:?}", x),
                    Literal::Float(x) => print!("{:?}", x),
                    Literal::Str(x) => print!("{}", x),
                }
                io::stdout().flush().unwrap();
            }
            Instruction::Mstore(addr) => {
                let value = self.pop();
                self.mem_store(addr, value);
            }
            Instruction::Mload(addr) => {
                let value = self.mem_load(&addr).unwrap().clone(); // todo error in code;
                self.push(value);
            }
        };
    }
}

fn main() {
    let mut machine = Machine::new(vec![
        Instruction::Push(Literal::Int(2)),
        Instruction::Push(Literal::Float(1.)),
        Instruction::Add,
        Instruction::Mstore(666),
        // ....
        Instruction::Push(Literal::Str("1 + 2 is ")),
        Instruction::Display,
        Instruction::Mload(666),
        Instruction::Display,
        Instruction::Push(Literal::Str("\n")),
        Instruction::Display,
    ]);
    println!("------------ machine output ------------");
    machine.execute();
}

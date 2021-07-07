use crate::chunk::*;
use crate::value::*;
use num_traits::FromPrimitive;

const STACK_MAX: usize = 256;

pub struct VM {
    pub chunk: Chunk,
    pub ip: usize,
    pub stack: [Value; STACK_MAX],
    pub stack_top: usize,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: [0.0; STACK_MAX],
            stack_top: 0,
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    fn reset_stack(&mut self) {
        self.stack_top = 0;
    }

    fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            if cfg!(feature = "DEBUG_TRACE_EXECUTION") {
                print!("          ");
                for slot in 0..self.stack_top {
                    let value = self.stack[slot];
                    print!("[ ");
                    print_value(value);
                    print!(" ]");
                }
                println!("");

                self.chunk.disassemble_instruction(self.ip);
            }

            let instruction = FromPrimitive::from_u8(self.chunk.code[self.ip]).unwrap(); // TODO:
            self.ip += 1;

            match instruction {
                Op::Constant => {
                    let byte = self.chunk.code[self.ip] as usize;
                    self.ip += 1;
                    let constant = self.chunk.constants.values[byte];
                    self.push(constant);
                },
                Op::Add      => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a + b);
                },
                Op::Subtract => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a - b);
                },
                Op::Multiply => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a * b);
                },
                Op::Divide   => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a / b);
                },
                Op::Negate   => {
                    let value = self.pop();
                    self.push(-value);
                },
                Op::Return   => {
                    print_value(self.pop());
                    println!("");
                    return InterpretResult::Ok;
                },
            };
        }
    }
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}
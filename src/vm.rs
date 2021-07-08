use crate::chunk::*;
use crate::compiler::*;
use crate::value::*;
use crate::value::Value::*;
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
            stack: [Nil; STACK_MAX],
            stack_top: 0,
        }
    }

    pub fn interpret(&mut self, source: &String) -> InterpretResult {
        let mut chunk = Chunk::new();
        let mut compiler = Compiler::new(source, &mut chunk);

        if !compiler.compile() {
            return InterpretResult::CompileError;
        }

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

    fn peek(&mut self, distance: usize) -> Value {
        let index = self.stack_top - 1 - distance;
        return self.stack[index];
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
                }
                Op::Nil => self.push(Nil),
                Op::False => self.push(Bool(false)),
                Op::True => self.push(Bool(true)),
                Op::Equal => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Bool(a == b));
                }
                Op::Greater => crate::binary_op!(self, Bool, >),
                Op::Less => crate::binary_op!(self, Bool, <),
                Op::Add => crate::binary_op!(self, Number, +),
                Op::Subtract => crate::binary_op!(self, Number, -),
                Op::Multiply => crate::binary_op!(self, Number, *),
                Op::Divide => crate::binary_op!(self, Number, /),
                Op::Not => {
                    let val = self.pop();
                    self.push(Bool(is_falsey(val)))
                }
                Op::Negate => {
                    if !self.peek(0).is_number() {
                        crate::error!(self, "Operand must be a number.");
                        return InterpretResult::RuntimeError;
                    }
                    let value = self.pop();
                    self.push(Number(-value.as_number()));
                }
                Op::Return => {
                    print_value(self.pop());
                    println!("");
                    return InterpretResult::Ok;
                }
            };
        }
    }

    fn runtime_error(&mut self, message: String) {
        eprintln!("{}", message); // todo: format?

        let instruction = self.ip - 1;
        let line = self.chunk.lines[instruction];
        eprintln!("[line {}] in script", line);
        self.reset_stack();
    }
}

fn is_falsey(val: Value) -> bool{
    match val {
        Nil => true,
        Bool(b) => !b,
        _ => false,
    }
}

#[macro_export]
macro_rules! error {
    ($vm: expr, $format: expr) => {
        $vm.runtime_error(format!($format))
    };
    ($vm: expr, $format: expr, $($args: expr),*) => {
        $vm.runtime_error(format!($format, $($args)* ))
    };
}

#[macro_export]
macro_rules! binary_op {
    ($vm:expr, $value_ctor:tt, $op:tt) => {
        {
            if !$vm.peek(0).is_number() || !$vm.peek(1).is_number() {
                crate::error!($vm, "Operands must be numbers.");
                return InterpretResult::RuntimeError;
            }
            let b = $vm.pop().as_number();
            let a = $vm.pop().as_number();
            $vm.push($value_ctor(a $op b));
        }
    }
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

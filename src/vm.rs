use crate::chunk::*;
use crate::compiler::*;
use crate::memory::*;
use crate::object::*;
use crate::value::*;
use crate::value::Value::*;
use num_traits::FromPrimitive;

const STACK_MAX: usize = 256;

pub struct VM {
    pub chunk: Chunk,
    pub ip: usize,
    pub stack: Vec<Value>,
    pub heap: Heap,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: vec![Nil; STACK_MAX],
            heap: Heap::new(),
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
        self.stack.clear();
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn peek(&mut self, distance: usize) -> Value {
        let index = self.stack.len() - 1 - distance;
        return self.stack[index];
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            if cfg!(feature = "DEBUG_TRACE_EXECUTION") {
                print!("          ");
                for value in &self.stack {
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
                Op::Add => {
                    match (self.pop(), self.pop()) {
                        (Number(b), Number(a)) => {
                            self.push(Number(a + b));
                        }
                        (Object(Obj::LString(b)), Object(Obj::LString(a))) => {
                            let str = format!("{}{}", a.obj(), b.obj());
                            let r = self.heap.manage(str);
                            self.push(Object(Obj::LString(r)));
                        }
                        _ => {
                            crate::error!(self, "Operands must be numbers or strings.");
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                Op::Subtract => crate::binary_op!(self, Number, -),
                Op::Multiply => crate::binary_op!(self, Number, *),
                Op::Divide => crate::binary_op!(self, Number, /),
                Op::Not => {
                    let val = self.pop();
                    self.push(Bool(is_falsey(val)))
                }
                Op::Negate => {
                    match self.pop() {
                        Number(n) => self.push(Number(-n)),
                        _ => {
                            crate::error!(self, "Operand must be a number.");
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                Op::Return => {
                    print_value(&self.pop());
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

    fn free_objects(&mut self) {
        self.heap.clear();
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
            match ($vm.pop(), $vm.pop()) {
                (Number(b), Number(a)) => {
                    $vm.push($value_ctor(a $op b));
                },
                _ => {
                    crate::error!($vm, "Operands must be numbers.");
                    return InterpretResult::RuntimeError;
                }
            }
        }
    }
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

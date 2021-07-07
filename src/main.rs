mod chunk;
mod debug;
mod value;
mod vm;

use chunk::*;
use std::convert::TryInto;
use vm::*;

fn main() {
    let mut chunk = Chunk::new();
    let mut vm = VM::new();

    let constant = chunk.add_constant(1.2).try_into().unwrap();
    chunk.write_op(Op::Constant, 123);
    chunk.write(constant, 123);

    let constant = chunk.add_constant(3.4).try_into().unwrap();
    chunk.write_op(Op::Constant, 123);
    chunk.write(constant, 123);

    chunk.write_op(Op::Add, 123);

    let constant = chunk.add_constant(5.6).try_into().unwrap();
    chunk.write_op(Op::Constant, 123);
    chunk.write(constant, 123);

    chunk.write_op(Op::Divide, 123);
    chunk.write_op(Op::Negate, 123);

    chunk.write_op(Op::Return, 123);
    chunk.disassemble("test chunk");

    vm.interpret(chunk);
}

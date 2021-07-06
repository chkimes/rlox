mod chunk;
mod debug;
mod value;

use chunk::*;
use std::convert::TryInto;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2).try_into().unwrap();
    chunk.write_op(Op::CONSTANT, 123);
    chunk.write(constant, 123);

    chunk.write_op(Op::RETURN, 123);
    chunk.disassemble("test chunk")
}

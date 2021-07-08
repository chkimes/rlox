use crate::value::*;
use num_derive::FromPrimitive;
use num_derive::ToPrimitive;
use num_traits::ToPrimitive;
use std::vec::Vec;

#[derive(FromPrimitive, ToPrimitive)]
pub enum Op {
    Constant,
    Nil,
    False,
    True,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Negate,
    Return,
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<u16>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: u16) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_op(&mut self, op: Op, line: u16) {
        self.write(ToPrimitive::to_u8(&op).unwrap(), line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.values.push(value);
        return self.constants.values.len() - 1;
    }
}

use num_traits::FromPrimitive;

use crate::chunk::*;
use crate::value::*;

impl Chunk {
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
          print!("   | ");
        } else {
          print!("{:4} ", self.lines[offset]);
        }

        let instruction = self.code[offset];
        match FromPrimitive::from_u8(instruction) {
            Some(Op::RETURN)   => simple_instruction("OP_RETURN", offset),
            Some(Op::CONSTANT) => constant_instruction("OP_CONSTANT", self, offset),
            None               => { println!("Unknown opcode: {}", instruction); offset + 1 }
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];

    print!("{:16} {:4} '", name, constant);
    print_value(chunk.constants.values[constant as usize]);
    println!("'");

    return offset + 2;
}
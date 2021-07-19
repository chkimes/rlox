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

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }

        let instruction = self.code[offset];
        match FromPrimitive::from_u8(instruction) {
            Some(Op::Constant) => constant_instruction("OP_CONSTANT", self, offset),
            Some(Op::Nil)      => simple_instruction("OP_NIL", offset),
            Some(Op::False)    => simple_instruction("OP_FALSE", offset),
            Some(Op::True)     => simple_instruction("OP_TRUE", offset),
            Some(Op::Equal)    => simple_instruction("OP_EQUAL", offset),
            Some(Op::Greater)  => simple_instruction("OP_GREATER", offset),
            Some(Op::Less)     => simple_instruction("OP_LESS", offset),
            Some(Op::Add)      => simple_instruction("OP_ADD", offset),
            Some(Op::Subtract) => simple_instruction("OP_SUBTRACT", offset),
            Some(Op::Multiply) => simple_instruction("OP_MULTIPLY", offset),
            Some(Op::Divide)   => simple_instruction("OP_DIVIDE", offset),
            Some(Op::Not)      => simple_instruction("OP_NOT", offset),
            Some(Op::Negate)   => simple_instruction("OP_NEGATE", offset),
            Some(Op::Return)   => simple_instruction("OP_RETURN", offset),
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
    print_value(&chunk.constants.values[constant as usize]);
    println!("'");

    return offset + 2;
}

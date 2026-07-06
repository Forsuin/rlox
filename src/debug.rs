use crate::chunk::Chunk;
use crate::tokens::OpCode;
use crate::values::print_value;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 && chunk.get_line(offset) == chunk.get_line(offset - 1) {
        print!("   | ");
    }
    else {
        print!("{:4} ", chunk.get_line(offset));
    }

    let instruction = chunk.get(offset);

    match instruction {
        OpCode::CONSTANT {idx} => {
            constant_instruction("OP_CONSTANT", chunk, offset, *idx)
        }
        OpCode::ADD => {simple_instruction("OP_ADD", offset)}
        OpCode::SUBTRACT => {simple_instruction("OP_SUBTRACT", offset)}
        OpCode::MULTIPLY => {simple_instruction("OP_MULTIPLY", offset)}
        OpCode::DIVIDE => {simple_instruction("OP_DIVIDE", offset)}
        OpCode::NEGATE => {
            simple_instruction("OP_NEGATE", offset)
        }
        OpCode::RETURN => {
            simple_instruction("OP_RETURN", offset)
        }
    }


}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize, index: usize) -> usize{
    print!("{:<16} {:4} '", name, index);
    print_value(chunk.get_constant(index));
    println!("'");
    offset + 1
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}
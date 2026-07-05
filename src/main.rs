use rlox::chunk::Chunk;
use rlox::debug::disassemble_chunk;
use rlox::tokens::OpCode;
use rlox::vm::VM;

fn main() {
    let mut vm = VM::new();

    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    
    chunk.push(OpCode::CONSTANT {idx: constant}, 123);
    
    chunk.push(OpCode::RETURN, 123);
    disassemble_chunk(&chunk, "test chunk");

    vm.interpret(&chunk);
}
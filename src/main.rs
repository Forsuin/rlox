use rlox::chunk::Chunk;
use rlox::debug::disassemble_chunk;
use rlox::tokens::OpCode;
use rlox::vm::VM;

fn main() {
    let mut vm = VM::new();

    let mut chunk = Chunk::new();

    // return - ((1.2 + 3.4) / 5.6)

    let mut constant = chunk.add_constant(1.2);

    chunk.push(OpCode::CONSTANT {idx: constant}, 123);

    constant = chunk.add_constant(3.4);
    chunk.push(OpCode::CONSTANT { idx: constant }, 123);

    chunk.push(OpCode::ADD, 123);

    constant = chunk.add_constant(5.6);
    chunk.push(OpCode::CONSTANT { idx: constant}, 123);

    chunk.push(OpCode::DIVIDE, 123);

    chunk.push(OpCode::NEGATE, 123);


    chunk.push(OpCode::RETURN, 123);
    disassemble_chunk(&chunk, "test chunk");

    vm.interpret(&chunk);
}
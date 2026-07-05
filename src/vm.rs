use crate::chunk::Chunk;
use crate::debug::disassemble_instruction;
use crate::tokens::OpCode;
use crate::values::{print_value, Value};

pub enum InterpretResult {
    OK,
    CompileError,
    RuntimeError,
}

pub struct VM<'vm>{
    chunk: Option<&'vm Chunk>,
    ip: usize,
    stack: Vec<Value>,
}

impl<'vm> VM<'vm> {
    pub fn new() -> Self {
        Self {
            chunk: None,
            ip: 0,
            stack: vec![],
        }
    }

    pub fn interpret(&mut self, chunk: &'vm Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        let instruction;

        let chunk = self.chunk.expect("Missing chunk");

        #[cfg(feature = "trace-execution")]
        {
            print!("          ");

            for value in self.stack.iter() {
                print!("[ ");
                print_value(value);
                print!(" ]");
            }

            print!("\n");

            disassemble_instruction(chunk, self.ip);

        }


        loop {
            instruction = chunk.get(self.ip);
            self.ip += 1;

            match instruction {
                OpCode::CONSTANT { idx } => {
                    let constant = chunk.get_constant(*idx);
                    self.push(*constant);
                    break;
                }
                OpCode::RETURN => {
                    print_value(&self.pop());
                    print!("\n");
                    return InterpretResult::OK;
                }
            }
        }

        InterpretResult::RuntimeError
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("VM stack should not be empty")
    }
}

use crate::chunk::Chunk;
use crate::tokens::OpCode;
use crate::values::{print_value, Value};

#[cfg(feature = "trace-execution")]
use crate::debug::disassemble_instruction;

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
        let mut instruction;

        let chunk = self.chunk.expect("Missing chunk");

        loop {
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

            instruction = chunk.get(self.ip);
            self.ip += 1;

            match instruction {
                OpCode::CONSTANT { idx } => {
                    let constant = chunk.get_constant(*idx);
                    self.push(*constant);
                },
                OpCode::NEGATE => {
                    let neg_val = -self.pop();
                    self.push(neg_val);
                }
                OpCode::RETURN => {
                    let val = self.pop();
                    print_value(&val);
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

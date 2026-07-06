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
                OpCode::ADD => {self.binary_op(|a, b| a + b)}
                OpCode::SUBTRACT => {self.binary_op(|a, b| a - b)}
                OpCode::MULTIPLY => {self.binary_op(|a, b| a * b)}
                OpCode::DIVIDE => {self.binary_op(|a, b| a / b)}
                OpCode::NEGATE => {
                    self.stack.last_mut().map(|val| -*val);
                }
                OpCode::RETURN => {
                    let val = self.pop();
                    print_value(&val);
                    println!();
                    return InterpretResult::OK;
                }
            }
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("VM stack should not be empty")
    }

    fn binary_op(&mut self, op: fn(Value, Value) -> Value) {
        let b = self.pop();
        let a = self.pop();
        self.push(op(a, b));
    }
}

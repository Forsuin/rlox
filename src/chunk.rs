use crate::tokens::OpCode;
use crate::values::Value;

#[derive(Debug)]
pub struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            constants: vec![],
            lines: vec![],
        }
    }

    pub fn push(&mut self, op_code: OpCode, line: usize) {
        self.code.push(op_code);
        self.lines.push(line);
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn get(&self, offset: usize) -> &OpCode {
        self.code.get(offset).unwrap()
    }

    pub fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, offset: usize) -> &Value {
        self.constants.get(offset).unwrap()
    }

    pub fn get_line(&self, offset: usize) -> usize {
        self.lines.get(offset).unwrap().clone()
    }
}
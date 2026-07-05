#[derive(Debug)]
pub enum OpCode {
    CONSTANT {idx: usize},
    NEGATE,
    RETURN,
}
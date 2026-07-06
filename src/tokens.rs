#[derive(Debug)]
pub enum OpCode {
    CONSTANT {idx: usize},
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    NEGATE,
    RETURN,
}
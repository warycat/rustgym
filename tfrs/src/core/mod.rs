mod shape;
mod tensor;
mod tensor_flow;

pub use shape::*;
pub use tensor::*;
pub use tensor_flow::*;

pub const NUM_CORES: usize = 2;

pub enum DataType {
    F32,
    I32,
    Bool,
}

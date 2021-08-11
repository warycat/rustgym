mod shape;
mod tensor;
mod tensor_data;
mod tensor_flow;

pub use shape::*;
pub use tensor::*;
pub use tensor_data::*;
pub use tensor_flow::*;

pub const NUM_CORES: u64 = 2;

pub enum DataType {
    F32,
    I32,
    Bool,
}

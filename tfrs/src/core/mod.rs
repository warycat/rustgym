mod tensor;
mod tensor_data;
mod tensor_flow;

pub use tensor::*;
pub use tensor_data::*;
pub use tensor_flow::*;

pub type TensorId = u64;
pub type Shape = Vec<usize>;

pub trait ShapeLike {
    fn tensor_size(&self) -> usize;
    fn rank(&self) -> usize;
}

impl ShapeLike for Shape {
    fn tensor_size(&self) -> usize {
        self.iter().product()
    }
    fn rank(&self) -> usize {
        self.len()
    }
}

pub const NUM_CORES: u64 = 2;

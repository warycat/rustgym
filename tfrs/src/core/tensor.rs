use crate::core::TensorData;
use crate::core::{Shape, ShapeLike};

pub type TensorId = u64;

pub struct Tensor {
    data: TensorData,
    shape: Vec<usize>,
    disposal_callbacks: Vec<Box<dyn FnMut()>>,
}

impl PartialEq for Tensor {
    fn eq(&self, other: &Tensor) -> bool {
        self.shape == other.shape && self.data == other.data
    }
}

impl std::fmt::Debug for Tensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tensor")
    }
}

impl Tensor {
    pub fn new(data: TensorData, shape: Shape) -> Self {
        assert_eq!(data.size(), shape.tensor_size());
        let disposal_callbacks = vec![];
        Tensor {
            data,
            shape,
            disposal_callbacks,
        }
    }

    pub fn data(&self) -> &TensorData {
        &self.data
    }

    pub fn shape(&self) -> &Vec<usize> {
        &self.shape
    }

    pub fn buf<T>(&self) -> *const T {
        self.data.buf() as *const T
    }

    pub fn buf_mut<T>(&self) -> *mut T {
        self.data.buf_mut() as *mut T
    }

    pub fn size(&self) -> usize {
        self.data.size()
    }

    pub fn register_disposal_callback(&mut self, dispose_fn: Box<dyn FnMut()>) {
        self.disposal_callbacks.push(Box::new(dispose_fn));
    }
}

impl Drop for Tensor {
    fn drop(&mut self) {
        for mut callback in self.disposal_callbacks.drain(..) {
            callback();
        }
    }
}

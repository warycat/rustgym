use crate::core::*;

impl TensorFlow {
    pub fn tensor1d<T: TensorValue>(&mut self, values: Vec<T>) -> TensorId {
        let shape = vec![values.len()];
        let tensor = Tensor::new(values, shape);
        self.register(tensor)
    }
}

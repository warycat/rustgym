use crate::core::*;

impl TensorFlow {
    pub fn scalar<T: TensorValue>(&mut self, value: T) -> TensorId {
        let tensor = Tensor::new(vec![value], vec![]);
        self.register(tensor)
    }
}

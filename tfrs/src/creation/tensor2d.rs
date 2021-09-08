use crate::core::*;

impl TensorFlow {
    pub fn tensor2d<T: TensorValue>(&mut self, values: Vec<T>, shape: Shape) -> TensorId {
        assert_eq!(shape.len(), 2);
        let tensor_data = Tensor::new(values, shape);
        self.register(tensor_data)
    }
}

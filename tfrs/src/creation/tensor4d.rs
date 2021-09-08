use crate::core::*;

impl TensorFlow {
    pub fn tensor4d<T: TensorValue>(&mut self, values: Vec<T>, shape: Shape) -> TensorId {
        assert_eq!(shape.len(), 4);
        let tensor = Tensor::new(values, shape);
        self.register(tensor)
    }
}

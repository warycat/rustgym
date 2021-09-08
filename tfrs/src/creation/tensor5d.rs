use crate::core::*;

impl TensorFlow {
    pub fn tensor5d<T: TensorValue>(&mut self, values: Vec<T>, shape: Shape) -> TensorId {
        assert_eq!(shape.len(), 5);
        let tensor = Tensor::new(values, shape);
        self.register(tensor)
    }
}

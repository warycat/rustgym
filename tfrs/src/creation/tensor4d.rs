use crate::core::{Shape, TensorData, TensorFlow, TensorId};

impl TensorFlow {
    pub fn tensor4d(&mut self, values: Vec<f32>, shape: Shape) -> TensorId {
        assert_eq!(shape.len(), 4);
        let tensor_data = TensorData::F32(values);
        self.register_tensor(tensor_data, shape)
    }
}

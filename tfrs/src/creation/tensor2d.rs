use crate::core::{Shape, TenserFlow, TensorData, TensorId};

impl TenserFlow {
    pub fn tensor2d(&mut self, values: Vec<f32>, shape: Shape) -> TensorId {
        assert_eq!(shape.len(), 2);
        let tensor_data = TensorData::F32(values);
        self.register_tensor(tensor_data, shape)
    }
}

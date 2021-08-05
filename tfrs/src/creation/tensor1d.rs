use crate::core::{TensorData, TensorFlow, TensorId};

impl TensorFlow {
    pub fn tensor1d(&mut self, values: Vec<f32>) -> TensorId {
        let shape = vec![values.len()];
        let tensor_data = TensorData::F32(values);
        self.register_tensor(tensor_data, shape)
    }
}

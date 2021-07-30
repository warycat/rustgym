use crate::{TenserFlow, TensorData, TensorId};

impl TenserFlow {
    pub fn scalar(&mut self, value: f32) -> TensorId {
        let tensor_data = TensorData::F32(vec![value]);
        self.register_tensor(tensor_data, vec![])
    }
}

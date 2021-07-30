use crate::core::*;

impl TenserFlow {
    pub fn buffer(&mut self, shape: Shape) -> TensorData {
        let shape = vec![0.0; values.size()];
        TensorData::F32(values)
    }
}

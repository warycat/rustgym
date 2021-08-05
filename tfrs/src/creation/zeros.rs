use crate::{Shape, ShapeLike, TensorData, TensorFlow, TensorId};

impl TensorFlow {
    pub fn zeros(&mut self, shape: Shape) -> TensorId {
        let values = vec![0.0; shape.tensor_size()];
        self.register_tensor(TensorData::F32(values), shape)
    }
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let a = tf.zeros(vec![2, 2]);
    let tensor = tf.get_tensor_info(a);
    assert_eq!(tensor.shape(), &vec![2, 2]);
    assert_eq!(tensor.data(), &TensorData::F32(vec![0.0, 0.0, 0.0, 0.0]));
}

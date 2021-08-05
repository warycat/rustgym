use crate::{Shape, ShapeLike, TensorData, TensorFlow, TensorId};

impl TensorFlow {
    pub fn fill(&mut self, shape: Shape, val: f32) -> TensorId {
        let values = vec![val; shape.tensor_size()];
        self.register_tensor(TensorData::F32(values), shape)
    }
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let a = tf.fill(vec![3, 2, 1], 2.0);
    let tensor = tf.get_tensor_info(a);
    assert_eq!(tensor.shape(), &vec![3, 2, 1]);
    assert_eq!(
        tensor.data(),
        &TensorData::F32(vec![2.0, 2.0, 2.0, 2.0, 2.0, 2.0])
    );
}

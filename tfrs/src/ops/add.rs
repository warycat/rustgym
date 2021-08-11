use crate::core::*;
use crate::ops::binary::add;

impl TensorFlow {
    pub fn add(&mut self, a_id: TensorId, b_id: TensorId) -> TensorId {
        let a = self.get_tensor_info(a_id);
        let b = self.get_tensor_info(b_id);
        let (output, shape) = add(a, b, self.threadpool);
        self.register_tensor(output, shape)
    }
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let a = tf.tensor1d(vec![1.0, 2.0, 3.0, 4.0]);
    let b = tf.tensor1d(vec![10.0, 20.0, 30.0, 40.0]);
    let c = tf.add(a, b);
    let cc = tf.get_tensor_info(c);
    assert_eq!(cc.data(), &TensorData::F32(vec![11.0, 22.0, 33.0, 44.0]));
    assert_eq!(cc.shape(), &vec![4]);
}

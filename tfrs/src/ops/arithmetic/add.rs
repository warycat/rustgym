use crate::core::*;
use crate::ops::binary::add;

impl TensorFlow {
    pub fn add(&mut self, a_id: TensorId, b_id: TensorId) -> TensorId {
        let a = self.get_f32(a_id);
        let b = self.get_f32(b_id);
        let c = add(a, b, self.threadpool);
        self.register(c)
    }
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let a_id = tf.tensor1d(vec![1.0, 2.0, 3.0, 4.0]);
    let b_id = tf.tensor1d(vec![10.0, 20.0, 30.0, 40.0]);
    let c_id = tf.add(a_id, b_id);
    let d_id = tf.tensor1d(vec![11.0, 22.0, 33.0, 44.000001]);
    let c = tf.get_f32(c_id);
    let d = tf.get_f32(d_id);
    assert_tensor_f32_eq!(c, d);
}

use crate::core::*;
use crate::ops::unary::sigmoid;

impl TensorFlow {
    pub fn sigmoid(&mut self, x_id: TensorId) -> TensorId {
        let x = self.get_f32(x_id);
        let y = sigmoid(x, self.threadpool);
        self.register(y)
    }
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let x_id = tf.tensor1d(vec![1.0, -3.0, 2.0, 7.0, -4.0]);
    let y_id = tf.sigmoid(x_id);
    let z_id = tf.tensor1d(vec![
        0.7310586,
        0.047425874,
        0.8807971,
        0.99908894,
        0.017986208,
    ]);
    let y = tf.get_f32(y_id);
    let z = tf.get_f32(z_id);
    assert_eq!(y, z);
}

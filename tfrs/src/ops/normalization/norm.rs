use crate::core::*;

impl TensorFlow {
    pub fn norm(&mut self, x_id: TensorId) -> TensorId {
        let x = self.get(x_id);
        let x_shape = x.shape();
        let rank = x_shape.rank();
        assert!(rank < 3);
        let x_data = x.as_f32().data();
        if rank == 0 {
            let y = x_data[0];
            return self.scalar(y.abs());
        }
        let n = x_shape.tensor_size();
        let mut sum = 0.0;
        for i in 0..n {
            let y = x_data[i];
            sum += y * y;
        }
        self.scalar(sum.sqrt())
    }
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();

    let x_id = tf.tensor1d(vec![1.0, -2.0, 3.0, -4.0]);
    let y_id = tf.norm(x_id);
    let z_id = tf.scalar(5.477226);
    let z = tf.get_f32(z_id);
    let y = tf.get_f32(y_id);
    assert_eq!(y, z);

    let x_id = tf.tensor2d(vec![1.0, 2.0, -3.0, 1.0, 1.0, 1.0], vec![3, 2]);
    let y_id = tf.norm(x_id);
    let z_id = tf.scalar(4.1231055);
    let z = tf.get_f32(z_id);
    let y = tf.get_f32(y_id);
    assert_eq!(y, z);
}

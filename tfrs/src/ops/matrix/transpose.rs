use crate::core::*;

impl TensorFlow {
    pub fn transpose(&mut self, x_id: TensorId) -> TensorId {
        use TensorId::*;
        let x = self.get(x_id);
        assert_eq!(x.shape().rank(), 2);
        self.register(match x_id {
            F32(_) => transpose(self.get_f32(x_id)),
            I32(_) => transpose(self.get_i32(x_id)),
            Bool(_) => transpose(self.get_bool(x_id)),
        })
    }
}

fn transpose<T: TensorValue>(x: &Tensor<T>) -> Box<Tensor<T>> {
    let x_shape = x.shape();
    let d0 = x_shape[0];
    let d1 = x_shape[1];
    let out_shape = vec![d1, d0];
    let x_data = x.data();
    let mut values = vec![T::default(); x_shape.tensor_size()];
    for i in 0..d0 {
        for j in 0..d1 {
            values[j * d0 + i] = x_data[i * d1 + j];
        }
    }
    Tensor::new(values, out_shape)
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let x_id = tf.tensor2d(vec![1, 2, 3, 4, 5, 6], vec![2, 3]);
    let y_id = tf.transpose(x_id);
    let z_id = tf.tensor2d(vec![1, 4, 2, 5, 3, 6], vec![3, 2]);
    let y = tf.get_i32(y_id);
    let z = tf.get_i32(z_id);
    assert_eq!(y, z);
}

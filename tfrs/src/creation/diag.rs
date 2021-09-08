use crate::core::*;

impl TensorFlow {
    pub fn diag(&mut self, id: TensorId) -> TensorId {
        use TensorId::*;
        self.register(match id {
            F32(_) => diag(self.get(id).as_f32()),
            I32(_) => diag(self.get(id).as_i32()),
            Bool(_) => diag(self.get(id).as_bool()),
        })
    }
}

fn diag<T: TensorValue>(tensor: &Tensor<T>) -> Box<Tensor<T>> {
    let data = tensor.data();
    let shape = tensor.shape();
    let size = data.len();
    let mut out_values = vec![T::default(); size * size];
    for i in 0..size {
        out_values[i + size * i] = data[i];
    }
    let mut out_shape = vec![];
    out_shape.extend(shape);
    out_shape.extend(shape);
    Tensor::new(out_values, out_shape)
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let a_id = tf.tensor1d(vec![5.0]);
    let b_id = tf.diag(a_id);
    let b = tf.get(b_id);
    let c = Tensor::new(vec![5.0], vec![1, 1]);
    assert_eq!(b.as_f32(), c.as_ref());

    let a_id = tf.tensor2d(vec![8.0, 2.0, 3.0, 4.0, 5.0, 1.0], vec![3, 2]);
    let b_id = tf.diag(a_id);
    let b = tf.get(b_id);
    let c = Tensor::new(
        vec![
            8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 1.0,
        ],
        vec![3, 2, 3, 2],
    );
    assert_eq!(b.as_f32(), c.as_ref());

    let a_id = tf.tensor3d(
        vec![
            8.0, 5.0, 5.0, 7.0, 9.0, 10.0, 15.0, 1.0, 2.0, 14.0, 12.0, 3.0,
        ],
        vec![2, 2, 3],
    );
    let b_id = tf.diag(a_id);
    let b = tf.get(b_id);
    let c = Tensor::new(
        vec![
            8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 9.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 15.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 14.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 12.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0,
        ],
        vec![2, 2, 3, 2, 2, 3],
    );
    assert_eq!(b.as_f32(), c.as_ref());
}

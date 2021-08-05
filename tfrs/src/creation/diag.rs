use crate::{TensorData, TensorFlow, TensorId};

impl TensorFlow {
    pub fn diag(&mut self, id: TensorId) -> TensorId {
        let tensor = self.get_tensor_info(id);
        let data = tensor.data();
        let shape = tensor.shape();
        let size = data.size();
        let mut out_values = vec![0.0; size * size];
        for i in 0..size {
            out_values[i + size * i] = data.get_f32(i);
        }
        let mut out_shape = vec![];
        out_shape.extend(shape);
        out_shape.extend(shape);
        self.register_tensor(TensorData::F32(out_values), out_shape)
    }
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let a = tf.tensor1d(vec![5.0]);
    let b = tf.diag(a);
    let tensor = tf.get_tensor_info(b);
    assert_eq!(tensor.data(), &TensorData::F32(vec![5.0]));
    assert_eq!(tensor.shape(), &vec![1, 1]);

    let a = tf.tensor2d(vec![8.0, 2.0, 3.0, 4.0, 5.0, 1.0], vec![3, 2]);
    let b = tf.diag(a);
    let tensor = tf.get_tensor_info(b);
    assert_eq!(
        tensor.data(),
        &TensorData::F32(vec![
            8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 1.0
        ])
    );
    assert_eq!(tensor.shape(), &vec![3, 2, 3, 2]);

    let a = tf.tensor3d(
        vec![
            8.0, 5.0, 5.0, 7.0, 9.0, 10.0, 15.0, 1.0, 2.0, 14.0, 12.0, 3.0,
        ],
        vec![2, 2, 3],
    );
    let b = tf.diag(a);
    let tensor = tf.get_tensor_info(b);
    assert_eq!(
        tensor.data(),
        &TensorData::F32(vec![
            8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 9.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 15.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 14.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 12.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0,
        ])
    );
    assert_eq!(tensor.shape(), &vec![2, 2, 3, 2, 2, 3]);
}

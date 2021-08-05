use crate::{TensorData, TensorFlow, TensorId};

impl TensorFlow {
    pub fn eye(&mut self, n: usize) -> TensorId {
        let mut values = vec![0.0; n * n];
        for i in 0..n {
            values[i * n + i] = 1.0;
        }
        let shape = vec![n, n];
        self.register_tensor(TensorData::F32(values), shape)
    }
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let a = tf.eye(3);
    let tensor = tf.get_tensor_info(a);
    assert_eq!(tensor.shape(), &vec![3, 3]);
    assert_eq!(
        tensor.data(),
        &TensorData::F32(vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
    );
}

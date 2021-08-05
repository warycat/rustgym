use crate::{TensorFlow, TensorId};

impl TensorFlow {
    pub fn clone(&mut self, id: TensorId) -> TensorId {
        let tensor = self.get_tensor_info(id);
        let data = tensor.data().clone();
        let shape = tensor.shape().clone();
        self.register_tensor(data, shape)
    }
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let a = tf.tensor2d(
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
        vec![3, 3],
    );
    let b = tf.clone(a);
    assert_eq!(tf.get_tensor_info(a), tf.get_tensor_info(b));
}

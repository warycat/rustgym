use crate::core::*;

impl TensorFlow {
    pub fn eye(&mut self, n: usize) -> TensorId {
        let mut values = vec![0.0; n * n];
        for i in 0..n {
            values[i * n + i] = 1.0;
        }
        let shape = vec![n, n];
        let tensor = Tensor::new(values, shape);
        self.register(tensor)
    }
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let a_id = tf.eye(3);
    let a = tf.get(a_id);
    let b = Tensor::new(
        vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        vec![3, 3],
    );
    assert_eq!(a.as_f32(), b.as_ref());
}

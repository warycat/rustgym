use crate::core::*;

impl TensorFlow {
    pub fn ones(&mut self, shape: Shape) -> TensorId {
        let values = vec![1.0; shape.tensor_size()];
        let tensor = Tensor::new(values, shape);
        self.register(tensor)
    }
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let a_id = tf.ones(vec![2, 2]);
    let a = tf.get(a_id);
    let b = Tensor::new(vec![1.0, 1.0, 1.0, 1.0], vec![2, 2]);
    assert_eq!(a.as_f32(), b.as_ref());
}

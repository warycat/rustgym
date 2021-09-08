use crate::core::*;

impl TensorFlow {
    pub fn fill(&mut self, shape: Shape, val: f32) -> TensorId {
        let values = vec![val; shape.tensor_size()];
        let tensor = Tensor::new(values, shape);
        self.register(tensor)
    }
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let a_id = tf.fill(vec![3, 2, 1], 2.0);
    let a = tf.get(a_id);
    let b = Tensor::new(vec![2.0, 2.0, 2.0, 2.0, 2.0, 2.0], vec![3, 2, 1]);
    assert_eq!(a.as_f32(), b.as_ref());
}

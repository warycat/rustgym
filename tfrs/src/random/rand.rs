use crate::core::*;

impl TensorFlow {
    pub fn rand(&mut self, shape: Shape, mut rand_function: impl FnMut() -> f32) -> TensorId {
        let size = shape.tensor_size();
        let mut values = vec![0.0; size];
        for i in 0..size {
            values[i] = rand_function();
        }
        let tensor = Tensor::new(values, shape);
        self.register(tensor)
    }
}

#[test]
fn test() {
    use rand_distr::{Distribution, Normal};
    let mut tf = TensorFlow::default();
    let normal = Normal::new(2.0, 3.0).unwrap();
    let a_id = tf.rand(vec![5], || normal.sample(&mut rand::thread_rng()));
    let a = tf.get(a_id);
    assert_eq!(a.shape(), &vec![5]);
}

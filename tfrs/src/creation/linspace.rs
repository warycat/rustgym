use crate::core::*;

impl TensorFlow {
    pub fn linspace(&mut self, start: f32, stop: f32, num: usize) -> TensorId {
        assert!(num >= 2);
        let mut values = vec![0.0; num];
        let step = (stop - start) / (num - 1) as f32;
        values[0] = start;
        for i in 1..num {
            values[i] = values[i - 1] + step;
        }
        let tensor = Tensor::new(values, vec![num]);
        self.register(tensor)
    }
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let a_id = tf.linspace(12.0, 17.0, 8);
    let a = tf.get(a_id);
    let b = Tensor::new(
        vec![
            12.000000, 12.714286, 13.428572, 14.142858, 14.857143, 15.571429, 16.285715, 17.000000,
        ],
        vec![8],
    );
    assert_eq!(a.as_f32(), b.as_ref());
}

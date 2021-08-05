use crate::{TensorData, TensorFlow, TensorId};

impl TensorFlow {
    pub fn linspace(&mut self, start: f32, stop: f32, num: usize) -> TensorId {
        assert!(num >= 2);
        let mut values = vec![0.0; num];
        let step = (stop - start) / (num - 1) as f32;
        values[0] = start;
        for i in 1..num {
            values[i] = values[i - 1] + step;
        }
        self.register_tensor(TensorData::F32(values), vec![num])
    }
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let a = tf.linspace(12.0, 17.0, 8);
    let tensor = tf.get_tensor_info(a);
    assert_eq!(tensor.shape(), &vec![8]);
    assert_eq!(
        tensor.data(),
        &TensorData::F32(vec![
            12.000000, 12.714286, 13.428572, 14.142858, 14.857143, 15.571429, 16.285715, 17.000000,
        ])
    );
}

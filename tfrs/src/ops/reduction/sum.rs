use crate::core::*;

impl TensorFlow {
    pub fn sum(&mut self, x_id: TensorId) -> TensorId {
        use TensorId::*;
        match x_id {
            F32(_) => self.scalar(self.get_f32(x_id).data().iter().sum::<f32>()),
            I32(_) => self.scalar(self.get_i32(x_id).as_i32().data().iter().sum::<i32>()),
            Bool(_) => self.scalar(self.get(x_id).as_i32().data().iter().sum::<i32>()),
        }
    }
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let a_id = tf.tensor2d(vec![1, 2, 3, 0, 0, 1], vec![3, 2]);
    let sum_id = tf.sum(a_id);
    let res_id = tf.scalar(7);
    let sum = tf.get_i32(sum_id);
    let res = tf.get_i32(res_id);
    assert_eq!(sum, res);
}

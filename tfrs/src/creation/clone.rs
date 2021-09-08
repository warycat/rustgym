use crate::core::*;

impl TensorFlow {
    pub fn clone(&mut self, id: TensorId) -> TensorId {
        use TensorId::*;
        let tensor = self.get(id);
        match id {
            F32(_) => {
                let clone: Tensor<f32> = tensor.as_f32().clone();
                self.register(Box::new(clone))
            }
            I32(_) => {
                let clone: Tensor<i32> = tensor.as_i32().clone();
                self.register(Box::new(clone))
            }
            Bool(_) => {
                let clone: Tensor<bool> = tensor.as_bool().clone();
                self.register(Box::new(clone))
            }
        }
    }
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let a_id = tf.tensor2d(
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
        vec![3, 3],
    );
    let b_id = tf.clone(a_id);
    let a = tf.get(a_id);
    let b = tf.get(b_id);
    assert_eq!(a.as_f32(), b.as_f32());
}

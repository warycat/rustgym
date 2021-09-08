use crate::core::*;

impl TensorFlow {
    pub fn concat(&mut self, tensors: Vec<TensorId>, axis: usize) -> TensorId {
        use TensorId::*;
        assert!(!tensors.is_empty());
        let first_id = tensors[0];
        self.register(match first_id {
            F32(_) => {
                let tensors_f32: Vec<&Tensor<f32>> =
                    tensors.iter().map(|&id| self.get(id).as_f32()).collect();
                concat(tensors_f32, axis)
            }
            I32(_) => {
                let tensors_i32: Vec<&Tensor<i32>> =
                    tensors.iter().map(|&id| self.get(id).as_i32()).collect();
                concat(tensors_i32, axis)
            }
            Bool(_) => {
                let tensors_bool: Vec<&Tensor<bool>> =
                    tensors.iter().map(|&id| self.get(id).as_bool()).collect();
                concat(tensors_bool, axis)
            }
        })
    }

    pub fn concat1d(&mut self, tensors: Vec<TensorId>) -> TensorId {
        self.concat(tensors, 0)
    }
}

fn concat<T: TensorValue>(tensors: Vec<&Tensor<T>>, axis: usize) -> Box<Tensor<T>> {
    assert!(!tensors.is_empty());
    let size = tensors.len();
    let first_tensor = tensors[0];
    let first_shape = first_tensor.shape();
    let rank = first_shape.len();
    let mut out_shape = first_shape.clone();
    for i in 1..size {
        let tensor = tensors[i];
        let shape = tensor.shape();
        for j in 0..rank {
            if j == axis {
                out_shape[axis] += shape[axis];
            } else {
                assert!(shape[j] == first_shape[j]);
            }
        }
    }
    let mut out_values = vec![T::default(); out_shape.tensor_size()];
    let mut offset = 0;
    for i in 0..size {
        let tensor = tensors[i];
        let data = tensor.data();
        let shape = tensor.shape();
        out_values[offset..(offset + shape.tensor_size())]
            .clone_from_slice(&data[..shape.tensor_size()]);
        offset += tensor.size();
    }
    Tensor::new(out_values, out_shape)
}

#[test]
fn test_concat1d() {
    let mut tf = TensorFlow::default();

    let a_id = tf.tensor1d(vec![3]);
    let b_id = tf.tensor1d(vec![5]);
    let c_id = tf.concat1d(vec![a_id, b_id]);
    let c = tf.get(c_id);
    let d = Tensor::new(vec![3, 5], vec![2]);
    assert_eq!(c.as_i32(), d.as_ref());

    let a_id = tf.tensor1d(vec![3]);
    let b_id = tf.tensor1d(vec![5, 7]);
    let c_id = tf.concat1d(vec![a_id, b_id]);
    let c = tf.get(c_id);
    let d = Tensor::new(vec![3, 5, 7], vec![3]);
    assert_eq!(c.as_i32(), d.as_ref());

    let a_id = tf.tensor1d(vec![3]);
    let b_id = tf.tensor1d(vec![5]);
    let c_id = tf.tensor1d(vec![7]);
    let d_id = tf.tensor1d(vec![9]);
    let e_id = tf.concat1d(vec![a_id, b_id, c_id, d_id]);
    let e = tf.get(e_id);
    let f = Tensor::new(vec![3, 5, 7, 9], vec![4]);
    assert_eq!(e.as_i32(), f.as_ref());

    let a_id = tf.tensor1d(vec![3]);
    let b_id = tf.concat1d(vec![a_id]);
    let b = tf.get(b_id);
    let c = Tensor::new(vec![3], vec![1]);
    assert_eq!(b.as_i32(), c.as_ref());

    let a_id = tf.tensor2d(vec![3], vec![1, 1]);
    let b_id = tf.tensor2d(vec![5], vec![1, 1]);
    let axis = 0;
    let c_id = tf.concat(vec![a_id, b_id], axis);
    let c = tf.get(c_id);
    let d = Tensor::new(vec![3, 5], vec![2, 1]);
    assert_eq!(c.as_i32(), d.as_ref());

    let a_id = tf.tensor2d(vec![3], vec![1, 1]);
    let b_id = tf.tensor2d(vec![5], vec![1, 1]);
    let axis = 1;
    let c_id = tf.concat(vec![a_id, b_id], axis);
    let c = tf.get(c_id);
    let d = Tensor::new(vec![3, 5], vec![1, 2]);
    assert_eq!(c.as_i32(), d.as_ref());
}

#[test]
fn test_concat2d() {
    let mut tf = TensorFlow::default();

    let a_id = tf.tensor2d(vec![1, 2, 3, 4], vec![2, 2]);
    let b_id = tf.tensor2d(vec![5, 6], vec![1, 2]);
    let c_id = tf.tensor2d(vec![7, 8], vec![1, 2]);
    let axis = 0;
    let d_id = tf.concat(vec![a_id, b_id, c_id], axis);
    let d = tf.get(d_id);
    let e = Tensor::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![4, 2]);
    assert_eq!(d.as_i32(), e.as_ref());

    let a_id = tf.tensor2d(vec![1, 2, 3, 4], vec![2, 2]);
    let b_id = tf.tensor2d(vec![5, 6, 7, 8], vec![2, 2]);
    let c_id = tf.tensor2d(vec![9, 10, 11, 12], vec![2, 2]);
    let axis = 1;
    let d_id = tf.concat(vec![a_id, b_id, c_id], axis);
    let d = tf.get(d_id);
    let e = Tensor::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], vec![2, 6]);
    assert_eq!(d.as_i32(), e.as_ref());

    let a_id = tf.tensor2d(vec![3], vec![1, 1]);
    let b_id = tf.tensor2d(vec![5], vec![1, 1]);
    let axis = 0;
    let c_id = tf.concat(vec![a_id, b_id], axis);
    let c = tf.get(c_id);
    let d = Tensor::new(vec![3, 5], vec![2, 1]);
    assert_eq!(c.as_i32(), d.as_ref());

    let a_id = tf.tensor2d::<i32>(vec![], vec![0, 5]);
    let b_id = tf.tensor2d::<i32>(vec![], vec![0, 5]);
    let c_id = tf.tensor2d::<i32>(vec![], vec![0, 5]);
    let axis = 0;
    let d_id = tf.concat(vec![a_id, b_id, c_id], axis);
    let d = tf.get(d_id);
    let e = Tensor::new(vec![], vec![0, 5]);
    assert_eq!(d.as_i32(), e.as_ref());
    let axis = 1;
    let d_id = tf.concat(vec![a_id, b_id, c_id], axis);
    let d = tf.get(d_id);
    let e = Tensor::new(vec![], vec![0, 15]);
    assert_eq!(d.as_i32(), e.as_ref());
}

use crate::core::*;

impl TenserFlow {
    pub fn concat(&mut self, tensors: Vec<TensorId>, axis: usize) -> TensorId {
        assert!(tensors.len() > 0);
        let size = tensors.len();
        let first_id = tensors[0];
        let first_tensor = self.get_tensor_info(first_id).expect("tensor");
        let first_shape = first_tensor.shape();
        let rank = first_shape.len();
        let mut out_shape = first_shape.clone();
        for i in 1..size {
            let tensor_id = tensors[i];
            let tensor = self.get_tensor_info(tensor_id).expect("tensor");
            let shape = tensor.shape();
            for j in 0..rank {
                if j == axis {
                    out_shape[axis] += shape[axis];
                } else {
                    assert!(shape[j] == first_shape[j]);
                }
            }
        }
        let mut out_values = vec![0.0; out_shape.tensor_size()];
        let mut offset = 0;
        for i in 0..size {
            let id = tensors[i];
            let tensor = self.get_tensor_info(id).expect("tensor");
            let values = tensor.data();
            let shape = tensor.shape();
            for j in 0..shape.tensor_size() {
                out_values[offset + j] = values.get_f32(j);
            }
            offset += tensor.size();
        }
        self.register_tensor(TensorData::F32(out_values), out_shape)
    }

    pub fn concat1d(&mut self, tensors: Vec<TensorId>) -> TensorId {
        self.concat(tensors, 0)
    }
}

#[test]
fn test_concat1d() {
    let mut tf = TenserFlow::new(NUM_CORES);

    let a = tf.tensor1d(vec![3.0]);
    let b = tf.tensor1d(vec![5.0]);
    let res = tf.concat1d(vec![a, b]);
    let res_tensor = tf.get_tensor_info(res).expect("tensor");
    let ans_data = TensorData::F32(vec![3.0, 5.0]);
    assert_eq!(res_tensor.data(), &ans_data);

    let a = tf.tensor1d(vec![3.0]);
    let b = tf.tensor1d(vec![5.0, 7.0]);
    let res = tf.concat1d(vec![a, b]);
    let res_tensor = tf.get_tensor_info(res).expect("tensor");
    let ans_data = TensorData::F32(vec![3.0, 5.0, 7.0]);
    assert_eq!(res_tensor.data(), &ans_data);

    let a = tf.tensor1d(vec![3.0]);
    let b = tf.tensor1d(vec![5.0]);
    let c = tf.tensor1d(vec![7.0]);
    let d = tf.tensor1d(vec![9.0]);
    let res = tf.concat1d(vec![a, b, c, d]);
    let res_tensor = tf.get_tensor_info(res).expect("tensor");
    let ans_data = TensorData::F32(vec![3.0, 5.0, 7.0, 9.0]);
    assert_eq!(res_tensor.data(), &ans_data);

    let a = tf.tensor1d(vec![3.0]);
    let res = tf.concat1d(vec![a]);
    let res_tensor = tf.get_tensor_info(res).expect("tensor");
    let ans_data = TensorData::F32(vec![3.0]);
    assert_eq!(res_tensor.data(), &ans_data);

    let a = tf.tensor2d(vec![3.0], vec![1, 1]);
    let b = tf.tensor2d(vec![5.0], vec![1, 1]);
    let axis = 0;
    let res = tf.concat(vec![a, b], axis);
    let res_tensor = tf.get_tensor_info(res).expect("tensor");
    let ans_data = TensorData::F32(vec![3.0, 5.0]);
    let ans_shape = vec![2, 1];
    assert_eq!(res_tensor.data(), &ans_data);
    assert_eq!(res_tensor.shape(), &ans_shape);

    let a = tf.tensor2d(vec![3.0], vec![1, 1]);
    let b = tf.tensor2d(vec![5.0], vec![1, 1]);
    let axis = 1;
    let res = tf.concat(vec![a, b], axis);
    let res_tensor = tf.get_tensor_info(res).expect("tensor");
    let ans_data = TensorData::F32(vec![3.0, 5.0]);
    let ans_shape = vec![1, 2];
    assert_eq!(res_tensor.data(), &ans_data);
    assert_eq!(res_tensor.shape(), &ans_shape);
}

#[test]
fn test_concat2d() {
    let mut tf = TenserFlow::new(NUM_CORES);

    let a = tf.tensor2d(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
    let b = tf.tensor2d(vec![5.0, 6.0], vec![1, 2]);
    let c = tf.tensor2d(vec![7.0, 8.0], vec![1, 2]);
    let axis = 0;
    let res = tf.concat(vec![a, b, c], axis);
    let res_tensor = tf.get_tensor_info(res).expect("tensor");
    let ans_data = TensorData::F32(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    let ans_shape = vec![4, 2];
    assert_eq!(res_tensor.data(), &ans_data);
    assert_eq!(res_tensor.shape(), &ans_shape);

    let a = tf.tensor2d(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
    let b = tf.tensor2d(vec![5.0, 6.0, 7.0, 8.0], vec![2, 2]);
    let c = tf.tensor2d(vec![9.0, 10.0, 11.0, 12.0], vec![2, 2]);
    let axis = 1;
    let res = tf.concat(vec![a, b, c], axis);
    let res_tensor = tf.get_tensor_info(res).expect("tensor");
    let ans_data = TensorData::F32(vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
    ]);
    let ans_shape = vec![2, 6];
    assert_eq!(res_tensor.data(), &ans_data);
    assert_eq!(res_tensor.shape(), &ans_shape);

    let a = tf.tensor2d(vec![3.0], vec![1, 1]);
    let b = tf.tensor2d(vec![5.0], vec![1, 1]);
    let axis = 0;
    let res = tf.concat(vec![a, b], axis);
    let res_tensor = tf.get_tensor_info(res).expect("tensor");
    let ans_data = TensorData::F32(vec![3.0, 5.0]);
    let ans_shape = vec![2, 1];
    assert_eq!(res_tensor.data(), &ans_data);
    assert_eq!(res_tensor.shape(), &ans_shape);

    let a = tf.tensor2d(vec![], vec![0, 5]);
    let b = tf.tensor2d(vec![], vec![0, 5]);
    let c = tf.tensor2d(vec![], vec![0, 5]);
    let axis = 0;
    let res = tf.concat(vec![a, b, c], axis);
    let res_tensor = tf.get_tensor_info(res).expect("tensor");
    let ans_data = TensorData::F32(vec![]);
    let ans_shape = vec![0, 5];
    assert_eq!(res_tensor.data(), &ans_data);
    assert_eq!(res_tensor.shape(), &ans_shape);
    let axis = 1;
    let res = tf.concat(vec![a, b, c], axis);
    let res_tensor = tf.get_tensor_info(res).expect("tensor");
    let ans_data = TensorData::F32(vec![]);
    let ans_shape = vec![0, 15];
    assert_eq!(res_tensor.data(), &ans_data);
    assert_eq!(res_tensor.shape(), &ans_shape);
}

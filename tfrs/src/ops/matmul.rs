use crate::core::*;

use std::ptr::null_mut;
use tfrs_sys::{
    pthreadpool, xnn_create_fully_connected_nc_f32, xnn_run_operator,
    xnn_setup_fully_connected_nc_f32, XNN_FLAG_TRANSPOSE_WEIGHTS,
};

impl TensorFlow {
    pub fn matmul(&mut self, a_id: TensorId, b_id: TensorId) -> TensorId {
        let a = self.get_tensor_info(a_id);
        let b = self.get_tensor_info(b_id);
        let (output, shape) = matmul(a, b, self.threadpool);
        self.register_tensor(output, shape)
    }
}

fn matmul(a: &Tensor, b: &Tensor, threadpool: *mut pthreadpool) -> (TensorData, Shape) {
    let a_shape = a.shape();
    let b_shape = b.shape();
    assert_eq!(a_shape.rank(), 2);
    assert_eq!(b_shape.rank(), 2);
    let left_dim = a_shape[0];
    let right_dim = b_shape[1];
    let shared_dim = a_shape[1];
    assert_eq!(shared_dim, b_shape[0]);

    let shape = vec![left_dim, right_dim];
    let output = TensorData::F32(vec![0.0; shape.tensor_size()]);
    let out_buf = output.buf_mut() as *mut f32;
    let a_buf = a.buf();
    let b_buf = b.buf();
    let input_channels = shared_dim as u64;
    let output_channels = right_dim as u64;
    let input_stride = input_channels;
    let output_stride = output_channels;
    let bias_buf = null_mut();
    let output_min = f32::NEG_INFINITY;
    let output_max = f32::INFINITY;
    let flags = XNN_FLAG_TRANSPOSE_WEIGHTS;
    let batch_size = left_dim as u64;
    let mut fully_connected_op = null_mut();

    unsafe {
        xnn_create_fully_connected_nc_f32(
            input_channels,
            output_channels,
            input_stride,
            output_stride,
            b_buf,
            bias_buf,
            output_min,
            output_max,
            flags,
            &mut fully_connected_op,
        );
        xnn_setup_fully_connected_nc_f32(
            fully_connected_op,
            batch_size,
            a_buf,
            out_buf,
            threadpool,
        );
        xnn_run_operator(fully_connected_op, threadpool);
    }
    (output, shape.clone())
}

#[test]
fn test_matmul() {
    let mut tf = TensorFlow::default();
    let x = tf.tensor2d(vec![2.0], vec![1, 1]);
    let m = tf.matmul(x, x);
    let mm = tf.get_tensor_info(m);
    assert_eq!(mm.data(), &TensorData::F32(vec![4.0]));
    assert_eq!(mm.shape(), &vec![1, 1]);

    let a = tf.tensor2d(vec![1.0, 2.0], vec![1, 2]);
    let b = tf.tensor2d(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
    let c = tf.matmul(a, b);
    let cc = tf.get_tensor_info(c);
    assert_eq!(cc.data(), &TensorData::F32(vec![7.0, 10.0]));
    assert_eq!(cc.shape(), &vec![1, 2]);
}

use crate::core::*;

use std::ptr::null_mut;
use tfrs_sys::{
    pthreadpool, xnn_create_fully_connected_nc_f32, xnn_run_operator,
    xnn_setup_fully_connected_nc_f32, XNN_FLAG_TRANSPOSE_WEIGHTS,
};

impl TensorFlow {
    pub fn matmul(&mut self, a_id: TensorId, b_id: TensorId) -> TensorId {
        let a = self.get_f32(a_id);
        let b = self.get_f32(b_id);
        let c = matmul(a, b, self.threadpool);
        self.register(c)
    }
}

fn matmul(a: &Tensor<f32>, b: &Tensor<f32>, threadpool: *mut pthreadpool) -> Box<Tensor<f32>> {
    let a_shape = a.shape();
    let b_shape = b.shape();
    assert_eq!(a_shape.rank(), 2);
    assert_eq!(b_shape.rank(), 2);
    let left_dim = a_shape[0];
    let right_dim = b_shape[1];
    let shared_dim = a_shape[1];
    assert_eq!(shared_dim, b_shape[0]);

    let shape = vec![left_dim, right_dim];
    let output = Tensor::new(vec![0.0; shape.tensor_size()], shape);
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
    output
}

#[test]
fn test_matmul() {
    let mut tf = TensorFlow::default();
    let x_id = tf.tensor2d(vec![2.0], vec![1, 1]);
    let y_id = tf.matmul(x_id, x_id);
    let z_id = tf.tensor2d(vec![4.0], vec![1, 1]);
    let y = tf.get_f32(y_id);
    let z = tf.get_f32(z_id);
    assert_eq!(y, z);

    let a_id = tf.tensor2d(vec![1.0, 2.0], vec![1, 2]);
    let b_id = tf.tensor2d(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
    let c_id = tf.matmul(a_id, b_id);
    let d_id = tf.tensor2d(vec![7.0, 10.0], vec![1, 2]);
    let c = tf.get_f32(c_id);
    let d = tf.get_f32(d_id);
    assert_eq!(c, d);
}

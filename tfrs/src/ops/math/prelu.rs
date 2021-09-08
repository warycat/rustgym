use crate::core::*;

use std::ptr::null_mut;
use tfrs_sys::{
    pthreadpool, xnn_create_prelu_nc_f32, xnn_operator_t, xnn_run_operator, xnn_setup_prelu_nc_f32,
};

impl TensorFlow {
    pub fn prelu(&mut self, x_id: TensorId, alpha_id: TensorId) -> TensorId {
        let x = self.get_f32(x_id);
        let alpha = self.get_f32(alpha_id);
        let y = prelu(x, alpha, self.threadpool);
        self.register(y)
    }
}

fn prelu(x: &Tensor<f32>, alpha: &Tensor<f32>, threadpool: *mut pthreadpool) -> Box<Tensor<f32>> {
    let shape = x.shape();
    let x_size = shape[0];
    let batch_size = x_size as u64;
    let x_buf = x.buf();
    let output = Tensor::new(vec![0.0; x_size], shape.clone());
    let out_buf = output.buf_mut() as *mut f32;
    let channels = 1;
    let input_stride = 1;
    let output_stride = 1;
    let negative_slop = alpha.buf() as *const f32;
    let flags = 0;
    let mut prelu_op: xnn_operator_t = null_mut();
    unsafe {
        xnn_create_prelu_nc_f32(
            channels,
            input_stride,
            output_stride,
            negative_slop,
            flags,
            &mut prelu_op,
        );
        xnn_setup_prelu_nc_f32(prelu_op, batch_size, x_buf, out_buf, threadpool);
        xnn_run_operator(prelu_op, threadpool);
    }
    output
}

#[test]
fn test_prelu() {
    let mut tf = TensorFlow::default();
    let x_id = tf.tensor1d(vec![-1.0, 2.0, -3.0, 4.0]);
    let alpha_id = tf.scalar(0.1);
    let y_id = tf.prelu(x_id, alpha_id);
    let z_id = tf.tensor1d(vec![-0.1, 2.0, -0.3, 4.0]);
    let y = tf.get_f32(y_id);
    let z = tf.get_f32(z_id);
    assert_eq!(y, z);
}

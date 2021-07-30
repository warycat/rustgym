use crate::core::*;

use std::ptr::null_mut;
use tfrs_sys::{
    pthreadpool, xnn_create_prelu_nc_f32, xnn_operator_t, xnn_run_operator, xnn_setup_prelu_nc_f32,
};

impl TenserFlow {
    pub fn prelu(&mut self, x_id: TensorId, alpha_id: TensorId) -> TensorId {
        let x = self.get_tensor_info(x_id).expect("tensor");
        let alpha = self.get_tensor_info(alpha_id).expect("tensor");
        let (output, shape) = prelu(x, alpha, self.threadpool);
        self.register_tensor(output, shape)
    }
}

fn prelu(x: &Tensor, alpha: &Tensor, threadpool: *mut pthreadpool) -> (TensorData, Shape) {
    let shape = x.shape();
    let x_size = shape[0];
    let batch_size = x_size as u64;
    let x_buf = x.buf();
    let output = TensorData::F32(vec![0.0; x_size]);
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
    (output, shape.clone())
}

#[test]
fn test_prelu() {
    let mut tf = TenserFlow::new(NUM_CORES);
    let x = tf.tensor1d(vec![-1.0, 2.0, -3.0, 4.0]);
    let alpha = tf.scalar(0.1);
    let res = tf.prelu(x, alpha);
    assert_eq!(
        tf.get_tensor_info(res).expect("tensor").data(),
        &TensorData::F32(vec![-0.1, 2.0, -0.3, 4.0])
    );
}

use crate::core::*;
use std::ptr::null_mut;
use tfrs_sys::{
    pthreadpool, xnn_create_softmax_nc_f32, xnn_run_operator, xnn_setup_softmax_nc_f32,
};

impl TensorFlow {
    pub fn softmax(&mut self, x_id: TensorId) -> TensorId {
        let x = self.get_f32(x_id);
        let y = softmax(x, self.threadpool);
        self.register(y)
    }
}

pub fn softmax(x: &Tensor<f32>, threadpool: *mut pthreadpool) -> Box<Tensor<f32>> {
    let x_shape = x.shape();
    let shape = x_shape.clone();
    let mut unary_op = null_mut();
    let size = shape.tensor_size();
    let channels = size;
    let input_stride = channels;
    let output_stride = channels;
    let flags = 0;
    let x_buf = x.buf() as *const f32;
    let output = Tensor::new(vec![0.0; size], shape);
    let out_buf = output.buf_mut() as *mut f32;
    let batch = 1;
    unsafe {
        xnn_create_softmax_nc_f32(channels, input_stride, output_stride, flags, &mut unary_op);
        xnn_setup_softmax_nc_f32(unary_op, batch, x_buf, out_buf, threadpool);
        xnn_run_operator(unary_op, threadpool);
    }
    output
}

#[test]
fn test() {
    let mut tf = TensorFlow::default();
    let x_id = tf.tensor1d(vec![2.0, 1.0, 3.0]);
    let y_id = tf.softmax(x_id);
    let z_id = tf.tensor1d(vec![0.24472848, 0.090030566, 0.6652409]);
    let sum_id = tf.sum(y_id);
    let one_id = tf.scalar(1.0);
    let y = tf.get_f32(y_id);
    let z = tf.get_f32(z_id);
    let sum = tf.get_f32(sum_id);
    let one = tf.get_f32(one_id);
    assert_eq!(y, z);
    assert_tensor_f32_eq!(sum, one);
}

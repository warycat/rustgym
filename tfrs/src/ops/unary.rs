use crate::core::*;
use std::ptr::null_mut;
use tfrs_sys::{
    pthreadpool, xnn_create_sigmoid_nc_f32, xnn_run_operator, xnn_setup_sigmoid_nc_f32,
};

#[macro_export]
macro_rules! unary_xnn_f32 {
    ($name:ident, $create_op:expr, $setup_op:expr) => {
        pub fn $name(x: &Tensor<f32>, threadpool: *mut pthreadpool) -> Box<Tensor<f32>> {
            let x_shape = x.shape();
            let shape = x_shape.clone();
            let mut unary_op = null_mut();
            let size = shape.tensor_size();
            let channels = 1;
            let input_stride = 1;
            let output_stride = 1;
            let flags = 0;
            let x_buf = x.buf() as *const f32;
            let output = Tensor::new(vec![0.0; size], shape);
            let out_buf = output.buf_mut() as *mut f32;
            let batch = size as u64;
            unsafe {
                $create_op(channels, input_stride, output_stride, flags, &mut unary_op);
                $setup_op(unary_op, batch, x_buf, out_buf, threadpool);
                xnn_run_operator(unary_op, threadpool);
            }
            output
        }
    };
}

unary_xnn_f32!(sigmoid, xnn_create_sigmoid_nc_f32, xnn_setup_sigmoid_nc_f32);

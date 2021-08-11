use crate::core::*;
use std::ptr::null_mut;
use tfrs_sys::{
    pthreadpool, size_t, xnn_create_add_nd_f32, xnn_run_operator, xnn_setup_add_nd_f32,
};

#[macro_export]
macro_rules! binary_xnn_f32 {
    ($name:ident, $create_op:expr, $setup_op:expr) => {
        pub fn $name(a: &Tensor, b: &Tensor, threadpool: *mut pthreadpool) -> (TensorData, Shape) {
            let a_shape = a.shape();
            let b_shape = b.shape();
            assert_eq!(a_shape, b_shape);
            let shape = a_shape.clone();
            let mut binary_op = null_mut();
            let output_min = f32::NEG_INFINITY;
            let output_max = f32::INFINITY;
            let flags = 0;
            let a_shape_len = a_shape.len() as u64;
            let a_shape_ptr = a_shape.buf() as *const size_t;
            let b_shape_len = b_shape.len() as u64;
            let b_shape_ptr = b_shape.buf() as *const size_t;
            let a_buf = a.buf() as *const f32;
            let b_buf = b.buf() as *const f32;
            let output = TensorData::F32(vec![0.0; shape.tensor_size()]);
            let out_buf = output.buf_mut() as *mut f32;
            unsafe {
                $create_op(output_min, output_max, flags, &mut binary_op);
                $setup_op(
                    binary_op,
                    a_shape_len,
                    a_shape_ptr,
                    b_shape_len,
                    b_shape_ptr,
                    a_buf,
                    b_buf,
                    out_buf,
                    threadpool,
                );
                xnn_run_operator(binary_op, threadpool);
            }
            (output, shape)
        }
    };
}

binary_xnn_f32!(add, xnn_create_add_nd_f32, xnn_setup_add_nd_f32);

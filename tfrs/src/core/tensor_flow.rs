use crate::core::*;

use std::collections::HashMap;
use std::ptr::null;
use tfrs_sys::{pthreadpool, pthreadpool_create, xnn_initialize};

#[derive(Debug)]
pub struct TenserFlow {
    last_id: TensorId,
    tensors: HashMap<TensorId, Tensor>,
    pub threadpool: *mut pthreadpool,
    xnn_operator_count: usize,
    prelu_op_cache: HashMap<f32, i32>,
}

impl TenserFlow {
    pub fn new(num_cores: u64) -> Self {
        let last_id = 0;
        let tensors = HashMap::new();
        let prelu_op_cache = HashMap::new();
        let threadpool = unsafe { pthreadpool_create(num_cores) };
        let xnn_operator_count = 0;
        unsafe { xnn_initialize(null()) };
        TenserFlow {
            last_id,
            tensors,
            threadpool,
            xnn_operator_count,
            prelu_op_cache,
        }
    }

    pub fn get_tensor_info(&self, tensor_id: TensorId) -> Option<&Tensor> {
        self.tensors.get(&tensor_id)
    }

    pub fn get_tensor_info_mut(&mut self, tensor_id: TensorId) -> Option<&mut Tensor> {
        self.tensors.get_mut(&tensor_id)
    }

    pub fn num_tensors(&self) -> usize {
        self.tensors.len()
    }

    pub fn register_tensor(&mut self, tensor_data: TensorData, shape: Vec<usize>) -> TensorId {
        self.last_id += 1;
        let tensor = Tensor::new(tensor_data, shape);
        self.tensors.insert(self.last_id, tensor);
        self.last_id
    }

    pub fn dispose_tensor(&mut self, tensor_id: TensorId) -> Option<Tensor> {
        self.tensors.remove(&tensor_id)
    }
}

#[test]
fn test_register_tensor() {
    let mut tf = TenserFlow::new(NUM_CORES);
    let tensor_data = TensorData::I32(vec![1, 2]);
    let size = tensor_data.size();
    assert_eq!(0, tf.num_tensors());
    let tensor_id = tf.register_tensor(tensor_data, vec![2]);
    assert_eq!(1, tf.num_tensors());
    let tensor = tf.get_tensor_info(1).expect("tensor");
    assert_eq!(size, tensor.size());
    tf.dispose_tensor(tensor_id);
    assert_eq!(0, tf.num_tensors());
}

#[test]
fn test_dispose_callback() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let tensor_0_callback_count: Rc<RefCell<i32>> = Rc::new(RefCell::new(0));
    {
        let mut tf = TenserFlow::new(NUM_CORES);
        assert_eq!(0, tf.num_tensors());
        let values_0 = TensorData::F32(vec![1.0, 2.0]);
        let tensor_id_0 = tf.register_tensor(values_0, vec![2]);
        let tensor_0_callback_count_clone = tensor_0_callback_count.clone();
        let callback = Box::new(move || {
            *tensor_0_callback_count_clone.borrow_mut() += 1;
        });
        tf.get_tensor_info_mut(tensor_id_0)
            .expect("tensor_0")
            .register_disposal_callback(callback.clone());
        tf.get_tensor_info_mut(tensor_id_0)
            .expect("tensor_0")
            .register_disposal_callback(callback);
    }
    let val = tensor_0_callback_count.borrow();
    assert_eq!(*val, 2);
}

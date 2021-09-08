use crate::core::*;

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ptr::null;
use tfrs_sys::{pthreadpool, pthreadpool_create, xnn_initialize};

#[derive(Debug, Copy, Clone)]
pub enum TensorId {
    F32(usize),
    I32(usize),
    Bool(usize),
}

impl TensorId {
    pub fn is_f32(&self) -> bool {
        matches!(self, TensorId::F32(_))
    }
    pub fn is_i32(&self) -> bool {
        matches!(self, TensorId::I32(_))
    }
    pub fn is_bool(&self) -> bool {
        matches!(self, TensorId::Bool(_))
    }
}

impl std::ops::Deref for TensorId {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        use TensorId::*;
        match self {
            F32(id) => id,
            I32(id) => id,
            Bool(id) => id,
        }
    }
}

#[derive(Debug)]
pub struct TensorFlow {
    last_id: usize,
    tensors: HashMap<usize, Box<dyn TensorLike>>,
    pub threadpool: *mut pthreadpool,
    xnn_operator_count: usize,
    prelu_op_cache: HashMap<f32, i32>,
}

impl TensorFlow {
    pub fn new(num_cores: u64) -> Self {
        let last_id = 0;
        let tensors = HashMap::new();
        let prelu_op_cache = HashMap::new();
        let threadpool = unsafe { pthreadpool_create(num_cores) };
        let xnn_operator_count = 0;
        unsafe { xnn_initialize(null()) };
        TensorFlow {
            last_id,
            tensors,
            threadpool,
            xnn_operator_count,
            prelu_op_cache,
        }
    }

    pub fn default() -> Self {
        TensorFlow::new(NUM_CORES)
    }

    pub fn get(&self, tensor_id: TensorId) -> &dyn TensorLike {
        self.tensors.get(&tensor_id).expect("tensor").as_ref()
    }

    pub fn get_f32(&self, tensor_id: TensorId) -> &Tensor<f32> {
        // assert!(tensor_id.is_f32(), "assertion failed:\n TensorId: `{:?}` is not f32", tensor_id);
        self.tensors.get(&tensor_id).expect("tensor").as_f32()
    }

    pub fn get_i32(&self, tensor_id: TensorId) -> &Tensor<i32> {
        assert!(
            tensor_id.is_i32(),
            "assertion failed:\n TensorId: `{:?}` is not i32",
            tensor_id
        );
        self.tensors.get(&tensor_id).expect("tensor").as_i32()
    }

    pub fn get_bool(&self, tensor_id: TensorId) -> &Tensor<bool> {
        assert!(
            tensor_id.is_bool(),
            "assertion failed:\n TensorId: `{:?}` is not bool",
            tensor_id
        );
        self.tensors.get(&tensor_id).expect("tensor").as_bool()
    }

    pub fn get_mut(&mut self, tensor_id: TensorId) -> &mut Box<dyn TensorLike> {
        self.tensors.get_mut(&tensor_id).expect("tensor")
    }

    pub fn register(&mut self, tensor: Box<dyn Any>) -> TensorId {
        let type_id = (&*tensor).type_id();
        if type_id == TypeId::of::<Tensor<f32>>() {
            self.register_f32(tensor.downcast::<Tensor<f32>>().unwrap())
        } else if type_id == TypeId::of::<Tensor<i32>>() {
            self.register_i32(tensor.downcast::<Tensor<i32>>().unwrap())
        } else if type_id == TypeId::of::<Tensor<bool>>() {
            self.register_bool(tensor.downcast::<Tensor<bool>>().unwrap())
        } else {
            panic!()
        }
    }

    pub fn register_f32(&mut self, tensor: Box<Tensor<f32>>) -> TensorId {
        self.last_id += 1;
        self.tensors.insert(self.last_id, tensor);
        TensorId::F32(self.last_id)
    }

    pub fn register_i32(&mut self, tensor: Box<Tensor<i32>>) -> TensorId {
        self.last_id += 1;
        self.tensors.insert(self.last_id, tensor);
        TensorId::I32(self.last_id)
    }

    pub fn register_bool(&mut self, tensor: Box<Tensor<bool>>) -> TensorId {
        self.last_id += 1;
        self.tensors.insert(self.last_id, tensor);
        TensorId::Bool(self.last_id)
    }

    pub fn dispose(&mut self, tensor_id: TensorId) -> Option<Box<dyn TensorLike>> {
        self.tensors.remove(&tensor_id)
    }

    pub fn num_tensors(&self) -> usize {
        self.tensors.len()
    }
}

#[test]
fn test_register_tensor() {
    let mut tf = TensorFlow::default();
    let tensor = Tensor::new(vec![1, 2], vec![2]);
    assert_eq!(0, tf.num_tensors());
    let tensor_id = tf.register(tensor);
    assert_eq!(1, tf.num_tensors());
    let tensor = tf.get(tensor_id);
    assert_eq!(2, tensor.size());
    tf.dispose(tensor_id);
    assert_eq!(0, tf.num_tensors());
}

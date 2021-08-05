use std::ffi::c_void;

#[derive(Debug, Clone)]
pub enum TensorData {
    F32(Vec<f32>),
    I32(Vec<i32>),
    Bool(Vec<bool>),
}

impl PartialEq for TensorData {
    fn eq(&self, other: &TensorData) -> bool {
        use TensorData::*;
        match (self, other) {
            (F32(a), F32(b)) => a == b,
            (_, _) => false,
        }
    }
}

impl TensorData {
    pub fn buf(&self) -> *const c_void {
        use TensorData::*;
        match self {
            F32(v) => v.as_ptr() as *const c_void,
            I32(v) => v.as_ptr() as *const c_void,
            Bool(v) => v.as_ptr() as *const c_void,
        }
    }

    pub fn buf_mut(&self) -> *mut c_void {
        use TensorData::*;
        match self {
            F32(v) => v.as_ptr() as *mut c_void,
            I32(v) => v.as_ptr() as *mut c_void,
            Bool(v) => v.as_ptr() as *mut c_void,
        }
    }

    pub fn get_i32(&self, i: usize) -> i32 {
        use TensorData::*;
        match self {
            I32(v) => v[i],
            _ => panic!(),
        }
    }

    pub fn get_f32(&self, i: usize) -> f32 {
        use TensorData::*;
        match self {
            F32(v) => v[i],
            _ => panic!(),
        }
    }

    pub fn get_bool(&self, i: usize) -> bool {
        use TensorData::*;
        match self {
            Bool(v) => v[i],
            _ => panic!(),
        }
    }
}

impl TensorData {
    pub fn size(&self) -> usize {
        match self {
            TensorData::F32(data) => data.len(),
            TensorData::I32(data) => data.len(),
            TensorData::Bool(data) => data.len(),
        }
    }
}

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
pub static mut VERSION: i32 = 0;

#[no_mangle]
extern "C" fn isBadVersion(version: i32) -> bool {
    unsafe { version >= VERSION }
}

#[repr(C)]
pub struct BinaryMatrix {
    get: fn(&BinaryMatrix, i32, i32) -> i32,
    dimensions: fn(&BinaryMatrix) -> *const i32,
    data: Vec<Vec<i32>>,
}

impl BinaryMatrix {
    pub fn new(data: Vec<Vec<i32>>) -> Self {
        fn dimensions(matrix: &BinaryMatrix) -> *const i32 {
            [matrix.data.len() as i32, matrix.data[0].len() as i32].as_ptr()
        }
        fn get(matrix: &BinaryMatrix, i: i32, j: i32) -> i32 {
            matrix.data[i as usize][j as usize]
        }

        BinaryMatrix {
            data,
            dimensions,
            get,
        }
    }
}

extern "C" {
    pub fn firstBadVersion(version: i32) -> i32;

    #[allow(improper_ctypes)]
    pub fn leftMostColumnWithOne(matrix: &BinaryMatrix) -> i32;

    fn csvg() -> *mut c_char;
}

pub fn svg() -> String {
    let c_buf: *const c_char = unsafe { csvg() };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();
    str_buf
}

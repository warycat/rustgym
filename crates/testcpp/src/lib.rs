extern "C" {
    pub fn firstBadVersion(version: i32) -> i32;
}

pub static mut VERSION: i32 = 0;

#[no_mangle]
extern "C" fn isBadVersion(version: i32) -> bool {
    unsafe { version >= VERSION }
}

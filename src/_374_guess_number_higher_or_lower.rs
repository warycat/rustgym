struct Solution;

static mut X: i32 = 0;

unsafe fn guess(num: i32) -> i32 {
    use std::cmp::Ordering::*;
    match X.cmp(&num) {
        Equal => 0,
        Greater => 1,
        Less => -1,
    }
}

impl Solution {
    #[allow(non_snake_case)]
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut l = 1;
        let mut r = n;
        while l < r {
            let m = l + (r - l) / 2;
            match guess(m) {
                0 => {
                    return m;
                }
                -1 => {
                    r = m;
                }
                1 => {
                    l = m + 1;
                }
                _ => {}
            }
        }
        l
    }
}

#[test]
fn test() {
    unsafe {
        X = 6;
        assert_eq!(Solution::guessNumber(10), 6);
    }
}

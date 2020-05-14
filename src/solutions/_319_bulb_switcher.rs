pub struct Solution;

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt() as i32
    }
}

#[test]
fn test() {
    let n = 3;
    let res = 1;
    assert_eq!(Solution::bulb_switch(n), res);
}

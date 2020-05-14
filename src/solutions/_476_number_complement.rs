pub struct Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut mask = !0;
        while mask & num != 0 {
            mask <<= 1;
        }
        !mask & !num
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_complement(5), 2);
    assert_eq!(Solution::find_complement(1), 0);
}

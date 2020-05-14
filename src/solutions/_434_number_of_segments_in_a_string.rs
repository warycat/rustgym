pub struct Solution;

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split_whitespace().count() as i32
    }
}

#[test]
fn test() {
    let s = "Hello, my name is John".to_string();
    assert_eq!(Solution::count_segments(s), 5);
}

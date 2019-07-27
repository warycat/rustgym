struct Solution;

impl Solution {
    fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(index) => index as i32,
            None => -1,
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
    assert_eq!(
        Solution::str_str("aaaaa".to_string(), "bba".to_string()),
        -1
    );
}

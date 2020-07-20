struct Solution;

impl Solution {
    fn rotate_string(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut c = "".to_string();
        c += &a;
        c += &a;
        c.contains(&b)
    }
}

#[test]
fn test() {
    let a = "abcde".to_string();
    let b = "cdeab".to_string();
    assert_eq!(Solution::rotate_string(a, b), true);
    let a = "abcde".to_string();
    let b = "abced".to_string();
    assert_eq!(Solution::rotate_string(a, b), false);
}

struct Solution;

impl Solution {
    fn check_string(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let mut t = s.clone();
        t.sort_unstable();
        s == t
    }
}

#[test]
fn test() {
    let s = "aaabbb".to_string();
    let res = true;
    assert_eq!(Solution::check_string(s), res);
    let s = "abab".to_string();
    let res = false;
    assert_eq!(Solution::check_string(s), res);
    let s = "bbb".to_string();
    let res = true;
    assert_eq!(Solution::check_string(s), res);
}

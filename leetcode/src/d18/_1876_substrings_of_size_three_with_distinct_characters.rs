struct Solution;

impl Solution {
    fn count_good_substrings(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut res = 0;
        for i in 2..n {
            if s[i] != s[i - 1] && s[i] != s[i - 2] && s[i - 1] != s[i - 2] {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "xyzzaz".to_string();
    let res = 1;
    assert_eq!(Solution::count_good_substrings(s), res);
    let s = "aababcabc".to_string();
    let res = 4;
    assert_eq!(Solution::count_good_substrings(s), res);
}

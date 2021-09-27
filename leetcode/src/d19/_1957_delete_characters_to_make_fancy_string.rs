struct Solution;

impl Solution {
    fn make_fancy_string(s: String) -> String {
        let mut count = 0;
        let mut prev = ' ';
        let mut res = "".to_string();
        for c in s.chars() {
            if c == prev {
                if count < 2 {
                    res.push(c);
                }
                count += 1;
            } else {
                res.push(c);
                prev = c;
                count = 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "leeetcode".to_string();
    let res = "leetcode".to_string();
    assert_eq!(Solution::make_fancy_string(s), res);
    let s = "aaabaaaa".to_string();
    let res = "aabaa".to_string();
    assert_eq!(Solution::make_fancy_string(s), res);
    let s = "aab".to_string();
    let res = "aab".to_string();
    assert_eq!(Solution::make_fancy_string(s), res);
}

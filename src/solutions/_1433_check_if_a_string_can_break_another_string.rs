struct Solution;

impl Solution {
    fn check_if_can_break(s1: String, s2: String) -> bool {
        let n = s1.len();
        let mut s1: Vec<char> = s1.chars().collect();
        let mut s2: Vec<char> = s2.chars().collect();
        s1.sort_unstable();
        s2.sort_unstable();
        let mut sum1 = 0;
        let mut sum2 = 0;
        for i in 0..n {
            if s1[i] <= s2[i] {
                sum1 += 1;
            }
            if s1[i] >= s2[i] {
                sum2 += 1;
            }
        }
        sum1 == n || sum2 == n
    }
}

#[test]
fn test() {
    let s1 = "abc".to_string();
    let s2 = "xya".to_string();
    let res = true;
    assert_eq!(Solution::check_if_can_break(s1, s2), res);
    let s1 = "abe".to_string();
    let s2 = "acd".to_string();
    let res = false;
    assert_eq!(Solution::check_if_can_break(s1, s2), res);
    let s1 = "leetcodee".to_string();
    let s2 = "interview".to_string();
    let res = true;
    assert_eq!(Solution::check_if_can_break(s1, s2), res);
}

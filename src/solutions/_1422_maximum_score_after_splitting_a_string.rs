struct Solution;

impl Solution {
    fn max_score(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut zeros = vec![0; n + 1];
        for i in 0..n {
            zeros[i + 1] = zeros[i] + if s[i] == '0' { 1 } else { 0 };
        }

        let mut ones = vec![0; n + 1];
        for i in (0..n).rev() {
            ones[i] = ones[i + 1] + if s[i] == '1' { 1 } else { 0 };
        }
        let mut res = 0;
        for i in 1..n {
            res = res.max(zeros[i] + ones[i]);
        }
        res
    }
}

#[test]
fn test() {
    let s = "011101".to_string();
    let res = 5;
    assert_eq!(Solution::max_score(s), res);
    let s = "00111".to_string();
    let res = 5;
    assert_eq!(Solution::max_score(s), res);
    let s = "1111".to_string();
    let res = 3;
    assert_eq!(Solution::max_score(s), res);
}

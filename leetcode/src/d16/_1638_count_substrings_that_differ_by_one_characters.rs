struct Solution;

impl Solution {
    fn count_substrings(s: String, t: String) -> i32 {
        let n = s.len();
        let m = t.len();
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                let k = (n - i).min(m - j);
                let mut diff = 0;
                for d in 0..k {
                    if s[i + d] != t[j + d] {
                        diff += 1;
                    }
                    if diff == 1 {
                        res += 1;
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "aba".to_string();
    let t = "baba".to_string();
    let res = 6;
    assert_eq!(Solution::count_substrings(s, t), res);
    let s = "ab".to_string();
    let t = "bb".to_string();
    let res = 3;
    assert_eq!(Solution::count_substrings(s, t), res);
    let s = "a".to_string();
    let t = "a".to_string();
    let res = 0;
    assert_eq!(Solution::count_substrings(s, t), res);
    let s = "abe".to_string();
    let t = "bbc".to_string();
    let res = 10;
    assert_eq!(Solution::count_substrings(s, t), res);
}

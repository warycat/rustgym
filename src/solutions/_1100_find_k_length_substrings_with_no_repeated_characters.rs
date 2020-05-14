struct Solution;
use std::collections::HashMap;

impl Solution {
    fn num_k_len_substr_no_repeats(s: String, k: i32) -> i32 {
        let k = k as usize;
        if s.len() < k {
            return 0;
        }
        let mut hm: HashMap<char, usize> = HashMap::new();
        let mut res = 0;
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        for i in 0..n {
            let ci = s[i];
            *hm.entry(ci).or_default() += 1;
            if i >= k {
                let j = i - k;
                let cj = s[j];
                *hm.entry(cj).or_default() -= 1;
            }
            if hm.values().filter(|&v| *v == 1).sum::<usize>() == k {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "havefunonleetcode".to_string();
    let k = 5;
    let res = 6;
    assert_eq!(Solution::num_k_len_substr_no_repeats(s, k), res);
    let s = "home".to_string();
    let k = 5;
    let res = 0;
    assert_eq!(Solution::num_k_len_substr_no_repeats(s, k), res);
}

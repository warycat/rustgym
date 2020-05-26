struct Solution;

use std::collections::HashMap;

impl Solution {
    fn max_freq(s: String, max_letters: i32, min_size: i32, _: i32) -> i32 {
        let n = s.len();
        let min_size = min_size as usize;
        let max_letters = max_letters as usize;
        let s: Vec<usize> = s.bytes().map(|b| (b - b'a') as usize).collect();
        let mut hm: HashMap<u32, usize> = HashMap::new();
        let mut count: Vec<usize> = vec![0; 26];
        let mut letters = 0;
        let mut hash = 0;
        let pow: u32 = 26u32.pow(min_size as u32);
        let mut res = 0;
        for i in (0..n).rev() {
            let first = s[i];
            if count[first] == 0 {
                letters += 1;
            }
            count[first] += 1;
            hash *= 26u32;
            hash += first as u32;
            if i + min_size < n {
                let last = s[i + min_size];
                if count[last] == 1 {
                    letters -= 1;
                }
                count[last] -= 1;
                hash -= last as u32 * pow;
            }
            if i + min_size <= n && letters <= max_letters {
                *hm.entry(hash).or_default() += 1;
                res = res.max(hm[&hash]);
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let s = "aababcaab".to_string();
    let max_letters = 2;
    let min_size = 3;
    let max_size = 4;
    let res = 2;
    assert_eq!(Solution::max_freq(s, max_letters, min_size, max_size), res);
    let s = "aaaa".to_string();
    let max_letters = 1;
    let min_size = 3;
    let max_size = 3;
    let res = 2;
    assert_eq!(Solution::max_freq(s, max_letters, min_size, max_size), res);
    let s = "aabcabcab".to_string();
    let max_letters = 2;
    let min_size = 2;
    let max_size = 3;
    let res = 3;
    assert_eq!(Solution::max_freq(s, max_letters, min_size, max_size), res);
    let s = "abcde".to_string();
    let max_letters = 2;
    let min_size = 3;
    let max_size = 3;
    let res = 0;
    assert_eq!(Solution::max_freq(s, max_letters, min_size, max_size), res);
}

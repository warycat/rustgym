struct Solution;

use std::collections::HashMap;

impl Solution {
    fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let mut memo: HashMap<(usize, usize, char, usize), usize> = HashMap::new();
        let n = s.len();
        let k = k as usize;
        let s: Vec<char> = s.chars().collect();

        Self::dp(0, k, ' ', 0, &mut memo, &s, n) as i32
    }

    fn compressed_length(len: usize) -> usize {
        if len <= 1 {
            len
        } else if len < 10 {
            2
        } else if len < 100 {
            3
        } else {
            4
        }
    }

    fn dp(
        start: usize,
        k: usize,
        c: char,
        m: usize,
        memo: &mut HashMap<(usize, usize, char, usize), usize>,
        s: &[char],
        n: usize,
    ) -> usize {
        if let Some(&res) = memo.get(&(start, k, c, m)) {
            res
        } else {
            let res = if start == n {
                Self::compressed_length(m)
            } else {
                let mut res = std::usize::MAX;
                if k > 0 {
                    res = res.min(Self::dp(start + 1, k - 1, c, m, memo, s, n));
                }
                if s[start] == c {
                    res = res.min(Self::dp(start + 1, k, c, m + 1, memo, s, n));
                } else {
                    res = res.min(
                        Self::compressed_length(m)
                            + Self::dp(start + 1, k, s[start], 1, memo, s, n),
                    );
                }
                res
            };
            memo.insert((start, k, c, m), res);
            res
        }
    }
}

#[test]
fn test() {
    let s = "aaabcccd".to_string();
    let k = 2;
    let res = 4;
    assert_eq!(Solution::get_length_of_optimal_compression(s, k), res);
    let s = "aabbaa".to_string();
    let k = 2;
    let res = 2;
    assert_eq!(Solution::get_length_of_optimal_compression(s, k), res);
    let s = "aaaaaaaaaaa".to_string();
    let k = 0;
    let res = 3;
    assert_eq!(Solution::get_length_of_optimal_compression(s, k), res);
}

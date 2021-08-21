struct Solution;
use std::collections::HashMap;

impl Solution {
    fn palindrome_partition(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let k = k as usize;
        let mut memo: HashMap<(usize, usize, usize), i32> = HashMap::new();
        Self::dp(0, n, k, &mut memo, &s)
    }

    fn dp(
        start: usize,
        end: usize,
        k: usize,
        memo: &mut HashMap<(usize, usize, usize), i32>,
        s: &[char],
    ) -> i32 {
        if k == end - start {
            return 0;
        }

        if let Some(&res) = memo.get(&(start, end, k)) {
            return res;
        }

        let res = if k == 1 {
            let n = end - start;
            s[start..end]
                .iter()
                .zip(s[start..end].iter().rev())
                .take(n / 2)
                .map(|v| if v.0 == v.1 { 0 } else { 1 })
                .sum()
        } else {
            let mut min = std::i32::MAX;
            for i in start + 1..=end - k + 1 {
                min = min.min(Self::dp(start, i, 1, memo, s) + Self::dp(i, end, k - 1, memo, s));
            }
            min
        };
        memo.insert((start, end, k), res);
        res
    }
}

#[test]
fn test() {
    let s = "abc".to_string();
    let k = 2;
    let res = 1;
    assert_eq!(Solution::palindrome_partition(s, k), res);
    let s = "aabbc".to_string();
    let k = 3;
    let res = 0;
    assert_eq!(Solution::palindrome_partition(s, k), res);
    let s = "leetcode".to_string();
    let k = 8;
    let res = 0;
    assert_eq!(Solution::palindrome_partition(s, k), res);
}

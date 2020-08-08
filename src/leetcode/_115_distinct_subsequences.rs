struct Solution;

use std::collections::HashMap;

impl Solution {
    fn num_distinct(s: String, t: String) -> i32 {
        let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let n = s.len();
        let m = t.len();
        Self::dp(0, 0, &mut memo, &s, &t, n, m)
    }

    fn dp(
        i: usize,
        j: usize,
        memo: &mut HashMap<(usize, usize), i32>,
        s: &[char],
        t: &[char],
        n: usize,
        m: usize,
    ) -> i32 {
        if let Some(&res) = memo.get(&(i, j)) {
            return res;
        }
        let res = if j == m {
            1
        } else {
            if i == n {
                0
            } else {
                if s[i] == t[j] {
                    Self::dp(i + 1, j + 1, memo, s, t, n, m) + Self::dp(i + 1, j, memo, s, t, n, m)
                } else {
                    Self::dp(i + 1, j, memo, s, t, n, m)
                }
            }
        };
        memo.insert((i, j), res);
        res
    }
}

#[test]
fn test() {
    let s = "rabbbit".to_string();
    let t = "rabbit".to_string();
    let res = 3;
    assert_eq!(Solution::num_distinct(s, t), res);
    let s = "babgbag".to_string();
    let t = "bag".to_string();
    let res = 5;
    assert_eq!(Solution::num_distinct(s, t), res);
}

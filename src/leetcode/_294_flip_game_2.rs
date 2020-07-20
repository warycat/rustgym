struct Solution;
use std::collections::HashMap;

impl Solution {
    fn can_win(s: String) -> bool {
        let mut s: Vec<char> = s.chars().collect();
        let n = s.len();
        if n == 0 {
            return false;
        }
        let mut memo: HashMap<Vec<char>, bool> = HashMap::new();
        Self::dp(&mut s, &mut memo, n)
    }
    fn dp(s: &mut Vec<char>, memo: &mut HashMap<Vec<char>, bool>, n: usize) -> bool {
        if let Some(&res) = memo.get(s) {
            return res;
        }
        let mut res = false;
        for i in 0..n - 1 {
            if res {
                break;
            }
            if s[i] == '+' && s[i + 1] == '+' {
                s[i] = '-';
                s[i + 1] = '-';
                res |= !Self::dp(s, memo, n);
                s[i] = '+';
                s[i + 1] = '+';
            }
        }
        memo.insert(s.to_vec(), res);
        res
    }
}

#[test]
fn test() {
    let s = "++++".to_string();
    let res = true;
    assert_eq!(Solution::can_win(s), res);
}

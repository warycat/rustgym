struct Solution;

use std::collections::HashMap;

impl Solution {
    fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        let s: Vec<u8> = s.bytes().collect();
        let mut res = 0;
        let mut memo: HashMap<(usize, usize, u8), i32> = HashMap::new();
        for i in 0..26 {
            res = res.max(Self::dp(0, n - 1, i as u8 + b'a', &mut memo, &s));
        }
        res
    }
    fn dp(
        left: usize,
        right: usize,
        c: u8,
        memo: &mut HashMap<(usize, usize, u8), i32>,
        s: &[u8],
    ) -> i32 {
        if left >= right {
            return 0;
        }
        if let Some(&res) = memo.get(&(left, right, c)) {
            res
        } else {
            let mut l = left;
            while l < right && s[l] != c {
                l += 1;
            }
            let mut r = right;
            while r > l && s[r] != c {
                r -= 1;
            }
            let res = if l == r {
                0
            } else {
                let mut res = 0;
                for i in 0..26 {
                    let d = b'a' + i as u8;
                    if d != c {
                        res = res.max(2 + Self::dp(l + 1, r - 1, d, memo, s));
                    }
                }
                res
            };
            memo.insert((left, right, c), res);
            res
        }
    }
}

#[test]
fn test() {
    let s = "bbabab".to_string();
    let res = 4;
    assert_eq!(Solution::longest_palindrome_subseq(s), res);
    let s = "dcbccacdb".to_string();
    let res = 4;
    assert_eq!(Solution::longest_palindrome_subseq(s), res);
    let s = "boob".to_string();
    let res = 4;
    assert_eq!(Solution::longest_palindrome_subseq(s), res);
}

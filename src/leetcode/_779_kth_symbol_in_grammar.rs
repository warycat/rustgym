struct Solution;

impl Solution {
    fn kth_grammar(n: i32, k: i32) -> i32 {
        let n = n as usize - 1;
        let k = k as usize - 1;
        Self::kth(n, k)
    }
    fn kth(n: usize, k: usize) -> i32 {
        if n == 0 {
            0
        } else {
            Self::kth(n - 1, k / 2) ^ (k % 2) as i32
        }
    }
}

#[test]
fn test() {
    let n = 1;
    let k = 1;
    let res = 0;
    assert_eq!(Solution::kth_grammar(n, k), res);
    let n = 2;
    let k = 1;
    let res = 0;
    assert_eq!(Solution::kth_grammar(n, k), res);
    let n = 2;
    let k = 2;
    let res = 1;
    assert_eq!(Solution::kth_grammar(n, k), res);
    let n = 4;
    let k = 5;
    let res = 1;
    assert_eq!(Solution::kth_grammar(n, k), res);
    let n = 30;
    let k = 417_219_134;
    let res = 1;
    assert_eq!(Solution::kth_grammar(n, k), res);
}

struct Solution;

impl Solution {
    fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let mut l = 1i64;
        let mut r = max_sum as i64;
        let n = n as usize;
        let index = index as usize;
        let mut res = 0;
        let kl = index as i64;
        let kr = (n - index - 1) as i64;
        while l <= r {
            let m = (l + (r - l) / 2) as i64;
            let sum = helper(m, kl) + m + helper(m, kr);
            if sum > max_sum as i64 {
                r = m - 1;
            } else {
                res = res.max(m);
                l = m + 1;
            }
        }
        res as i32
    }
}

fn helper(m: i64, k: i64) -> i64 {
    if m > k {
        (m - 1 + m - 1 - k + 1) * k / 2
    } else {
        (m - 1 + 1) * (m - 1) / 2 + (k - m + 1)
    }
}

#[test]
fn test() {
    let n = 4;
    let index = 2;
    let max_sum = 6;
    let res = 2;
    assert_eq!(Solution::max_value(n, index, max_sum), res);
    let n = 6;
    let index = 1;
    let max_sum = 10;
    let res = 3;
    assert_eq!(Solution::max_value(n, index, max_sum), res);
}

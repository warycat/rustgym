struct Solution;

impl Solution {
    fn kth_factor(n: i32, mut k: i32) -> i32 {
        for i in 1..=n {
            if n % i == 0 {
                k -= 1;
                if k == 0 {
                    return i;
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    let n = 12;
    let k = 3;
    let res = 3;
    assert_eq!(Solution::kth_factor(n, k), res);
    let n = 7;
    let k = 2;
    let res = 7;
    assert_eq!(Solution::kth_factor(n, k), res);
    let n = 4;
    let k = 4;
    let res = -1;
    assert_eq!(Solution::kth_factor(n, k), res);
    let n = 1;
    let k = 1;
    let res = 1;
    assert_eq!(Solution::kth_factor(n, k), res);
    let n = 1000;
    let k = 3;
    let res = 4;
    assert_eq!(Solution::kth_factor(n, k), res);
}

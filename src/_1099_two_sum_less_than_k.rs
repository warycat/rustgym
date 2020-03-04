struct Solution;

impl Solution {
    fn two_sum_less_than_k(mut a: Vec<i32>, k: i32) -> i32 {
        a.sort_unstable();
        let n = a.len();
        let mut l = 0;
        let mut r = n - 1;
        let mut res = -1;
        while l < r {
            let sum = a[l] + a[r];
            if sum < k {
                res = res.max(sum);
                l += 1;
            } else {
                r -= 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let a = vec![34, 23, 1, 24, 75, 33, 54, 8];
    let k = 60;
    let res = 58;
    assert_eq!(Solution::two_sum_less_than_k(a, k), res);
    let a = vec![10, 20, 30];
    let k = 15;
    let res = -1;
    assert_eq!(Solution::two_sum_less_than_k(a, k), res);
}

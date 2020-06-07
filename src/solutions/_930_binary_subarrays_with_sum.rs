struct Solution;

impl Solution {
    fn num_subarrays_with_sum(a: Vec<i32>, s: i32) -> i32 {
        let n = a.len();
        let mut count = vec![0; n + 1];
        count[0] = 1;
        let mut sum = 0;
        let mut res = 0;
        for i in 0..n {
            sum += a[i];
            if sum >= s {
                res += count[(sum - s) as usize];
            }
            count[sum as usize] += 1;
        }
        res as i32
    }
}

#[test]
fn test() {
    let a = vec![1, 0, 1, 0, 1];
    let s = 2;
    let res = 4;
    assert_eq!(Solution::num_subarrays_with_sum(a, s), res);
}

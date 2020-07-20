struct Solution;

impl Solution {
    fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
        let n = k as usize;
        let mut count: Vec<i32> = vec![0; n];
        let mut sum = 0;
        let mut res = 0;
        count[0] = 1;
        for x in a {
            sum = (sum + x % k + k) % k;
            res += count[sum as usize];
            count[sum as usize] += 1;
        }
        res
    }
}

#[test]
fn test() {
    let a = vec![4, 5, 0, -2, -3, 1];
    let k = 5;
    let res = 7;
    assert_eq!(Solution::subarrays_div_by_k(a, k), res);
}

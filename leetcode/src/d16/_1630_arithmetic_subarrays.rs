struct Solution;

impl Solution {
    fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let m = l.len();
        let mut res = vec![];
        for i in 0..m {
            let start = l[i] as usize;
            let end = r[i] as usize + 1;
            let mut arr = nums[start..end].to_vec();
            arr.sort_unstable();
            let diff = arr[1] - arr[0];
            res.push(arr.windows(2).all(|w| w[1] - w[0] == diff));
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![4, 6, 5, 9, 3, 7];
    let l = vec![0, 0, 2];
    let r = vec![2, 3, 5];
    let res = vec![true, false, true];
    assert_eq!(Solution::check_arithmetic_subarrays(nums, l, r), res);
    let nums = vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10];
    let l = vec![0, 1, 6, 4, 8, 7];
    let r = vec![4, 4, 9, 7, 9, 10];
    let res = vec![false, true, false, false, true, true];
    assert_eq!(Solution::check_arithmetic_subarrays(nums, l, r), res);
}

struct Solution;

impl Solution {
    fn min_difference(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 3 {
            return 0;
        }
        nums.sort_unstable();
        let mut res = std::i32::MAX;
        for i in 0..=3 {
            let min = nums[i..n - (3 - i)].iter().min().unwrap();
            let max = nums[i..n - (3 - i)].iter().max().unwrap();
            res = res.min(max - min);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![5, 3, 2, 4];
    let res = 0;
    assert_eq!(Solution::min_difference(nums), res);
    let nums = vec![1, 5, 0, 10, 14];
    let res = 1;
    assert_eq!(Solution::min_difference(nums), res);
    let nums = vec![6, 6, 0, 1, 1, 4, 6];
    let res = 2;
    assert_eq!(Solution::min_difference(nums), res);
    let nums = vec![1, 5, 6, 14, 15];
    let res = 1;
    assert_eq!(Solution::min_difference(nums), res);
}

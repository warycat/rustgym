struct Solution;

impl Solution {
    fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![];
        for i in 0..n {
            res.push(nums[i]);
            res.push(nums[i + n]);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![2, 5, 1, 3, 4, 7];
    let n = 3;
    let res = vec![2, 3, 5, 4, 1, 7];
    assert_eq!(Solution::shuffle(nums, n), res);
    let nums = vec![1, 2, 3, 4, 4, 3, 2, 1];
    let n = 4;
    let res = vec![1, 4, 2, 3, 3, 2, 4, 1];
    assert_eq!(Solution::shuffle(nums, n), res);
    let nums = vec![1, 1, 2, 2];
    let n = 2;
    let res = vec![1, 2, 1, 2];
    assert_eq!(Solution::shuffle(nums, n), res);
}

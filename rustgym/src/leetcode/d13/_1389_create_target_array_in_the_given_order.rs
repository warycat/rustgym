struct Solution;

impl Solution {
    fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let n = nums.len();
        for i in 0..n {
            let e = nums[i];
            let j = index[i] as usize;
            res.insert(j, e);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![0, 1, 2, 3, 4];
    let index = vec![0, 1, 2, 2, 1];
    let res = vec![0, 4, 1, 3, 2];
    assert_eq!(Solution::create_target_array(nums, index), res);
    let nums = vec![1, 2, 3, 4, 0];
    let index = vec![0, 1, 2, 3, 0];
    let res = vec![0, 1, 2, 3, 4];
    assert_eq!(Solution::create_target_array(nums, index), res);
    let nums = vec![1];
    let index = vec![0];
    let res = vec![1];
    assert_eq!(Solution::create_target_array(nums, index), res);
}

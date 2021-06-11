struct Solution;

impl Solution {
    fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let n = nums.len();
        let mut res = std::i32::MAX;
        for i in 0..n {
            if nums[i] == target {
                res = res.min((i as i32 - start).abs());
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4, 5];
    let target = 5;
    let start = 3;
    let res = 1;
    assert_eq!(Solution::get_min_distance(nums, target, start), res);
    let nums = vec![1];
    let target = 1;
    let start = 0;
    let res = 0;
    assert_eq!(Solution::get_min_distance(nums, target, start), res);
    let nums = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let target = 1;
    let start = 0;
    let res = 0;
    assert_eq!(Solution::get_min_distance(nums, target, start), res);
}

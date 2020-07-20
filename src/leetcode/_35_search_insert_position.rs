struct Solution;

impl Solution {
    fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(i) => i as i32,
            Err(i) => i as i32,
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    assert_eq!(Solution::search_insert(nums, target), 2);
    let nums = vec![1, 3, 5, 6];
    let target = 2;
    assert_eq!(Solution::search_insert(nums, target), 1);
    let nums = vec![1, 3, 5, 6];
    let target = 7;
    assert_eq!(Solution::search_insert(nums, target), 4);
    let nums = vec![1, 3, 5, 6];
    let target = 0;
    assert_eq!(Solution::search_insert(nums, target), 0);
}

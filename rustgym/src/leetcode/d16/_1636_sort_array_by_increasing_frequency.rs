struct Solution;
use std::cmp::Reverse;
use std::collections::HashMap;

impl Solution {
    fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut count: HashMap<i32, usize> = HashMap::new();
        for i in 0..n {
            *count.entry(nums[i]).or_default() += 1;
        }
        nums.sort_by_key(|&x| (count[&x], Reverse(x)));
        nums
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 2, 2, 2, 3];
    let res = vec![3, 1, 1, 2, 2, 2];
    assert_eq!(Solution::frequency_sort(nums), res);
    let nums = vec![2, 3, 1, 3, 2];
    let res = vec![1, 3, 3, 2, 2];
    assert_eq!(Solution::frequency_sort(nums), res);
    let nums = vec![-1, 1, -6, 4, 5, -6, 1, 4, 1];
    let res = vec![5, -1, 4, 4, -6, -6, 1, 1, 1];
    assert_eq!(Solution::frequency_sort(nums), res);
}

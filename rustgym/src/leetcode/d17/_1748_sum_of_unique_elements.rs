struct Solution;

use std::collections::HashMap;

impl Solution {
    fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut count: HashMap<i32, usize> = HashMap::new();
        for &x in &nums {
            *count.entry(x).or_default() += 1;
        }
        nums.into_iter()
            .map(|x| if count[&x] == 1 { x } else { 0 })
            .sum()
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 2];
    let res = 4;
    assert_eq!(Solution::sum_of_unique(nums), res);
    let nums = vec![1, 1, 1, 1, 1];
    let res = 0;
    assert_eq!(Solution::sum_of_unique(nums), res);
    let nums = vec![1, 2, 3, 4, 5];
    let res = 15;
    assert_eq!(Solution::sum_of_unique(nums), res);
}

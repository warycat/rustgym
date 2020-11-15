struct Solution;

use std::collections::HashMap;

impl Solution {
    fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let n = nums.len();
        let sum: i32 = nums.iter().sum();
        let y = sum - x;
        if y == 0 {
            return n as i32;
        }
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let mut prev = 0;
        hm.insert(0, 0);
        let mut max = 0;
        for i in 0..n {
            prev += nums[i];
            hm.insert(prev, i + 1);
            if let Some(j) = hm.get(&(prev - y)) {
                max = max.max(i + 1 - j);
            }
        }

        if max == 0 {
            -1
        } else {
            (n - max) as i32
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 4, 2, 3];
    let x = 5;
    let res = 2;
    assert_eq!(Solution::min_operations(nums, x), res);
    let nums = vec![5, 6, 7, 8, 9];
    let x = 4;
    let res = -1;
    assert_eq!(Solution::min_operations(nums, x), res);
    let nums = vec![3, 2, 20, 1, 1, 3];
    let x = 10;
    let res = 5;
    assert_eq!(Solution::min_operations(nums, x), res);
    let nums = vec![
        8828, 9581, 49, 9818, 9974, 9869, 9991, 10000, 10000, 10000, 9999, 9993, 9904, 8819, 1231,
        6309,
    ];
    let x = 134365;
    let res = 16;
    assert_eq!(Solution::min_operations(nums, x), res);
}

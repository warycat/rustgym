struct Solution;

use std::collections::HashMap;

impl Solution {
    fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut count: HashMap<i32, usize> = HashMap::new();
        let n = nums.len();
        let mut res = 0;
        for i in 0..n {
            for j in i + 1..n {
                let product = nums[i] * nums[j];
                let x = count.entry(product).or_default();
                res += *x;
                *x += 1;
            }
        }
        res as i32 * 8
    }
}

#[test]
fn test() {
    let nums = vec![2, 3, 4, 6];
    let res = 8;
    assert_eq!(Solution::tuple_same_product(nums), res);
    let nums = vec![1, 2, 4, 5, 10];
    let res = 16;
    assert_eq!(Solution::tuple_same_product(nums), res);
    let nums = vec![2, 3, 4, 6, 8, 12];
    let res = 40;
    assert_eq!(Solution::tuple_same_product(nums), res);
    let nums = vec![2, 3, 5, 7];
    let res = 0;
    assert_eq!(Solution::tuple_same_product(nums), res);
}

struct Solution;

use std::collections::HashSet;

impl Solution {
    fn distribute_candies(candies: Vec<i32>) -> i32 {
        let n = candies.len();
        let mut hs: HashSet<i32> = HashSet::new();
        for val in candies {
            hs.insert(val);
        }
        usize::min(n / 2, hs.len()) as i32
    }
}

#[test]
fn test() {
    let candies = vec![1, 1, 2, 2, 3, 3];
    assert_eq!(Solution::distribute_candies(candies), 3);
    let candies = vec![1, 1, 2, 3];
    assert_eq!(Solution::distribute_candies(candies), 2);
}

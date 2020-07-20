struct Solution;
use std::collections::HashMap;

impl Solution {
    fn is_possible(nums: Vec<i32>) -> bool {
        let mut left: HashMap<i32, usize> = HashMap::new();
        let mut end: HashMap<i32, usize> = HashMap::new();
        for &x in &nums {
            *left.entry(x).or_default() += 1;
        }

        for &x in &nums {
            if *left.entry(x).or_default() == 0 {
                continue;
            }
            if *end.entry(x - 1).or_default() > 0 {
                *left.entry(x).or_default() -= 1;
                *end.entry(x - 1).or_default() -= 1;
                *end.entry(x).or_default() += 1;
                continue;
            }
            if *left.entry(x + 1).or_default() > 0 && *left.entry(x + 2).or_default() > 0 {
                *left.entry(x).or_default() -= 1;
                *left.entry(x + 1).or_default() -= 1;
                *left.entry(x + 2).or_default() -= 1;
                *end.entry(x + 2).or_default() += 1;
                continue;
            }
            return false;
        }
        true
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 3, 4, 5];
    let res = true;
    assert_eq!(Solution::is_possible(nums), res);
    let nums = vec![1, 2, 3, 3, 4, 4, 5, 5];
    let res = true;
    assert_eq!(Solution::is_possible(nums), res);
    let nums = vec![1, 2, 3, 4, 4, 5];
    let res = false;
    assert_eq!(Solution::is_possible(nums), res);
}

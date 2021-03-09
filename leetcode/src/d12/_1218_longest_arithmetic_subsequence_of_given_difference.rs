struct Solution;
use std::collections::HashMap;

impl Solution {
    fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let mut res = 0;
        for x in arr {
            let prev = if let Some(&size) = hm.get(&(x - difference)) {
                size
            } else {
                0
            };
            let count = hm.entry(x).or_default();
            *count = (*count).max(prev + 1);
            res = res.max(*count);
        }
        res as i32
    }
}

#[test]
fn test() {
    let arr = vec![1, 2, 3, 4];
    let difference = 1;
    let res = 4;
    assert_eq!(Solution::longest_subsequence(arr, difference), res);
    let arr = vec![1, 3, 5, 7];
    let difference = 1;
    let res = 1;
    assert_eq!(Solution::longest_subsequence(arr, difference), res);
    let arr = vec![1, 5, 7, 8, 5, 3, 4, 2, 1];
    let difference = -2;
    let res = 4;
    assert_eq!(Solution::longest_subsequence(arr, difference), res);
}

struct Solution;

use std::collections::HashMap;

impl Solution {
    fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut res: usize = 0;
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let mut diff = 0;
        let n = nums.len();
        hm.entry(0).or_default();
        for i in 0..n {
            if nums[i] == 1 {
                diff += 1;
            } else {
                diff -= 1;
            }
            let j = *hm.entry(diff).or_insert(i + 1);
            res = usize::max(i + 1 - j, res);
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![0, 1];
    let res = 2;
    assert_eq!(Solution::find_max_length(nums), res);
    let nums = vec![0, 1, 0];
    let res = 2;
    assert_eq!(Solution::find_max_length(nums), res);
}

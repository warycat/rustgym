struct Solution;
use std::collections::HashMap;

impl Solution {
    fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut prefix = vec![0; n];
        let mut prev = 0;
        let mut hm: HashMap<i32, Vec<usize>> = HashMap::new();
        hm.entry(0).or_default().push(0);
        for i in 0..n {
            prefix[i] = nums[i] + prev;
            prev = prefix[i];
            hm.entry(prev).or_default().push(i + 1);
        }
        let mut res = 0;
        for i in 0..n {
            if let Some(v) = hm.get(&(prefix[i] - k)) {
                for &j in v {
                    if i + 1 > j {
                        res = res.max(i + 1 - j);
                    }
                }
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, -1, 5, -2, 3];
    let k = 3;
    let res = 4;
    assert_eq!(Solution::max_sub_array_len(nums, k), res);
    let nums = vec![-2, -1, 2, 1];
    let k = 1;
    let res = 2;
    assert_eq!(Solution::max_sub_array_len(nums, k), res);
}

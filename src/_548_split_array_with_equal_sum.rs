struct Solution;
use std::collections::HashSet;

impl Solution {
    fn split_array(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut prefix = vec![0; n];
        let mut prev = 0;
        if n < 7 {
            return false;
        }
        for i in 0..n {
            prev += nums[i];
            prefix[i] = prev;
        }
        for j in 3..n - 3 {
            let mut hs: HashSet<i32> = HashSet::new();
            for i in 1..j - 1 {
                let a = prefix[i - 1];
                let b = prefix[j - 1] - prefix[i];
                if a == b {
                    hs.insert(a);
                }
            }
            for k in j + 1..n - 1 {
                let c = prefix[k - 1] - prefix[j];
                let d = prefix[n - 1] - prefix[k];
                if c == d {
                    if hs.contains(&c) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 1, 2, 1, 2, 1];
    let res = true;
    assert_eq!(Solution::split_array(nums), res);
}

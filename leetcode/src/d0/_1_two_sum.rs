struct Solution;

use std::collections::HashMap;

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = hm.get(&(target - num)) {
                return vec![j, i as i32];
            } else {
                hm.insert(num, i as i32);
            }
        }
        vec![]
    }
}

// Alternate implementation
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut d: HashMap<i32, i32> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let c = target - num;
            match d.get(&c) {
                Some(&v) => {
                    return vec![v, i as i32];
                },
                None => d.insert(num, i as i32)
            };
        }
        vec![-1, -1]
    }
}

#[test]
fn test() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
}

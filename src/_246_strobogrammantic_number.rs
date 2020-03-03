struct Solution;

use std::collections::HashMap;

impl Solution {
    fn is_strobogrammatic(nums: String) -> bool {
        let map: HashMap<char, char> =
            vec![('0', '0'), ('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')]
                .into_iter()
                .collect();
        let nums: Vec<char> = nums.chars().collect();
        let n = nums.len();
        for i in 0..nums.len() {
            if let Some(&x) = map.get(&nums[n - 1 - i]) {
                if nums[i] != x {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_strobogrammatic("0".to_string()), true);
    assert_eq!(Solution::is_strobogrammatic("69".to_string()), true);
    assert_eq!(Solution::is_strobogrammatic("88".to_string()), true);
    assert_eq!(Solution::is_strobogrammatic("962".to_string()), false);
}

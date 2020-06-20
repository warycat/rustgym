struct Solution;

use std::collections::HashMap;

impl Solution {
    fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        let nums: Vec<i32> = (1..=max_choosable_integer).collect();
        let n = nums.len();
        if desired_total > nums.iter().sum::<i32>() {
            return false;
        }
        let mut memo: HashMap<u32, bool> = HashMap::new();
        Self::dp(desired_total, (1 << n) - 1, &mut memo, &nums, n)
    }

    fn dp(total: i32, bitset: u32, memo: &mut HashMap<u32, bool>, nums: &[i32], n: usize) -> bool {
        if let Some(&res) = memo.get(&bitset) {
            return res;
        }
        let mut res = false;
        for i in 0..n {
            if res {
                break;
            }
            if 1 << i & bitset > 0 {
                if total - nums[i] <= 0 {
                    res = true;
                }
                res |= !Self::dp(total - nums[i], bitset & !(1 << i), memo, nums, n);
            }
        }
        memo.insert(bitset, res);
        res
    }
}
#[test]
fn test() {
    let max_choosable_integer = 10;
    let desired_total = 11;
    let res = false;
    assert_eq!(
        Solution::can_i_win(max_choosable_integer, desired_total),
        res
    );
    let max_choosable_integer = 4;
    let desired_total = 6;
    let res = true;
    assert_eq!(
        Solution::can_i_win(max_choosable_integer, desired_total),
        res
    );
    let max_choosable_integer = 5;
    let desired_total = 50;
    let res = false;
    assert_eq!(
        Solution::can_i_win(max_choosable_integer, desired_total),
        res
    );
    let max_choosable_integer = 18;
    let desired_total = 188;
    let res = false;
    assert_eq!(
        Solution::can_i_win(max_choosable_integer, desired_total),
        res
    );
}

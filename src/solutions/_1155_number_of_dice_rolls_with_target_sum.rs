struct Solution;

use std::collections::HashMap;

impl Solution {
    fn num_rolls_to_target(d: i32, f: i32, target: i32) -> i32 {
        let mut memo: HashMap<(i32, i32), i32> = HashMap::new();
        Self::dp(d, f, target, &mut memo)
    }

    fn dp(d: i32, f: i32, target: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
        if let Some(&val) = memo.get(&(d, target)) {
            return val;
        }
        let res = if d == 0 {
            if target == 0 {
                1
            } else {
                0
            }
        } else {
            let mut sum = 0;
            for i in 1..=f {
                sum += Self::dp(d - 1, f, target - i, memo);
                sum %= 1_000_000_007;
            }
            sum
        };
        memo.insert((d, target), res);
        res
    }
}

#[test]
fn test() {
    let d = 1;
    let f = 6;
    let target = 3;
    let res = 1;
    assert_eq!(Solution::num_rolls_to_target(d, f, target), res);
    let d = 2;
    let f = 6;
    let target = 7;
    let res = 6;
    assert_eq!(Solution::num_rolls_to_target(d, f, target), res);
    let d = 2;
    let f = 5;
    let target = 10;
    let res = 1;
    assert_eq!(Solution::num_rolls_to_target(d, f, target), res);
    let d = 1;
    let f = 2;
    let target = 3;
    let res = 0;
    assert_eq!(Solution::num_rolls_to_target(d, f, target), res);
    let d = 30;
    let f = 30;
    let target = 500;
    let res = 222_616_187;
    assert_eq!(Solution::num_rolls_to_target(d, f, target), res);
}

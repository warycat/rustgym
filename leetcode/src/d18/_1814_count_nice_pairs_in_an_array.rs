struct Solution;

use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        let mut res: i64 = 0;
        for x in nums {
            let diff = x - rev(x);
            let count = hm.entry(diff).or_default();
            res += *count as i64;
            *count += 1;
        }
        (res % MOD) as i32
    }
}

fn rev(mut x: i32) -> i32 {
    let mut digits: Vec<i32> = vec![];
    while x > 0 {
        digits.push(x % 10);
        x /= 10;
    }
    let mut res = 0;
    for x in digits {
        res *= 10;
        res += x;
    }
    res
}

#[test]
fn test() {
    let nums = vec![42, 11, 1, 97];
    let res = 2;
    assert_eq!(Solution::count_nice_pairs(nums), res);
    let nums = vec![13, 10, 35, 24, 76];
    let res = 4;
    assert_eq!(Solution::count_nice_pairs(nums), res);
}

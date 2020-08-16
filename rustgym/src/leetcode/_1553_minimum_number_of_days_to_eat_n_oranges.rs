struct Solution;

use std::collections::HashMap;

impl Solution {
    fn min_days(n: i32) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        Self::dp(n, &mut memo)
    }

    fn dp(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if n <= 1 {
            n
        } else {
            if let Some(&res) = memo.get(&n) {
                return res;
            }
            let res = 1 + (n % 2 + Self::dp(n / 2, memo)).min(n % 3 + Self::dp(n / 3, memo));
            memo.insert(n, res);
            res
        }
    }
}

#[test]
fn test() {
    let n = 10;
    let res = 4;
    assert_eq!(Solution::min_days(n), res);
    let n = 6;
    let res = 3;
    assert_eq!(Solution::min_days(n), res);
    let n = 1;
    let res = 1;
    assert_eq!(Solution::min_days(n), res);
    let n = 56;
    let res = 6;
    assert_eq!(Solution::min_days(n), res);
    let n = 9209408;
    let res = 23;
    assert_eq!(Solution::min_days(n), res);
    let n = 84806671;
    let res = 32;
    assert_eq!(Solution::min_days(n), res);
}

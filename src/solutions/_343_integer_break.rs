struct Solution;
use std::collections::HashMap;

impl Solution {
    fn integer_break(n: i32) -> i32 {
        let mut memo: HashMap<(i32, bool), i32> = HashMap::new();
        Self::dp(n, true, &mut memo)
    }

    fn dp(n: i32, split: bool, memo: &mut HashMap<(i32, bool), i32>) -> i32 {
        if let Some(&res) = memo.get(&(n, split)) {
            return res;
        }
        let mut res = if split { 0 } else { n };
        for i in 1..n {
            res = res.max(i * Self::dp(n - i, false, memo));
        }
        memo.insert((n, split), res);
        res
    }
}

#[test]
fn test() {
    let n = 2;
    let res = 1;
    assert_eq!(Solution::integer_break(n), res);
    let n = 10;
    let res = 36;
    assert_eq!(Solution::integer_break(n), res);
}

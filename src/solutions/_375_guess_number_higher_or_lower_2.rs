struct Solution;
use std::collections::HashMap;

impl Solution {
    fn get_money_amount(n: i32) -> i32 {
        let mut memo: HashMap<(i32, i32), i32> = HashMap::new();
        Self::dp(1, n, &mut memo)
    }

    fn dp(left: i32, right: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
        if left == right {
            0
        } else {
            if let Some(&res) = memo.get(&(left, right)) {
                return res;
            }
            let mut res = std::i32::MAX;
            for i in left..right {
                let a = if i != left {
                    Self::dp(left, i - 1, memo)
                } else {
                    0
                };
                let b = if i != right {
                    Self::dp(i + 1, right, memo)
                } else {
                    0
                };
                res = res.min(i + a.max(b));
            }
            memo.insert((left, right), res);
            res
        }
    }
}

#[test]
fn test() {
    let n = 10;
    let res = 16;
    assert_eq!(Solution::get_money_amount(n), res);
}

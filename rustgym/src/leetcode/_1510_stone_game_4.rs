struct Solution;

use std::collections::HashMap;

impl Solution {
    fn winner_square_game(n: i32) -> bool {
        let mut memo: HashMap<i32, bool> = HashMap::new();
        Self::dp(n, &mut memo)
    }

    fn dp(n: i32, memo: &mut HashMap<i32, bool>) -> bool {
        if let Some(&res) = memo.get(&n) {
            return res;
        }
        let x = Self::sqrt(n);
        let res = if x * x == n {
            true
        } else {
            let mut res = false;
            for i in 1..=x {
                let y = n - i * i;
                if !Self::dp(y, memo) {
                    res = true;
                    break;
                }
            }
            res
        };
        memo.insert(n, res);
        res
    }

    fn sqrt(x: i32) -> i32 {
        (x as f64).sqrt() as i32
    }
}

#[test]
fn test() {
    let n = 1;
    let res = true;
    assert_eq!(Solution::winner_square_game(n), res);
    let n = 2;
    let res = false;
    assert_eq!(Solution::winner_square_game(n), res);
    let n = 4;
    let res = true;
    assert_eq!(Solution::winner_square_game(n), res);
    let n = 7;
    let res = false;
    assert_eq!(Solution::winner_square_game(n), res);
    let n = 17;
    let res = false;
    assert_eq!(Solution::winner_square_game(n), res);
}

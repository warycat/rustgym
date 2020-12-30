struct Solution;
use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn number_of_ways(num_people: i32) -> i32 {
        let mut memo: HashMap<i32, i64> = HashMap::new();
        memo.insert(2, 2);
        memo.insert(1, 1);
        memo.insert(0, 1);
        Self::dp(num_people / 2, &mut memo) as i32
    }

    fn dp(n: i32, memo: &mut HashMap<i32, i64>) -> i64 {
        if let Some(&res) = memo.get(&n) {
            return res;
        }
        let mut res = 0;
        for i in 0..n {
            res += Self::dp(i, memo) * Self::dp(n - 1 - i, memo);
            res %= MOD;
        }
        memo.insert(n, res);
        res
    }
}

#[test]
fn test() {
    let num_people = 6;
    let res = 5;
    assert_eq!(Solution::number_of_ways(num_people), res);
    let num_people = 140;
    let res = 685542858;
    assert_eq!(Solution::number_of_ways(num_people), res);
}

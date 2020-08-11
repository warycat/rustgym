struct Solution;
use std::cmp::Ordering::*;
use std::collections::HashMap;

impl Solution {
    fn stone_game_iii(values: Vec<i32>) -> String {
        let n = values.len();
        let mut memo: HashMap<usize, i32> = HashMap::new();
        match Self::dp(0, &mut memo, &values, n).cmp(&0) {
            Equal => "Tie".to_string(),
            Greater => "Alice".to_string(),
            Less => "Bob".to_string(),
        }
    }

    fn dp(start: usize, memo: &mut HashMap<usize, i32>, values: &[i32], n: usize) -> i32 {
        if let Some(&res) = memo.get(&start) {
            return res;
        }
        let res = if start == n {
            0
        } else {
            let mut sum = 0;
            let mut max = std::i32::MIN;
            for i in start..(start + 3).min(n) {
                sum += values[i];
                max = max.max(sum - Self::dp(i + 1, memo, values, n));
            }
            max
        };
        memo.insert(start, res);
        res
    }
}

#[test]
fn test() {
    let values = vec![1, 2, 3, 7];
    let res = "Bob".to_string();
    assert_eq!(Solution::stone_game_iii(values), res);
    let values = vec![1, 2, 3, 6];
    let res = "Tie".to_string();
    assert_eq!(Solution::stone_game_iii(values), res);
    let values = vec![1, 2, 3, -1, -2, -3, 7];
    let res = "Alice".to_string();
    assert_eq!(Solution::stone_game_iii(values), res);
    let values = vec![-1, -2, -3];
    let res = "Tie".to_string();
    assert_eq!(Solution::stone_game_iii(values), res);
}

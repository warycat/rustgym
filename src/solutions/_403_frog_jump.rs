struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    fn can_cross(stones: Vec<i32>) -> bool {
        let n = stones.len();
        let end = stones[n - 1];
        let mut memo: HashMap<(i32, i32), bool> = HashMap::new();
        let stones: HashSet<i32> = HashSet::from_iter(stones);
        Self::dp(0, 1, &mut memo, &stones, end)
    }

    fn dp(
        start: i32,
        k: i32,
        memo: &mut HashMap<(i32, i32), bool>,
        stones: &HashSet<i32>,
        end: i32,
    ) -> bool {
        if let Some(&res) = memo.get(&(start, k)) {
            return res;
        }
        let res = if start + k == end {
            true
        } else {
            if stones.contains(&(start + k)) {
                if k - 1 > 0 {
                    Self::dp(start + k, k + 1, memo, stones, end)
                        || Self::dp(start + k, k, memo, stones, end)
                        || Self::dp(start + k, k - 1, memo, stones, end)
                } else {
                    Self::dp(start + k, k + 1, memo, stones, end)
                        || Self::dp(start + k, k, memo, stones, end)
                }
            } else {
                false
            }
        };
        memo.insert((start, k), res);
        res
    }
}

#[test]
fn test() {
    let stones = vec![0, 1, 3, 5, 6, 8, 12, 17];
    let res = true;
    assert_eq!(Solution::can_cross(stones), res);
    let stones = vec![0, 1, 2, 3, 4, 8, 9, 11];
    let res = false;
    assert_eq!(Solution::can_cross(stones), res);
}

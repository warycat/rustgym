struct Solution;

use std::collections::HashMap;

impl Solution {
    fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
        let n = stone_value.len();
        Self::dp(0, n, &mut memo, &stone_value)
    }
    fn dp(
        start: usize,
        end: usize,
        memo: &mut HashMap<(usize, usize), i32>,
        stones: &[i32],
    ) -> i32 {
        if end - start < 2 {
            return 0;
        }
        if let Some(&res) = memo.get(&(start, end)) {
            return res;
        }
        let mut left = 0;
        let mut right = stones[start..end].iter().sum::<i32>();
        let mut res = 0;
        for i in start..end {
            left += stones[i];
            right -= stones[i];
            if left > right {
                res = res.max(right + Self::dp(i + 1, end, memo, stones));
                continue;
            }
            if left < right {
                res = res.max(left + Self::dp(start, i + 1, memo, stones));
                continue;
            }
            res = res.max(left + Self::dp(start, i + 1, memo, stones));
            res = res.max(right + Self::dp(i + 1, end, memo, stones));
        }
        memo.insert((start, end), res);
        res
    }
}

#[test]
fn test() {
    let stone_value = vec![6, 2, 3, 4, 5, 5];
    let res = 18;
    assert_eq!(Solution::stone_game_v(stone_value), res);
}

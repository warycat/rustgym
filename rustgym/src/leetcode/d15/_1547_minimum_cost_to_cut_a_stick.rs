struct Solution;

use std::collections::HashMap;

impl Solution {
    fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
        cuts.sort_unstable();
        let mut memo: HashMap<(i32, i32), i32> = HashMap::new();
        Self::dp(0, n, &mut memo, &cuts)
    }

    fn dp(left: i32, right: i32, memo: &mut HashMap<(i32, i32), i32>, cuts: &[i32]) -> i32 {
        if let Some(&res) = memo.get(&(left, right)) {
            return res;
        }
        let mut cuts_inside = vec![];
        for &x in cuts {
            if x > left && x < right {
                cuts_inside.push(x);
            }
        }
        let res = if !cuts_inside.is_empty() {
            let mut min = std::i32::MAX;
            for x in cuts_inside {
                min = min.min(Self::dp(left, x, memo, cuts) + Self::dp(x, right, memo, cuts));
            }
            min + right - left
        } else {
            0
        };
        memo.insert((left, right), res);
        res
    }
}

#[test]
fn test() {
    let n = 7;
    let cuts = vec![1, 3, 4, 5];
    let res = 16;
    assert_eq!(Solution::min_cost(n, cuts), res);
    let n = 9;
    let cuts = vec![5, 6, 1, 4, 2];
    let res = 22;
    assert_eq!(Solution::min_cost(n, cuts), res);
}

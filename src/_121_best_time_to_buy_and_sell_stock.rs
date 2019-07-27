struct Solution;

use std::i32;

impl Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len() as usize;
        if n < 2 {
            return 0;
        }
        let mut mins = vec![i32::MAX; n];
        mins[0] = prices[0];
        for i in 1..n {
            mins[i] = i32::min(mins[i - 1], prices[i]);
        }
        let mut max = 0;
        for i in 1..n {
            max = i32::max(prices[i] - mins[i - 1], max);
        }
        max
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}

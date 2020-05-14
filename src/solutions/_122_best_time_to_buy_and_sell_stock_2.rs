struct Solution;

impl Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                max += prices[i] - prices[i - 1]
            }
        }
        max
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}

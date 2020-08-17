struct Solution;

impl Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
        let mut cost = std::i32::MAX;
        let mut profit = 0;
        for price in prices {
            profit = profit.max(price - cost);
            cost = cost.min(price);
        }
        profit
    }
}

#[test]
fn test() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let res = 5;
    assert_eq!(Solution::max_profit(prices), res);
    let prices = vec![7, 6, 4, 3, 1];
    let res = 0;
    assert_eq!(Solution::max_profit(prices), res);
}

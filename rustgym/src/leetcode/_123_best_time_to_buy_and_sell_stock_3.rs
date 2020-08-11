struct Solution;

impl Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
        let mut t1_cost = std::i32::MAX;
        let mut t2_cost = std::i32::MAX;
        let mut t1_profit = 0;
        let mut t2_profit = 0;
        for x in prices {
            t1_cost = t1_cost.min(x);
            t1_profit = t1_profit.max(x - t1_cost);
            t2_cost = t2_cost.min(x - t1_profit);
            t2_profit = t2_profit.max(x - t2_cost);
        }
        t2_profit
    }
}

#[test]
fn test() {
    let prices = vec![3, 3, 5, 0, 0, 3, 1, 4];
    let res = 6;
    assert_eq!(Solution::max_profit(prices), res);
    let prices = vec![1, 2, 3, 4, 5];
    let res = 4;
    assert_eq!(Solution::max_profit(prices), res);
    let prices = vec![7, 6, 4, 3, 1];
    let res = 0;
    assert_eq!(Solution::max_profit(prices), res);
}

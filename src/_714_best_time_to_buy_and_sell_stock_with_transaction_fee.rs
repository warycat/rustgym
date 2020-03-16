struct Solution;

impl Solution {
    fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let n = prices.len();
        let mut cash = 0;
        let mut hold = -prices[0];
        for i in 1..n {
            cash = cash.max(hold + prices[i] - fee);
            hold = hold.max(cash - prices[i]);
        }
        cash
    }
}

#[test]
fn test() {
    let prices = vec![1, 3, 2, 8, 4, 9];
    let fee = 2;
    let res = 8;
    assert_eq!(Solution::max_profit(prices, fee), res);
}

struct Solution;

impl Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n < 2 {
            return 0;
        }
        let mut mins = vec![0; n];
        let mut prev_min = std::i32::MAX;
        for i in 0..n {
            prev_min = prev_min.min(prices[i]);
            mins[i] = prev_min;
        }
        let mut res = 0;
        for i in 1..n {
            res = res.max(prices[i] - mins[i - 1]);
        }
        res
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

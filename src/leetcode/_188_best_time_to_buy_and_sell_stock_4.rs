struct Solution;

impl Solution {
    fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut k = k as usize;
        k = k.min(n / 2);
        if k == 0 {
            return 0;
        }
        let mut min_costs = vec![std::i32::MAX; k];
        let mut max_profits = vec![0; k];
        for price in prices {
            min_costs[0] = min_costs[0].min(price);
            max_profits[0] = max_profits[0].max(price - min_costs[0]);
            for i in 1..k {
                min_costs[i] = min_costs[i].min(price - max_profits[i - 1]);
                max_profits[i] = max_profits[i].max(price - min_costs[i]);
            }
        }
        max_profits[k - 1] as i32
    }
}

#[test]
fn test() {
    let prices = vec![2, 4, 1];
    let k = 2;
    let res = 2;
    assert_eq!(Solution::max_profit(k, prices), res);
    let prices = vec![3, 2, 6, 5, 0, 3];
    let k = 2;
    let res = 7;
    assert_eq!(Solution::max_profit(k, prices), res);
}

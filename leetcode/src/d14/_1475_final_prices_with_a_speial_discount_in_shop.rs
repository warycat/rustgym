struct Solution;

impl Solution {
    fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let n = prices.len();
        let mut stack = vec![];
        let mut res = prices.to_vec();
        for i in 0..n {
            while let Some(&j) = stack.last() {
                if prices[i] > prices[j] {
                    break;
                } else {
                    stack.pop();
                    res[j] = prices[j] - prices[i];
                }
            }
            stack.push(i);
        }
        res
    }
}

#[test]
fn test() {
    let prices = vec![8, 4, 6, 2, 3];
    let res = vec![4, 2, 4, 2, 3];
    assert_eq!(Solution::final_prices(prices), res);
    let prices = vec![1, 2, 3, 4, 5];
    let res = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution::final_prices(prices), res);
    let prices = vec![10, 1, 1, 6];
    let res = vec![9, 0, 1, 6];
    assert_eq!(Solution::final_prices(prices), res);
}

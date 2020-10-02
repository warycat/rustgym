struct Solution;

impl Solution {
    fn min_operations_max_profit(
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        let mut wait = 0;
        let mut profit = 0;
        let mut max_profit = 0;
        let mut res = -1;
        let mut run = 0;
        while run < customers.len() || wait > 0 {
            if run < customers.len() {
                wait += customers[run];
            }
            profit += wait.min(4) * boarding_cost - running_cost;
            wait -= wait.min(4);
            if max_profit < profit {
                max_profit = profit;
                res = run as i32 + 1;
            }
            run += 1;
        }
        res
    }
}

#[test]
fn test() {
    let customers = vec![8, 3];
    let boarding_cost = 5;
    let running_cost = 6;
    let res = 3;
    assert_eq!(
        Solution::min_operations_max_profit(customers, boarding_cost, running_cost),
        res
    );
}

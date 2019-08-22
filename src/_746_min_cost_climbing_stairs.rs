struct Solution;

impl Solution {
    fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        let n = cost.len();
        cost.push(0);
        if n < 2 {
            return 0;
        }
        for i in 2..=n {
            cost[i] += i32::min(cost[i - 1], cost[i - 2]);
        }
        cost[n]
    }
}

#[test]
fn test() {
    let cost = vec![10, 15, 20];
    assert_eq!(Solution::min_cost_climbing_stairs(cost), 15);
    let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    assert_eq!(Solution::min_cost_climbing_stairs(cost), 6);
}

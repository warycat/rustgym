struct Solution;

impl Solution {
    fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable();
        let mut i = 0;
        let mut res = 0;
        while let Some(x) = cost.pop() {
            i += 1;
            i %= 3;
            if i != 0 {
                res += x;
            }
        }
        res
    }
}

#[test]
fn test() {
    let cost = vec![1, 2, 3];
    let res = 5;
    assert_eq!(Solution::minimum_cost(cost), res);
    let cost = vec![6, 5, 7, 9, 2, 2];
    let res = 23;
    assert_eq!(Solution::minimum_cost(cost), res);
    let cost = vec![5, 5];
    let res = 10;
    assert_eq!(Solution::minimum_cost(cost), res);
}

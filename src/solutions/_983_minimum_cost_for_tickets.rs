struct Solution;

impl Solution {
    fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let n = days.len();
        let mut dp: Vec<i32> = vec![];
        let pass = vec![1, 7, 30];
        for i in 0..n {
            let mut mins = costs.clone();
            for k in 0..3 {
                for j in (0..i).rev() {
                    if days[i] - days[j] >= pass[k] {
                        mins[k] += dp[j];
                        break;
                    }
                }
            }
            dp.push(*mins.iter().min().unwrap());
        }
        dp[n - 1]
    }
}

#[test]
fn test() {
    let days = vec![1, 4, 6, 7, 8, 20];
    let costs = vec![2, 7, 15];
    let res = 11;
    assert_eq!(Solution::mincost_tickets(days, costs), res);
    let days = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31];
    let costs = vec![2, 7, 15];
    let res = 17;
    assert_eq!(Solution::mincost_tickets(days, costs), res);
}

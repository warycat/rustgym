struct Solution;

impl Solution {
    fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .into_iter()
            .map(|v| v.into_iter().sum())
            .max()
            .unwrap()
    }
}

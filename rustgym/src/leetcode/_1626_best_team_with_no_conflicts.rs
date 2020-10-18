struct Solution;

impl Solution {
    fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let n = scores.len();
        let mut players = vec![];
        for i in 0..n {
            players.push((ages[i], scores[i]));
        }
        players.sort_unstable();
        let mut dp = vec![0; n];
        for i in 0..n {
            dp[i] = players[i].1;
            for j in 0..i {
                if players[j].1 <= players[i].1 {
                    dp[i] = dp[i].max(players[i].1 + dp[j]);
                }
            }
        }
        dp.into_iter().max().unwrap()
    }
}

#[test]
fn test() {
    let scores = vec![1, 3, 5, 10, 15];
    let ages = vec![1, 2, 3, 4, 5];
    let res = 34;
    assert_eq!(Solution::best_team_score(scores, ages), res);
    let scores = vec![4, 5, 6, 5];
    let ages = vec![2, 1, 2, 1];
    let res = 16;
    assert_eq!(Solution::best_team_score(scores, ages), res);
    let scores = vec![1, 2, 3, 5];
    let ages = vec![8, 9, 10, 1];
    let res = 6;
    assert_eq!(Solution::best_team_score(scores, ages), res);
}

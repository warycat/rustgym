struct Solution;

impl Solution {
    fn min_cost(s: String, cost: Vec<i32>) -> i32 {
        let n = s.len();
        let s: Vec<u8> = s.bytes().collect();
        let sum: i32 = cost.iter().sum();
        let mut dp = vec![0; 26];
        let mut max = 0;
        for i in 0..n {
            let b = (s[i] - b'a') as usize;
            dp[b] = dp[b].max((0..26).filter(|&j| j != b).map(|j| dp[j]).max().unwrap() + cost[i]);
            max = max.max(dp[b]);
        }
        sum - max
    }
}

#[test]
fn test() {
    let s = "abaac".to_string();
    let cost = vec![1, 2, 3, 4, 5];
    let res = 3;
    assert_eq!(Solution::min_cost(s, cost), res);
    let s = "baab".to_string();
    let cost = vec![8, 7, 2, 10];
    let res = 2;
    assert_eq!(Solution::min_cost(s, cost), res);
}

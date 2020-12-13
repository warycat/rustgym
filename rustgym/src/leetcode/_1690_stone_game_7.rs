struct Solution;

impl Solution {
    fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut memo: Vec<Vec<i32>> = vec![vec![0; n]; n];
        let sum = stones.iter().sum();
        Self::dp(0, n - 1, &mut memo, &stones, sum)
    }

    fn dp(l: usize, r: usize, memo: &mut Vec<Vec<i32>>, stones: &[i32], sum: i32) -> i32 {
        if l == r {
            0
        } else {
            if memo[l][r] != 0 {
                memo[l][r]
            } else {
                let mut res = 0;
                res = res.max(sum - stones[l] - Self::dp(l + 1, r, memo, stones, sum - stones[l]));
                res = res.max(sum - stones[r] - Self::dp(l, r - 1, memo, stones, sum - stones[r]));
                memo[l][r] = res;
                res
            }
        }
    }
}

#[test]
fn test() {
    let stones = vec![5, 3, 1, 4, 2];
    let res = 6;
    assert_eq!(Solution::stone_game_vii(stones), res);
    let stones = vec![7, 90, 5, 1, 100, 10, 10, 2];
    let res = 122;
    assert_eq!(Solution::stone_game_vii(stones), res);
}

struct Solution;

impl Solution {
    fn minimum_distance(word: String) -> i32 {
        let n = word.len();
        let s: Vec<i32> = word.bytes().map(|b| (b - b'A') as i32).collect();
        let mut memo: Vec<Vec<Vec<i32>>> = vec![vec![vec![-1; 27]; 27]; n];
        Self::dp(0, 26, 26, &mut memo, &s, n)
    }

    fn dp(
        start: usize,
        f1: i32,
        f2: i32,
        memo: &mut Vec<Vec<Vec<i32>>>,
        s: &[i32],
        n: usize,
    ) -> i32 {
        if start == n {
            0
        } else {
            if memo[start][f1 as usize][f2 as usize] != -1 {
                return memo[start][f1 as usize][f2 as usize];
            }
            let mut res = std::i32::MAX;
            let g = s[start];
            res = res.min(Self::dp(start + 1, g, f2, memo, s, n) + Self::dist(f1, g));
            res = res.min(Self::dp(start + 1, f1, g, memo, s, n) + Self::dist(f2, g));
            memo[start][f1 as usize][f2 as usize] = res;
            res
        }
    }

    fn dist(f: i32, g: i32) -> i32 {
        if f == 26 {
            0
        } else {
            (f / 6 - g / 6).abs() + (f % 6 - g % 6).abs()
        }
    }
}

#[test]
fn test() {
    let word = "CAKE".to_string();
    let res = 3;
    assert_eq!(Solution::minimum_distance(word), res);
    let word = "HAPPY".to_string();
    let res = 6;
    assert_eq!(Solution::minimum_distance(word), res);
    let word = "NEW".to_string();
    let res = 3;
    assert_eq!(Solution::minimum_distance(word), res);
    let word = "YEAR".to_string();
    let res = 7;
    assert_eq!(Solution::minimum_distance(word), res);
}

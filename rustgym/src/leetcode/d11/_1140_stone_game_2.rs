struct Solution;
use std::collections::HashMap;

impl Solution {
    fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut memo: HashMap<(usize, usize), (i32, i32)> = HashMap::new();
        Self::dp(0, 1, &mut memo, &piles, n).0
    }

    fn dp(
        start: usize,
        m: usize,
        memo: &mut HashMap<(usize, usize), (i32, i32)>,
        piles: &[i32],
        n: usize,
    ) -> (i32, i32) {
        if start == n {
            (0, 0)
        } else {
            if let Some(&res) = memo.get(&(start, m)) {
                return res;
            }
            let mut a = 0;
            let mut res = (0, 0);
            for i in start..(start + 2 * m).min(n) {
                a += piles[i];
                let x = i - start + 1;
                let (b, c) = Self::dp(i + 1, x.max(m), memo, piles, n);
                if a + c > res.0 {
                    res = (a + c, b);
                }
            }
            memo.insert((start, m), res);
            res
        }
    }
}

#[test]
fn test() {
    let piles = vec![2, 7, 9, 4, 4];
    let res = 10;
    assert_eq!(Solution::stone_game_ii(piles), res);
}

struct Solution;

use std::collections::HashMap;

impl Solution {
    fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
        let n = cost.len();
        let m = cost[0].len();
        let mask2: u32 = (1 << m) - 1;
        let mut memo: HashMap<(usize, u32), i32> = HashMap::new();
        Self::dp(n, mask2, &mut memo, &cost, n, m)
    }

    fn dp(
        n1: usize,
        mask2: u32,
        memo: &mut HashMap<(usize, u32), i32>,
        cost: &[Vec<i32>],
        n: usize,
        m: usize,
    ) -> i32 {
        if let Some(&res) = memo.get(&(n1, mask2)) {
            return res;
        }
        let res = if n1 == 0 {
            (0..m)
                .map(|j| {
                    if (mask2 & 1 << j) != 0 {
                        (0..n).map(|i| cost[i][j]).min().unwrap()
                    } else {
                        0
                    }
                })
                .sum::<i32>()
        } else {
            let mut res = std::i32::MAX;
            for j in 0..m {
                let next2 = mask2 & !(1 << j);
                res = res.min(cost[n1 - 1][j] + Self::dp(n1 - 1, next2, memo, cost, n, m));
            }
            res
        };
        memo.insert((n1, mask2), res);
        res
    }
}

#[test]
fn test() {
    let cost = vec_vec_i32![[15, 96], [36, 2]];
    let res = 17;
    assert_eq!(Solution::connect_two_groups(cost), res);
    let cost = vec_vec_i32![[1, 3, 5], [4, 1, 1], [1, 5, 3]];
    let res = 4;
    assert_eq!(Solution::connect_two_groups(cost), res);
    let cost = vec_vec_i32![[2, 5, 1], [3, 4, 7], [8, 1, 2], [6, 2, 4], [3, 8, 8]];
    let res = 10;
    assert_eq!(Solution::connect_two_groups(cost), res);
}

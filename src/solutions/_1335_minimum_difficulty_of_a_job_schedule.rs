struct Solution;

use std::collections::HashMap;

impl Solution {
    fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        let d = d as usize;
        if d > n {
            return -1;
        }
        let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
        Self::dp(0, d, &mut memo, &job_difficulty, n)
    }

    fn dp(
        start: usize,
        d: usize,
        memo: &mut HashMap<(usize, usize), i32>,
        jobs: &[i32],
        n: usize,
    ) -> i32 {
        if let Some(&res) = memo.get(&(start, d)) {
            return res;
        }
        let res = if d == 1 {
            *jobs[start..n].iter().max().unwrap()
        } else {
            if start + d == n {
                jobs[start..start + d].iter().sum()
            } else {
                let mut min = std::i32::MAX;
                let mut max = 0;
                for i in start..=(n - d) {
                    max = max.max(jobs[i]);
                    min = min.min(max + Self::dp(i + 1, d - 1, memo, jobs, n));
                }
                min
            }
        };
        memo.insert((start, d), res);
        res
    }
}

#[test]
fn test() {
    let job_difficulty = vec![6, 5, 4, 3, 2, 1];
    let d = 2;
    let res = 7;
    assert_eq!(Solution::min_difficulty(job_difficulty, d), res);
    let job_difficulty = vec![9, 9, 9];
    let d = 4;
    let res = -1;
    assert_eq!(Solution::min_difficulty(job_difficulty, d), res);
    let job_difficulty = vec![1, 1, 1];
    let d = 3;
    let res = 3;
    assert_eq!(Solution::min_difficulty(job_difficulty, d), res);
    let job_difficulty = vec![7, 1, 7, 1, 7, 1];
    let d = 3;
    let res = 15;
    assert_eq!(Solution::min_difficulty(job_difficulty, d), res);
    let job_difficulty = vec![11, 111, 22, 222, 33, 333, 44, 444];
    let d = 6;
    let res = 843;
    assert_eq!(Solution::min_difficulty(job_difficulty, d), res);
}

struct Solution;

impl Solution {
    fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let n = candidates.len();
        candidates.sort_unstable();
        let mut combination: Vec<i32> = vec![];
        let mut res: Vec<Vec<i32>> = vec![];
        Self::backtrack(0, target, &mut combination, &mut res, &candidates, n);
        res
    }
    fn backtrack(
        start: usize,
        target: i32,
        combination: &mut Vec<i32>,
        all: &mut Vec<Vec<i32>>,
        candidates: &[i32],
        n: usize,
    ) {
        if target == 0 {
            all.push(combination.to_vec());
        } else {
            for i in start..n {
                if candidates[i] > target {
                    break;
                } else {
                    combination.push(candidates[i]);
                    Self::backtrack(i, target - candidates[i], combination, all, candidates, n);
                    combination.pop();
                }
            }
        }
    }
}

#[test]
fn test() {
    let candidates = vec![2, 3, 6, 7];
    let target = 7;
    let res = vec![vec![2, 2, 3], vec![7]];
    assert_eq!(Solution::combination_sum(candidates, target), res);
    let candidates = vec![2, 3, 5];
    let target = 8;
    let res = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
    assert_eq!(Solution::combination_sum(candidates, target), res);
}

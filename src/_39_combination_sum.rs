struct Solution;

impl Solution {
    fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut combination: Vec<i32> = vec![];
        let mut res: Vec<Vec<i32>> = vec![];
        Solution::backtrack(&mut res, &candidates, &mut combination, target, 0);
        res
    }
    fn backtrack(
        res: &mut Vec<Vec<i32>>,
        candidates: &[i32],
        combination: &mut Vec<i32>,
        target: i32,
        begin: usize,
    ) {
        if target == 0 {
            res.push(combination.to_vec());
        } else {
            for i in begin..candidates.len() {
                if candidates[i] > target {
                    break;
                } else {
                    combination.push(candidates[i]);
                    Solution::backtrack(res, candidates, combination, target - candidates[i], i);
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

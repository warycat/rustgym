struct Solution;
use std::collections::HashMap;

impl Solution {
    fn path_sum(nums: Vec<i32>) -> i32 {
        let mut tree: HashMap<(usize, usize), i32> = HashMap::new();
        for x in nums {
            let v = x % 10;
            let p = ((x / 10) % 10 - 1) as usize;
            let d = ((x / 100) - 1) as usize;
            *tree.entry((d, p)).or_default() = v;
        }
        let mut res = 0;
        let mut path: Vec<i32> = vec![];
        Self::dfs((0, 0), &mut path, &mut res, &tree);
        res
    }

    fn dfs(
        start: (usize, usize),
        path: &mut Vec<i32>,
        sum: &mut i32,
        tree: &HashMap<(usize, usize), i32>,
    ) {
        let val = tree[&start];
        path.push(val);
        let left = (start.0 + 1, start.1 * 2);
        let right = (start.0 + 1, start.1 * 2 + 1);
        if !tree.contains_key(&left) && !tree.contains_key(&right) {
            *sum += path.iter().copied().sum::<i32>();
        }
        if tree.contains_key(&left) {
            Self::dfs(left, path, sum, tree);
        }
        if tree.contains_key(&right) {
            Self::dfs(right, path, sum, tree);
        }
        path.pop();
    }
}

#[test]
fn test() {
    let nums = vec![113, 215, 221];
    let res = 12;
    assert_eq!(Solution::path_sum(nums), res);
    let nums = vec![113, 221];
    let res = 4;
    assert_eq!(Solution::path_sum(nums), res);
}

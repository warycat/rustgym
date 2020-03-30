struct Solution;
use std::collections::HashSet;

impl Solution {
    fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        let mut hs: HashSet<Vec<i32>> = HashSet::new();
        Self::dfs(0, &mut nums, &mut hs, n);
        let mut res: Vec<Vec<i32>> = hs.drain().collect();
        res.sort_unstable();
        res
    }

    fn dfs(start: usize, nums: &mut Vec<i32>, all: &mut HashSet<Vec<i32>>, n: usize) {
        if start == n {
            all.insert(nums.to_vec());
        } else {
            for i in start..n {
                nums.swap(start, i);
                Self::dfs(start + 1, nums, all, n);
                nums.swap(start, i);
            }
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 2];
    let res = vec_vec_i32![[1, 1, 2], [1, 2, 1], [2, 1, 1]];
    assert_eq!(Solution::permute_unique(nums), res);
    let nums = vec![2, 2, 1, 1];
    let res = vec_vec_i32![
        [1, 1, 2, 2],
        [1, 2, 1, 2],
        [1, 2, 2, 1],
        [2, 1, 1, 2],
        [2, 1, 2, 1],
        [2, 2, 1, 1]
    ];
    assert_eq!(Solution::permute_unique(nums), res);
}

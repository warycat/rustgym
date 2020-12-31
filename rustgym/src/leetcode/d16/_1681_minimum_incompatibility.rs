struct Solution;

use std::collections::HashSet;

impl Solution {
    fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = nums.len();
        let mut sets: Vec<(u32, i32)> = vec![];
        let mut visited: HashSet<i32> = HashSet::new();
        Self::dfs(0, n / k, 0, &mut visited, &mut sets, &nums, n);
        let mut memo: Vec<Option<Option<i32>>> = vec![None; 1 << n];
        if let Some(sum) = Self::dp((1 << n) - 1, &mut memo, &sets, &nums, n, k) {
            sum
        } else {
            -1
        }
    }

    fn dp(
        cur: u32,
        memo: &mut Vec<Option<Option<i32>>>,
        sets: &[(u32, i32)],
        nums: &[i32],
        n: usize,
        k: usize,
    ) -> Option<i32> {
        if cur == 0 {
            Some(0)
        } else {
            if let Some(res) = memo[cur as usize] {
                res
            } else {
                let mut min_sum = std::i32::MAX;
                for (set, incompatibility) in sets {
                    if (set & cur).count_ones() as usize == n / k {
                        if let Some(sum) = Self::dp(cur & !set, memo, sets, nums, n, k) {
                            min_sum = min_sum.min(incompatibility + sum);
                        }
                    }
                }
                let res = if min_sum == std::i32::MAX {
                    None
                } else {
                    Some(min_sum)
                };
                memo[cur as usize] = Some(res);
                res
            }
        }
    }

    fn dfs(
        start: usize,
        size: usize,
        cur: u32,
        visited: &mut HashSet<i32>,
        all: &mut Vec<(u32, i32)>,
        nums: &[i32],
        n: usize,
    ) {
        if size == 0 {
            all.push((
                cur,
                visited.iter().max().unwrap() - visited.iter().min().unwrap(),
            ));
        } else {
            for i in start..n {
                if visited.insert(nums[i]) {
                    Self::dfs(i + 1, size - 1, cur | 1 << i, visited, all, nums, n);
                    visited.remove(&nums[i]);
                }
            }
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 1, 4];
    let k = 2;
    let res = 4;
    assert_eq!(Solution::minimum_incompatibility(nums, k), res);
    let nums = vec![6, 3, 8, 1, 3, 1, 2, 2];
    let k = 4;
    let res = 6;
    assert_eq!(Solution::minimum_incompatibility(nums, k), res);
    let nums = vec![5, 3, 3, 6, 3, 3];
    let k = 3;
    let res = -1;
    assert_eq!(Solution::minimum_incompatibility(nums, k), res);
    let nums = vec![14, 4, 6, 6, 4, 14, 13, 12, 3, 1, 7, 14, 3, 10, 5];
    let k = 1;
    let res = -1;
    assert_eq!(Solution::minimum_incompatibility(nums, k), res);
}

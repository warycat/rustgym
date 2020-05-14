struct Solution;

impl Solution {
    fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut res: Vec<Vec<i32>> = vec![];
        let mut used: Vec<bool> = vec![false; n];
        let mut cur: Vec<i32> = vec![];
        nums.sort_unstable();
        Self::dfs(&mut cur, &mut used, &mut res, &nums, n);
        res
    }

    fn dfs(
        cur: &mut Vec<i32>,
        used: &mut Vec<bool>,
        all: &mut Vec<Vec<i32>>,
        nums: &[i32],
        n: usize,
    ) {
        if cur.len() == n {
            all.push(cur.to_vec());
        } else {
            for i in 0..n {
                if used[i] {
                    continue;
                }
                if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                    continue;
                }
                used[i] = true;
                cur.push(nums[i]);
                Self::dfs(cur, used, all, nums, n);
                used[i] = false;
                cur.pop();
            }
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 2];
    let mut res = vec_vec_i32![[1, 1, 2], [1, 2, 1], [2, 1, 1]];
    let mut ans = Solution::permute_unique(nums);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let nums = vec![2, 2, 1, 1];
    let mut res = vec_vec_i32![
        [1, 1, 2, 2],
        [1, 2, 1, 2],
        [1, 2, 2, 1],
        [2, 1, 1, 2],
        [2, 1, 2, 1],
        [2, 2, 1, 1]
    ];
    let mut ans = Solution::permute_unique(nums);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}

struct Solution;

impl Solution {
    fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        if k > 9 {
            return vec![];
        }
        let nums: Vec<i32> = (1..10).collect();
        let mut cur = vec![];
        let mut res = vec![];
        Self::dfs(0, n, &mut cur, &mut res, &nums, k as usize);
        res
    }
    fn dfs(
        start: usize,
        target: i32,
        cur: &mut Vec<i32>,
        all: &mut Vec<Vec<i32>>,
        nums: &[i32],
        n: usize,
    ) {
        if cur.len() == n {
            if target == 0 {
                all.push(cur.to_vec());
            }
        } else {
            if target > 0 && start < nums.len() {
                Self::dfs(start + 1, target, cur, all, nums, n);
                cur.push(nums[start]);
                Self::dfs(start + 1, target - nums[start], cur, all, nums, n);
                cur.pop();
            }
        }
    }
}

#[test]
fn test() {
    let k = 3;
    let n = 7;
    let res = vec_vec_i32![[1, 2, 4]];
    assert_eq!(Solution::combination_sum3(k, n), res);
    let k = 3;
    let n = 9;
    let mut res = vec_vec_i32![[1, 2, 6], [1, 3, 5], [2, 3, 4]];
    let mut ans = Solution::combination_sum3(k, n);
    res.sort_unstable();
    ans.sort_unstable();
    assert_eq!(ans, res);
}

struct Solution;

impl Solution {
    fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let n = nums.len();
        let mut cur: Vec<i32> = vec![];
        Self::dfs(0, &mut cur, &mut res, &nums, n);
        res
    }

    fn dfs(start: usize, cur: &mut Vec<i32>, all: &mut Vec<Vec<i32>>, nums: &[i32], n: usize) {
        if start == n {
            if cur.len() > 1 {
                all.push(cur.to_vec());
            }
        } else {
            if cur.is_empty() || nums[start] >= *cur.last().unwrap() {
                cur.push(nums[start]);
                Self::dfs(start + 1, cur, all, nums, n);
                cur.pop();
            }
            if cur.is_empty() || nums[start] != *cur.last().unwrap() {
                Self::dfs(start + 1, cur, all, nums, n);
            }
        }
    }
}

#[test]
fn test() {
    let nums = vec![4, 6, 7, 7];
    let mut res = vec_vec_i32![
        [4, 6],
        [4, 7],
        [4, 6, 7],
        [4, 6, 7, 7],
        [6, 7],
        [6, 7, 7],
        [7, 7],
        [4, 7, 7]
    ];
    let mut ans = Solution::find_subsequences(nums);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}

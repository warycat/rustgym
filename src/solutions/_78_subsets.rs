struct Solution;

impl Solution {
    fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut v: Vec<i32> = vec![];
        let n = nums.len();
        Self::dfs(0, &mut res, &nums, &mut v, n);
        res
    }

    fn dfs(i: usize, all: &mut Vec<Vec<i32>>, nums: &[i32], cur: &mut Vec<i32>, n: usize) {
        if i == n {
            all.push(cur.to_vec());
        } else {
            Self::dfs(i + 1, all, nums, cur, n);
            cur.push(nums[i]);
            Self::dfs(i + 1, all, nums, cur, n);
            cur.pop();
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    let res = vec![
        vec![],
        vec![3],
        vec![2],
        vec![2, 3],
        vec![1],
        vec![1, 3],
        vec![1, 2],
        vec![1, 2, 3],
    ];
    assert_eq!(Solution::subsets(nums), res);
}

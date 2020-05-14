struct Solution;

impl Solution {
    fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let nums: Vec<i32> = (1..=n).collect();
        let n = n as usize;
        let k = k as usize;
        let mut cur: Vec<i32> = vec![];
        Self::dfs(0, 0, &mut cur, &mut res, &nums, n, k);
        res
    }

    fn dfs(
        start: usize,
        count: usize,
        cur: &mut Vec<i32>,
        all: &mut Vec<Vec<i32>>,
        nums: &[i32],
        n: usize,
        k: usize,
    ) {
        if count == k {
            all.push(cur.to_vec());
        } else {
            for i in start..n {
                cur.push(nums[i]);
                Self::dfs(i + 1, count + 1, cur, all, nums, n, k);
                cur.pop();
            }
        }
    }
}

#[test]
fn test() {
    let n = 4;
    let k = 2;
    let mut res = vec_vec_i32![[2, 4], [3, 4], [2, 3], [1, 2], [1, 3], [1, 4]];
    let mut ans = Solution::combine(n, k);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}

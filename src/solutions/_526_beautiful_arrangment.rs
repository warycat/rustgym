struct Solution;

impl Solution {
    fn count_arrangement(n: i32) -> i32 {
        let mut res = 0;
        let mut nums = (1..=n).collect();
        let n = n as usize;
        Self::dfs(0, &mut nums, &mut res, n);
        res
    }

    fn dfs(start: usize, nums: &mut Vec<i32>, all: &mut i32, n: usize) {
        if start == n {
            *all += 1;
        } else {
            for i in start..n {
                if nums[i] % (start as i32 + 1) == 0 || (start as i32 + 1) % nums[i] == 0 {
                    nums.swap(start, i);
                    Self::dfs(start + 1, nums, all, n);
                    nums.swap(start, i);
                }
            }
        }
    }
}

#[test]
fn test() {
    let n = 2;
    let res = 2;
    assert_eq!(Solution::count_arrangement(n), res);
}

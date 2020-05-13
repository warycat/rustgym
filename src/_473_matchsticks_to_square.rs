struct Solution;

impl Solution {
    fn makesquare(mut nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n == 0 {
            return false;
        }
        let sum: i32 = nums.iter().sum();
        if sum % 4 != 0 {
            return false;
        }
        nums.sort_unstable_by_key(|&x| -x);
        let mut sides = vec![0; 4];
        Self::dfs(0, &mut sides, &nums, sum / 4, n)
    }

    fn dfs(start: usize, sides: &mut Vec<i32>, nums: &[i32], sum: i32, n: usize) -> bool {
        if start == n {
            true
        } else {
            for i in 0..4 {
                if sides[i] + nums[start] > sum {
                    continue;
                }
                sides[i] += nums[start];
                if Self::dfs(start + 1, sides, nums, sum, n) {
                    return true;
                }
                sides[i] -= nums[start];
            }
            false
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 2, 2, 2];
    let res = true;
    assert_eq!(Solution::makesquare(nums), res);
    let nums = vec![1, 1, 2, 2, 2];
    let res = true;
    assert_eq!(Solution::makesquare(nums), res);
    let nums = vec![5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3];
    let res = true;
    assert_eq!(Solution::makesquare(nums), res);
}

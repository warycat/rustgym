struct Solution;

impl Solution {
    fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left = vec![0; n];
        let mut dp: Vec<i32> = vec![];
        for i in 0..n {
            let x = nums[i];
            let size = match dp.binary_search(&x) {
                Ok(j) => j + 1,
                Err(j) => {
                    if j == dp.len() {
                        dp.push(x);
                    } else {
                        dp[j] = x;
                    }
                    j + 1
                }
            };
            left[i] = size;
        }

        let mut right = vec![0; n];
        let mut dp: Vec<i32> = vec![];
        for i in (0..n).rev() {
            let x = nums[i];
            let size = match dp.binary_search(&x) {
                Ok(j) => j + 1,
                Err(j) => {
                    if j == dp.len() {
                        dp.push(x);
                    } else {
                        dp[j] = x;
                    }
                    j + 1
                }
            };
            right[i] = size;
        }

        let mut res = n;
        for i in 0..n {
            if left[i] != 1 && right[i] != 1 {
                res = res.min(n - (left[i] + right[i] - 1));
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 3, 1];
    let res = 0;
    assert_eq!(Solution::minimum_mountain_removals(nums), res);
    let nums = vec![2, 1, 1, 5, 6, 2, 3, 1];
    let res = 3;
    assert_eq!(Solution::minimum_mountain_removals(nums), res);
    let nums = vec![4, 3, 2, 1, 1, 2, 3, 1];
    let res = 4;
    assert_eq!(Solution::minimum_mountain_removals(nums), res);
    let nums = vec![1, 2, 3, 4, 4, 3, 2, 1];
    let res = 1;
    assert_eq!(Solution::minimum_mountain_removals(nums), res);
}

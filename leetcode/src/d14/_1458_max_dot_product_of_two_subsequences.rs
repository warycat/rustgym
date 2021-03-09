struct Solution;

impl Solution {
    fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();
        let mut memo = vec![vec![std::i32::MIN; m + 1]; n + 1];
        Self::dp(n, m, &mut memo, &nums1, &nums2)
    }
    fn dp(n: usize, m: usize, memo: &mut Vec<Vec<i32>>, nums1: &[i32], nums2: &[i32]) -> i32 {
        if memo[n][m] > std::i32::MIN {
            memo[n][m]
        } else {
            let mut res = nums1[n - 1] * nums2[m - 1];
            if n > 1 && m > 1 {
                res = res.max(Self::dp(n - 1, m - 1, memo, nums1, nums2).max(0) + res);
            }
            if n > 1 {
                res = res.max(Self::dp(n - 1, m, memo, nums1, nums2));
            }
            if m > 1 {
                res = res.max(Self::dp(n, m - 1, memo, nums1, nums2));
            }
            memo[n][m] = res;
            res
        }
    }
}

#[test]
fn test() {
    let nums1 = vec![2, 1, -2, 5];
    let nums2 = vec![3, 0, -6];
    let res = 18;
    assert_eq!(Solution::max_dot_product(nums1, nums2), res);
    let nums1 = vec![3, -2];
    let nums2 = vec![2, -6, 7];
    let res = 21;
    assert_eq!(Solution::max_dot_product(nums1, nums2), res);
    let nums1 = vec![-1, -1];
    let nums2 = vec![1, 1];
    let res = -1;
    assert_eq!(Solution::max_dot_product(nums1, nums2), res);
}

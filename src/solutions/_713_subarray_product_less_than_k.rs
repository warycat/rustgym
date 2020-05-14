struct Solution;

impl Solution {
    fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut i = 0;
        let mut product = 1;
        let mut res = 0;
        for j in 0..n {
            product *= nums[j];
            while i <= j && product >= k {
                product /= nums[i];
                i += 1;
            }
            res += (j + 1 - i) as i32;
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![10, 5, 2, 6];
    let k = 100;
    let res = 8;
    assert_eq!(Solution::num_subarray_product_less_than_k(nums, k), res);
    let nums = vec![1, 2, 3];
    let k = 0;
    let res = 0;
    assert_eq!(Solution::num_subarray_product_less_than_k(nums, k), res);
    let nums = vec![1, 1, 1];
    let k = 1;
    let res = 0;
    assert_eq!(Solution::num_subarray_product_less_than_k(nums, k), res);
}

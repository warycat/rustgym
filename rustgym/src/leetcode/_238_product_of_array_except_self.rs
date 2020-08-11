struct Solution;

impl Solution {
    fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res: Vec<i32> = vec![1; n];
        let mut product = 1;
        for i in 0..n {
            res[i] *= product;
            product *= nums[i];
        }
        product = 1;
        for i in (0..n).rev() {
            res[i] *= product;
            product *= nums[i];
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4];
    let res = vec![24, 12, 8, 6];
    assert_eq!(Solution::product_except_self(nums), res);
}

struct Solution;

impl Solution {
    fn min_operations(nums: Vec<i32>) -> i32 {
        let mut ones = 0;
        let mut max_width = 0;
        for mut x in nums {
            ones += x.count_ones();
            let mut width = 0;
            while x > 0 {
                x >>= 1;
                width += 1;
            }
            max_width = max_width.max(width);
        }
        ((ones + max_width) as i32 - 1).max(0)
    }
}

#[test]
fn test() {
    let nums = vec![1, 5];
    let res = 5;
    assert_eq!(Solution::min_operations(nums), res);
    let nums = vec![2, 2];
    let res = 3;
    assert_eq!(Solution::min_operations(nums), res);
    let nums = vec![4, 2, 5];
    let res = 6;
    assert_eq!(Solution::min_operations(nums), res);
    let nums = vec![3, 2, 2, 4];
    let res = 7;
    assert_eq!(Solution::min_operations(nums), res);
    let nums = vec![2, 4, 8, 16];
    let res = 8;
    assert_eq!(Solution::min_operations(nums), res);
    let nums = vec![0];
    let res = 0;
    assert_eq!(Solution::min_operations(nums), res);
}

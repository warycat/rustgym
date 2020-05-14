struct Solution;

impl Solution {
    fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        let mut sums = vec![0, 0];
        let n = nums.len();
        for i in 0..n {
            let mut adj = vec![];
            if i > 0 {
                adj.push(nums[i - 1]);
            }
            if i + 1 < n {
                adj.push(nums[i + 1]);
            }
            let min = adj.into_iter().min().unwrap();
            if nums[i] >= min {
                sums[i % 2] += (nums[i] - min) + 1;
            }
        }
        sums[0].min(sums[1])
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    let res = 2;
    assert_eq!(Solution::moves_to_make_zigzag(nums), res);
    let nums = vec![9, 6, 1, 6, 2];
    let res = 4;
    assert_eq!(Solution::moves_to_make_zigzag(nums), res);
    let nums = vec![10, 4, 4, 10, 10, 6, 2, 3];
    let res = 13;
    assert_eq!(Solution::moves_to_make_zigzag(nums), res);
}

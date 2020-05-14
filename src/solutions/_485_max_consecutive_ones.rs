struct Solution;

impl Solution {
    fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut count = 0;
        for x in nums {
            if x == 1 {
                count += 1;
                max = i32::max(count, max);
            } else {
                count = 0;
            }
        }
        max
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 0, 1, 1, 1];
    assert_eq!(Solution::find_max_consecutive_ones(nums), 3);
}

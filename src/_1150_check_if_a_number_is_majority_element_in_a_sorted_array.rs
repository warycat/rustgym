struct Solution;

impl Solution {
    fn is_majority_element(nums: Vec<i32>, target: i32) -> bool {
        let mut first: Option<usize> = None;
        let mut last: Option<usize> = None;
        let n = nums.len();
        for i in 0..n {
            if nums[i] == target {
                if first.is_none() {
                    first = Some(i);
                }
                last = Some(i);
            }
        }
        if first.is_none() {
            false
        } else {
            last.unwrap() - first.unwrap() + 1 > n / 2
        }
    }
}

#[test]
fn test() {
    let nums = vec![2, 4, 5, 5, 5, 5, 5, 6, 6];
    let target = 5;
    assert_eq!(Solution::is_majority_element(nums, target), true);
}

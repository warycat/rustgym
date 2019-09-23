struct Solution;

impl Solution {
    fn check_possibility(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 2 {
            return true;
        }
        let mut p: Option<usize> = None;
        for i in 0..n - 1 {
            if nums[i] > nums[i + 1] {
                if p.is_some() {
                    return false;
                } else {
                    p = Some(i);
                }
            }
        }
        if let Some(p) = p {
            if p != 0 && p != n - 2 {
                if nums[p - 1] >= nums[p + 1] && nums[p] >= nums[p + 2] {
                    return false;
                }
            }
        }

        true
    }
}

#[test]
fn test() {
    assert_eq!(Solution::check_possibility(vec![4, 2, 3]), true);
    assert_eq!(Solution::check_possibility(vec![4, 2, 1]), false);
}
